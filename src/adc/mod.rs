mod types;

// Re-export everything in types.rs
pub use types::*;

use crate::{
    charger::DischargeCurrentLimit, common::Task, Ibatmeasenable, NtcThermistorType, Ntcautotim,
    Tempautotim, Vbatautoenable, Vbatburstenable,
};
use libm::logf;

/// Convert an ADC voltage measurement to a voltage in volts
///
/// # Arguments
///
/// * `v_adc` - The ADC voltage measurement in u16
/// * `v_full_scale` - The full scale voltage for the measurement in volts
fn convert_vadc_to_voltage(v_adc: u16, v_full_scale: f32) -> f32 {
    // Convert result to f32
    // 1023.0 is the maximum value for the 10 bit ADC.
    (v_adc as f32 / 1023.0) * v_full_scale
}

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Measure VBAT
    ///
    /// This function triggers a VBAT measurement and returns the result.
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured VBAT voltage
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBAT measurement result
    pub async fn measure_vbat(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        // Disable VBAT burst measurement
        self.configure_vbat_burst_measurement(false).await?;

        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering VBAT measurement...");
        self.device
            .adc()
            .taskvbatmeasure()
            .dispatch_async(|command| command.set_taskvbatmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        let result = self.get_vbat_measurement_result().await?;

        Ok(result)
    }

    /// Measure VBAT burst (VBAT0, VBAT1, VBAT2, VBAT3)
    ///
    /// This function triggers a VBAT measurement in Burst mode and returns the results.
    /// A VBAT measurement triggered in burst mode performs four consecutive measurements, with each result available separately.
    ///
    /// # Returns
    ///
    /// * `Ok((f32, f32, f32, f32))` - The measured VBAT voltages (VBAT0, VBAT1, VBAT2, VBAT3)
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBAT measurement result
    pub async fn measure_vbat_burst(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<(f32, f32, f32, f32), crate::NPM1300Error<I2c::Error>> {
        // Enable VBAT burst measurement
        self.configure_vbat_burst_measurement(true).await?;

        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering VBAT measurement...");
        self.device
            .adc()
            .taskvbatmeasure()
            .dispatch_async(|command| command.set_taskvbatmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        // Conversions are run back-to-back and complete in tCONV.
        self.delay.delay_us(250).await;

        let vbat0 = self.get_vbat_burst_measurement_result(0).await?;
        let vbat1 = self.get_vbat_burst_measurement_result(1).await?;
        let vbat2 = self.get_vbat_burst_measurement_result(2).await?;
        let vbat3 = self.get_vbat_burst_measurement_result(3).await?;

        Ok((vbat0, vbat1, vbat2, vbat3))
    }

    /// Get the latest VBAT measurement result without triggering a new measurement
    ///
    /// This function retrieves the most recent VBAT measurement result from the ADC registers.
    /// It is primarily intended for use with automatic measurements where the ADC is already
    /// configured to take periodic VBAT readings.
    /// Delays must be handled by the caller.
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The most recent VBAT measurement in volts
    /// * `Err(NPM1300Error)` - An error occurred while reading the ADC registers
    pub async fn get_vbat_measurement_result(
        &mut self,
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        let msb = self
            .device
            .adc()
            .adcvbatresultmsb()
            .read_async()
            .await?
            .vbatresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .vbatresultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // 5.0 is VFSVBAT, the full scale voltage for measuring VBAT.
        let result = convert_vadc_to_voltage(result, 5.0);

        Ok(result)
    }

    /// Get the VBAT measurement result for a specific VBAT index
    ///
    /// This function retrieves the VBAT measurement result for a specific VBAT index from the ADC registers.
    /// It is primarily intended for use with VBAT burst measurements where the ADC is already
    /// configured to take periodic VBAT readings.
    /// Delays must be handled by the caller.
    ///
    /// # Arguments
    ///
    /// * `vbat_index` - The index of the VBAT measurement result to retrieve (0-3)
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The VBAT measurement result in volts
    /// * `Err(NPM1300Error)` - An error occurred while reading the ADC registers
    pub async fn get_vbat_burst_measurement_result(
        &mut self,
        vbat_index: u8,
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        let vbat_index = vbat_index as usize;
        let msb = self
            .device
            .adc()
            .adcvbatburstresultmsb(vbat_index)
            .read_async()
            .await?
            .vbatresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_1_resultlsbs()
            .read_async()
            .await?
            .vbat_3_resultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // 5.0 is VFSVBAT, the full scale voltage for measuring VBAT.
        let result = convert_vadc_to_voltage(result, 5.0);

        Ok(result)
    }

    /// Measure NTC
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured NTC resistance in degrees Celsius
    /// * `Err(NPM1300Error)` - An error occurred while reading the NTC measurement result
    //TODO: test this function
    pub async fn measure_ntc(&mut self) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering NTC measurement...");
        self.device
            .adc()
            .taskntcmeasure()
            .dispatch_async(|command| command.set_taskntcmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        let result = self.get_ntc_measurement_result().await?;
        Ok(result)
    }

    /// Get the latest NTC measurement result without triggering a new measurement
    ///
    /// This function retrieves the most recent NTC measurement result from the ADC registers.
    /// It is primarily intended for use with automatic measurements where the ADC is already
    /// configured to take periodic NTC readings.
    /// Delays must be handled by the caller.
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The most recent NTC measurement in degrees Celsius
    /// * `Err(NPM1300Error)` - An error occurred while reading the ADC registers
    pub async fn get_ntc_measurement_result(
        &mut self,
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        let msb = self
            .device
            .adc()
            .adcntcresultmsb()
            .read_async()
            .await?
            .ntcresultmsb();
        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .ntcresultlsb();
        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;
        // Convert result to f32
        // The temperature is returned in degrees Celsius
        //
        let result = if let Some(ntc_beta) = self.ntc_beta {
            1.0 / ((1.0 / 298.15) - (1.0 / ntc_beta) * logf((1024.0 / result as f32) - 1.0))
                - 273.15
        } else {
            return Err(crate::NPM1300Error::InvalidNtcBeta);
        };
        Ok(result)
    }

    /// Measure die temperature
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured die temperature in degrees Celsius
    /// * `Err(NPM1300Error)` - An error occurred while reading the die temperature measurement result
    pub async fn measure_die_temperature(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering die temperature measurement...");
        self.device
            .adc()
            .tasktempmeasure()
            .dispatch_async(|command| command.set_tasktempmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        let result = self.get_die_temperature_measurement_result().await?;
        Ok(result)
    }

    /// Get the latest die temperature measurement result without triggering a new measurement
    ///
    /// This function retrieves the most recent die temperature measurement result from the ADC registers.
    /// It is primarily intended for use with automatic measurements where the ADC is already
    /// configured to take periodic die temperature readings.
    /// Delays must be handled by the caller.
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The most recent die temperature measurement in degrees Celsius
    /// * `Err(NPM1300Error)` - An error occurred while reading the ADC registers
    pub async fn get_die_temperature_measurement_result(
        &mut self,
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        let msb = self
            .device
            .adc()
            .adctempresultmsb()
            .read_async()
            .await?
            .tempresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .tempresultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // The temperature is returned in degrees Celsius
        let result = 394.67 - 0.7926 * result as f32;

        Ok(result)
    }

    /// Measure VSYS
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured VSYS voltage
    /// * `Err(NPM1300Error)` - An error occurred while reading the VSYS measurement result
    pub async fn measure_vsys(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering VSYS measurement...");
        self.device
            .adc()
            .taskvsysmeasure()
            .dispatch_async(|command| command.set_taskvsysmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        // Read measurement result
        let msb = self
            .device
            .adc()
            .adcvsysresultmsb()
            .read_async()
            .await?
            .vsysresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .vsysresultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // 5.0 is VFSVSYS, the full scale voltage for measuring VSYS.
        let result = convert_vadc_to_voltage(result, 5.0);

        Ok(result)
    }

    /// Measure VBUS
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured VBUS voltage
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBUS measurement result
    pub async fn measure_vbus(&mut self) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering VBUS measurement...");
        self.device
            .adc()
            .taskvbus_7_measure()
            .dispatch_async(|command| command.set_taskvsysmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        // Read measurement result
        let msb = self
            .device
            .adc()
            .adcvbatburstresultmsb(3)
            .read_async()
            .await?
            .vbatresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_1_resultlsbs()
            .read_async()
            .await?
            .vbat_3_resultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // 5.0 is VFSVBUS, the full scale voltage for measuring VBUS.
        let result = convert_vadc_to_voltage(result, 7.5);

        Ok(result)
    }

    /// Measure delayed VBAT
    ///
    /// # Note
    ///
    /// This function currently does not work as expected. The measurement will always take > 1024ms to complete.
    ///
    /// # Arguments
    ///
    /// * `delay` - The delay in milliseconds (4-514ms in steps of 2ms)
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured delayed VBAT voltage
    /// * `Err(NPM1300Error)` - An error occurred while reading the delayed VBAT measurement result
    pub async fn measure_delayed_vbat(
        &mut self,
        delay: u16,
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        // Test if delay is valid
        if !(4..=514).contains(&delay) || delay % 2 != 0 {
            return Err(crate::NPM1300Error::InvalidVbatMeasurementDelayValue);
        }
        // Calculate register delay
        let register_delay = ((delay / 2) - 2) as u8;
        #[cfg(feature = "defmt-03")]
        defmt::trace!(
            "Setting delayed VBAT measurement delay register value to {}",
            register_delay
        );
        // Configure VBAT delay timer
        self.device
            .adc()
            .adcdeltimconf()
            .write_async(|reg| {
                reg.set_vbatdeltim(register_delay);
            })
            .await?;
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering delayed VBAT measurement...");
        self.device
            .adc()
            .taskdelayedvbatmeasure()
            .dispatch_async(|command| command.set_taskdlydvbatmeasure(Task::Trigger))
            .await?;

        // Wait for delayed measurement to start
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for delayed measurement to start...");
        // HACK: we found that no matter what delay we set, the measurement will always take > 1024
        // ms to complete. We do not know why yet.
        self.delay.delay_ms(1025).await;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for delayed measurement to complete...");
        self.delay.delay_us(250).await;

        let result = self.get_vbat_measurement_result().await?;

        Ok(result)
    }

    /// Get the delayed VBAT measurement delay configuration
    ///
    /// # Returns
    ///
    /// * `Ok(u8)` - The delayed VBAT measurement delay configuration
    /// * `Err(NPM1300Error)` - An error occurred while reading the delayed VBAT measurement delay configuration
    pub async fn get_vbat_delay_configuration(
        &mut self,
    ) -> Result<u8, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .adc()
            .adcdeltimconf()
            .read_async()
            .await?
            .vbatdeltim())
    }

    /// Configure auto VBAT measurement
    ///
    /// # Arguments
    ///
    /// * `enable` - If true, enable auto VBAT measurement every 1 second, otherwise single measurement when triggered
    pub async fn configure_auto_vbat_measurement(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .adc()
            .adcconfig()
            .modify_async(|reg| {
                reg.set_vbatautoenable(if enable {
                    Vbatautoenable::Autoenable
                } else {
                    Vbatautoenable::Noauto
                })
            })
            .await
    }

    /// Get the VBAT auto measurement configuration
    ///
    /// # Returns
    ///
    /// * `Ok(Vbatautoenable)` - The VBAT auto measurement configuration
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBAT auto measurement configuration
    pub async fn get_vbat_auto_measurement_configuration(
        &mut self,
    ) -> Result<Vbatautoenable, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .adc()
            .adcconfig()
            .read_async()
            .await?
            .vbatautoenable())
    }

    /// Configure auto VBAT burst measurement
    ///
    /// # Arguments
    ///
    /// * `enable` - If true, enable VBAT burst measurement, otherwise single measurement
    pub async fn configure_vbat_burst_measurement(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .adc()
            .adcconfig()
            .modify_async(|reg| {
                reg.set_vbatburstenable(if enable {
                    Vbatburstenable::Burstmode
                } else {
                    Vbatburstenable::Singlemode
                })
            })
            .await
    }

    /// Get the VBAT burst measurement configuration
    ///
    /// # Returns
    ///
    /// * `Ok(Vbatburstenable)` - The VBAT burst measurement configuration
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBAT burst measurement configuration
    pub async fn get_vbat_burst_measurement_configuration(
        &mut self,
    ) -> Result<Vbatburstenable, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .adc()
            .adcconfig()
            .read_async()
            .await?
            .vbatburstenable())
    }

    /// Configure the NTC thermistor resistance value
    ///
    /// # Arguments
    ///
    /// * `ntc_value` - The NTC thermistor resistance value to configure
    pub async fn configure_ntc_resistance(
        &mut self,
        ntc_resistance: NtcThermistorType,
        ntc_beta: Option<f32>,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Write NTC value to register
        self.device
            .adc()
            .adcntcrsel()
            .write_async(|reg| reg.set_adcntcrsel(ntc_resistance))
            .await?;
        if ntc_resistance != NtcThermistorType::None {
            // Add a check to ensure the NTC beta is not None
            if ntc_beta.is_none() {
                return Err(crate::NPM1300Error::InvalidNtcBeta);
            }
            self.ntc_beta = ntc_beta;
        }
        Ok(())
    }

    /// Get the NTC resistance configuration
    ///
    /// # Returns
    ///
    /// * `Ok(NtcThermistorType)` - The NTC resistance configuration
    /// * `Err(NPM1300Error)` - An error occurred while reading the NTC resistance configuration
    pub async fn get_ntc_resistance_configuration(
        &mut self,
    ) -> Result<NtcThermistorType, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .adc()
            .adcntcrsel()
            .read_async()
            .await?
            .adcntcrsel())
    }

    /// Configure the NTC auto measurement interval
    ///
    /// # Arguments
    ///
    /// * `ntc_auto_measurement_interval` - The NTC auto measurement interval to configure
    pub async fn configure_auto_ntc_measurement(
        &mut self,
        ntc_auto_measurement_interval: Ntcautotim,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Write NTC auto measurement interval to register
        self.device
            .adc()
            .adcautotimconf()
            .modify_async(|reg| reg.set_ntcautotim(ntc_auto_measurement_interval))
            .await?;

        // Update toggle register
        self.device
            .adc()
            .taskautotimupdate()
            .dispatch_async(|command| command.set_taskautotimupdate(Task::Trigger))
            .await
    }

    /// Get the NTC auto measurement configuration
    ///
    /// # Returns
    ///
    /// * `Ok(Ntcautotim)` - The NTC auto measurement configuration
    /// * `Err(NPM1300Error)` - An error occurred while reading the NTC auto measurement configuration
    pub async fn get_ntc_auto_measurement_configuration(
        &mut self,
    ) -> Result<Ntcautotim, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .adc()
            .adcautotimconf()
            .read_async()
            .await?
            .ntcautotim())
    }

    /// Configure the die temperature auto measurement interval
    ///
    /// # Arguments
    ///
    /// * `die_temperature_auto_measurement_interval` - The die temperature auto measurement interval to configure
    pub async fn configure_die_temperature_auto_measurement_interval(
        &mut self,
        die_temperature_auto_measurement_interval: Tempautotim,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .adc()
            .adcautotimconf()
            .modify_async(|reg| reg.set_tempautotim(die_temperature_auto_measurement_interval))
            .await?;

        // Update toggle register
        self.device
            .adc()
            .taskautotimupdate()
            .dispatch_async(|command| command.set_taskautotimupdate(Task::Trigger))
            .await
    }

    /// Get the die temperature auto measurement configuration
    ///
    /// # Returns
    ///
    /// * `Ok(Tempautotim)` - The die temperature auto measurement configuration
    /// * `Err(NPM1300Error)` - An error occurred while reading the die temperature auto measurement configuration
    pub async fn get_die_temperature_auto_measurement_configuration(
        &mut self,
    ) -> Result<Tempautotim, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .adc()
            .adcautotimconf()
            .read_async()
            .await?
            .tempautotim())
    }

    /// Configure auto ibat measurement enabled. It seems you dont
    /// need to trigger the measurement, it is done automatically
    /// when you measure vbat
    ///
    /// # Arguments
    ///
    /// * `enable` - If true, enable IBAT measurement
    pub async fn configure_ibat_measurement(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .adc()
            .adcibatmeasen()
            .modify_async(|reg| {
                reg.set_ibatmeasenable(if enable {
                    Ibatmeasenable::Ibaton
                } else {
                    Ibatmeasenable::Ibatoff
                })
            })
            .await
    }

    ///Get ibat status, this register is 4 bit wide,
    /// but i dont want to do breaking changes in the the
    /// device.yaml in case someone is using this functionality,
    /// but this is how it is read in the  npm1300 charger c example
    ///
    ///
    /// Ibat status register:
    ///
    /// define IBAT_STAT_DISCHARGE      0x04U
    /// define IBAT_STAT_CHARGE_TRICKLE 0x0CU
    /// define IBAT_STAT_CHARGE_COOL    0x0DU
    /// define IBAT_STAT_CHARGE_NORMAL  0x0FU
    ///
    ///
    pub async fn get_ibat_status(
        &mut self,
    ) -> Result<IbatStatuscodes, crate::NPM1300Error<I2c::Error>> {
        let msb = self
            .device
            .adc()
            .adcibatmeasstatus()
            .read_async()
            .await?
            .bchargermode();
        let lsb = self
            .device
            .adc()
            .adcibatmeasstatus()
            .read_async()
            .await?
            .bchargericharge();

        // 3. Combine them into a 4-bit value:
        let raw_status = ((msb as u8) << 2) | (lsb as u8);

        // 4. Convert the 4-bit code into our enum
        let status_enum = IbatStatuscodes::from(raw_status);

        return Ok(status_enum);
    }

    /// Measure IBAT current.
    /// This is a 10 bit value which seems
    /// to use the same register as the one used in vbat burst
    /// so it might not be compatible with the vbat burst measurement
    /// the ibat measurement is a 1023 bit value which is scaled to
    /// the current max charge and discharge current, so the scaling is
    /// variable and needs to be scaled based on what is currently
    /// happening in the charger
    ///
    /// # Returns
    /// Raw ac ibat value as uint16
    ///
    pub async fn measure_ibat_raw(&mut self) -> Result<u16, crate::NPM1300Error<I2c::Error>> {
        // Read measurement result
        let msb = self
            .device
            .adc()
            .adcvbatburstresultmsb(2)
            .read_async()
            .await?
            .vbatresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_1_resultlsbs()
            .read_async()
            .await?
            .vbat_2_resultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;
        return Ok(result);
    }
    /// Measure IBAT current and convert it to a scaled value
    /// This is more at home in the ADC function, but i couldnt
    /// figure out how to make it work get the max charge current
    /// and discharge current limit from the charger lib
    /// so i just put them in the charger lib
    ///
    /// # Returns
    ///
    /// a scaled signed integer in ma of the current charge or discharge current
    pub async fn measure_ibat(&mut self) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        let ibat_raw = self.measure_ibat_raw().await? as f32;
        let ibat_status = self.get_ibat_status().await?;
        let charger_max_current = self.get_charger_config_current().await? as f32;
        let discharge_current_limit = self.get_discharge_current_limit().await?;
        let ibat_scaled = match ibat_status {
            IbatStatuscodes::IbatStatDischarge => {
                let max_discharge_current = match discharge_current_limit {
                    DischargeCurrentLimit::Low => 200,
                    DischargeCurrentLimit::High => 1000,
                } as f32;
                (ibat_raw * -max_discharge_current) / 1024.0
            }
            IbatStatuscodes::IbatStatChargeTrickle
            | IbatStatuscodes::IbatStatChargeCool
            | IbatStatuscodes::IbatStatChargeNormal => (ibat_raw * charger_max_current) / 1024.0,
            _ => 0.0,
        };
        return Ok(ibat_scaled);
    }
}

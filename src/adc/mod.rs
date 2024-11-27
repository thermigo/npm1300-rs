use crate::{common::Task, Vbatautoenable};

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Trigger VBAT measurement
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured VBAT voltage
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBAT measurement result
    pub async fn trigger_vbat_measurement(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
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

        // Read measurement result
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
        // 1023.0 is the maximum value for the 10 bit ADC.
        let result = (result as f32 / 1023.0) * 5.0;

        Ok(result)
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
            .write_async(|reg| {
                reg.set_vbatautoenable(if enable {
                    Vbatautoenable::Autoenable
                } else {
                    Vbatautoenable::Noauto
                })
            })
            .await
    }
}

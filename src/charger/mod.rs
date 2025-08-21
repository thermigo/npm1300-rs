mod types;

// Re-export everything in types.rs
pub use types::*;

use libm::roundf;

use crate::{common::Task, Bchgilimbatactive, Dietemphigh, NPM1300Error};

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Clear charger errors
    ///
    /// Clears charger errors in BCHGERRREASON and BCHGERRSENSOR registers.
    /// Also releases the charger from error state.
    pub async fn clear_charger_errors(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Clear charger errors in BCHGERRREASON and BCHGERRSENSOR
        self.device
            .charger()
            .taskclearchgerr()
            .dispatch_async(|command| command.set_taskclearchgerr(Task::Trigger))
            .await?;
        // Release the charger from error state
        self.device
            .charger()
            .taskreleaseerr()
            .dispatch_async(|command| command.set_taskreleaseerror(Task::Trigger))
            .await
    }

    /// Clear charger safety timer
    ///
    /// Resets the charger safety timer which enforces a 7-hour timeout for the combined
    /// constant current and constant voltage charging phases.
    ///
    /// # Safety
    ///
    /// When the safety timer expires, the host must verify charging conditions are safe
    /// before clearing the timer and resuming charging operations.
    pub async fn clear_charger_safety_timer(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Clear charger safety timer
        self.device
            .charger()
            .taskclearsafetytimer()
            .dispatch_async(|command| command.set_taskclearsafetytimer(Task::Trigger))
            .await
    }

    /// Enable battery charging
    ///
    /// Before enabling charging, ensure proper configuration of:
    /// - Charging current
    /// - Termination voltages for normal and warm temperature conditions
    /// - NTC thermistor settings (either configure type or disable if not present)
    /// - VBUS input current limit
    ///
    /// # Safety
    /// Improper charger configuration may lead to unsafe charging conditions.
    pub async fn enable_battery_charging(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Enable battery charging
        self.device
            .charger()
            .bchgenableset()
            .modify_async(|reg| reg.set_enablecharging(ChargerEnableSet::EnableCharger))
            .await
    }

    /// Disable battery charging
    pub async fn disable_battery_charging(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Disable battery charging
        self.device
            .charger()
            .bchgenableclr()
            .modify_async(|reg| reg.set_enablecharging(ChargerEnableClear::DisableCharger))
            .await
    }

    /// Get the charging enable status
    ///
    /// # Returns
    ///
    /// `true` if charging is enabled, `false` if disabled
    pub async fn is_charging_enabled(&mut self) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.charger().bchgenableset().read_async().await?;
        match status.enablecharging() {
            Ok(ChargerEnableSet::EnableCharger) => Ok(true),
            Ok(ChargerEnableSet::NoEffect) => Ok(false),
            Err(_) => panic!("Failed to read BCHGENABLESET register"),
        }
    }

    /// Enables maximum charge current in cool temperature conditions
    ///
    /// Enables 100% charge current as defined by BCHGISETMSB:BCHGISETLSB
    pub async fn enable_battery_charger_full_charge_in_cool_temp(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgenableset()
            .modify_async(|reg| {
                reg.set_enablefullchgcool(
                    ChargerEnableFullCurrentChargeInCoolTempSet::EnableFullCurrentChargeInCoolTemp,
                )
            })
            .await
    }

    /// Disables maximum charge current in cool temperature conditions
    ///
    /// Disables 100% charge current as defined by BCHGISETMSB:BCHGISETLSB
    pub async fn disable_battery_charger_full_charge_in_cool_temp(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgenableclr()
            .modify_async(|reg| {
                reg.set_enablefullchgcool(
                    ChargerEnableFullCurrentChargeInCoolTempClear::DisableFullCurrentChargeInCoolTemp,
                )
            })
            .await
    }

    /// Get the full charge current in cool temperature status
    ///
    /// # Returns
    ///
    /// `true` if full charge current in cool temperature is enabled, `false` if disabled
    pub async fn is_full_charge_current_in_cool_temp_enabled(
        &mut self,
    ) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.charger().bchgenableset().read_async().await?;
        match status.enablefullchgcool() {
            Ok(ChargerEnableFullCurrentChargeInCoolTempSet::EnableFullCurrentChargeInCoolTemp) => {
                Ok(true)
            }
            Ok(ChargerEnableFullCurrentChargeInCoolTempSet::NoEffect) => Ok(false),
            Err(_) => panic!("Failed to read BCHGENABLESET register"),
        }
    }

    /// Enables battery recharge once charged
    pub async fn enable_battery_recharge(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgdisableclr()
            .modify_async(|reg| {
                reg.set_disablerecharge(ChargerDisableRechargeClear::EnableRecharge)
            })
            .await
    }

    /// Disables battery recharge once charged
    pub async fn disable_battery_recharge(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgdisableset()
            .modify_async(|reg| reg.set_disablerecharge(ChargerDisableRechargeSet::DisableRecharge))
            .await
    }

    /// Get the battery recharge status
    ///
    /// # Returns
    ///
    /// `true` if battery recharge is enabled, `false` if disabled
    pub async fn is_battery_recharge_enabled(
        &mut self,
    ) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.charger().bchgdisableset().read_async().await?;
        match status.disablerecharge() {
            Ok(ChargerDisableRechargeSet::NoEffect) => Ok(true),
            Ok(ChargerDisableRechargeSet::DisableRecharge) => Ok(false),
            Err(_) => panic!("Failed to read BCHGDISABLESET register"),
        }
    }

    /// Ignores NTC thermistor measurements when charging
    pub async fn ignore_ntc_measurements(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgdisableset()
            .modify_async(|reg| reg.set_disablentc(DisableNtcSet::IgnoreNtc))
            .await
    }

    /// Uses NTC thermistor measurements when charging
    pub async fn use_ntc_measurements(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgdisableclr()
            .modify_async(|reg| reg.set_disablentc(DisableNtcClear::UseNtc))
            .await
    }

    /// Get the NTC thermistor measurements status
    ///
    /// # Returns
    ///
    /// `true` if NTC thermistor measurements are ignored, `false` if not ignored
    pub async fn is_ntc_measurement_ignored(
        &mut self,
    ) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.charger().bchgdisableset().read_async().await?;
        match status.disablentc() {
            Ok(DisableNtcSet::IgnoreNtc) => Ok(true),
            Ok(DisableNtcSet::NoEffect) => Ok(false),
            Err(_) => panic!("Failed to read BCHGDISABLESET register"),
        }
    }

    /// Configure the battery charger current
    ///
    /// Sets the maximum charging current for the battery charger.
    ///
    /// # Arguments
    ///
    /// * `current_ma` - The desired charging current in milliamps (mA). Maximum value is 800mA.
    ///
    /// # Errors
    ///
    /// Returns `NPM1300Error::ChargerCurrentTooHigh` if the requested current exceeds 800mA.
    ///
    /// # Note
    ///
    /// This function will temporarily disable charging while updating the current settings if
    /// charging is enabled, then restore the previous charging state.
    pub async fn set_charger_current(
        &mut self,
        current_ma: u16,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        const MAX_CURRENT_MA: u16 = 800;

        if current_ma > MAX_CURRENT_MA {
            return Err(crate::NPM1300Error::ChargerCurrentTooHigh(current_ma));
        }

        // Check if charging was enabled
        let was_enabled = self.is_charging_enabled().await?;

        // Only disable if it was enabled
        if was_enabled {
            self.disable_battery_charging().await?;
        }

        // Convert current to register values:
        // MSB = floor(current_ma/4)
        // LSB = 1 if (current_ma/2) is odd, 0 if even
        let msb = (current_ma / 4) as u8;
        let lsb = ((current_ma / 2) & 1) as u8;

        // Update MSB register
        self.device
            .charger()
            .bchgisetmsb()
            .write_async(|reg| reg.set_bchgisetchargemsb(msb))
            .await?;

        // Update LSB register
        self.device
            .charger()
            .bchgisetlsb()
            .write_async(|reg| reg.set_bchgisetchargelsb(lsb))
            .await?;

        // Only re-enable if it was enabled before
        if was_enabled {
            self.enable_battery_charging().await
        } else {
            Ok(())
        }
    }
    /// Get the configured battery charger current
    ///
    /// * `Ok(u16)` - The configured charging current in milliamps (mA)
    /// * `Err(NPM1300Error)` - An error occurred while reading the register values
    pub async fn get_charger_config_current(
        &mut self,
    ) -> Result<u16, crate::NPM1300Error<I2c::Error>> {
        let msb = self
            .device
            .charger()
            .bchgisetmsb()
            .read_async()
            .await?
            .bchgisetchargemsb();
        let lsb = self
            .device
            .charger()
            .bchgisetlsb()
            .read_async()
            .await?
            .bchgisetchargelsb();

        Ok((msb as u16) << 2 | (lsb as u16) << 1)
    }
    /// Set the battery discharge current limit
    ///
    /// Configures the maximum discharge current limit for the battery.
    ///
    /// # Arguments
    ///
    /// * `limit` - The discharge current limit setting:
    ///   * `DischargeCurrentLimit::Low` - Sets 200mA maximum discharge current
    ///   * `DischargeCurrentLimit::High` - Sets 1000mA maximum discharge current
    ///
    /// # Note
    ///
    /// * The low discharge current limit increases current measurement accuracy and
    ///   optimizes fuel gauge performance at lower discharge currents.
    /// * The high discharge current limit increases the maximum range of current measurement
    ///
    /// # Safety
    /// If the system load exceeds the discharge current limit, VSYS voltage drops below
    /// VSYSPOF causing the device to reset.
    pub async fn set_discharge_current_limit(
        &mut self,
        limit: DischargeCurrentLimit,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        let (msb, lsb) = match limit {
            DischargeCurrentLimit::Low => {
                // 200mA maximum discharge current
                // MSB = 42 (0x2A), LSB = 0
                (42, 0)
            }
            DischargeCurrentLimit::High => {
                // 1000mA maximum discharge current
                // MSB = 207 (0xCF), LSB = 1
                (207, 1)
            }
        };

        // Update MSB register
        self.device
            .charger()
            .bchgisetdischargemsb()
            .write_async(|reg| reg.set_bchgisetdischargemsb(msb))
            .await?;

        // Update LSB register
        self.device
            .charger()
            .bchgisetdischargelsb()
            .write_async(|reg| reg.set_bchgisetdischargelsb(lsb))
            .await
    }

    /// Set the battery charging termination voltage for normal temperature conditions
    ///
    /// Configures the voltage at which constant current charging starts when battery
    /// temperature is in the normal range.
    ///
    /// # Arguments
    ///
    /// * `termination_voltage` - The desired termination voltage setting
    pub async fn set_normal_temperature_termination_voltage(
        &mut self,
        termination_voltage: ChargerTerminationVoltage,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgvterm()
            .write_async(|reg| reg.set_bchgvtermnorm(termination_voltage))
            .await
    }

    /// Set the battery charging termination voltage for warm temperature conditions
    ///
    /// Configures the voltage at which constant current charging starts when battery
    /// temperature is in the warm range.
    ///
    /// # Arguments
    ///
    /// * `termination_voltage` - The desired termination voltage setting
    pub async fn set_warm_temperature_termination_voltage(
        &mut self,
        termination_voltage: ChargerTerminationVoltage,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgvtermr()
            .write_async(|reg| reg.set_bchgvtermreduced(termination_voltage))
            .await
    }

    /// Set the battery charging trickle level
    ///
    /// Trickle charging current, ITRICKLE, is 10% of ICHG.
    /// Trickle charging is active when VBAT < VTRICKLE_FAST (default 2.9 V).
    ///
    /// # Arguments
    ///
    /// * `trickle_level` - The desired trickle level setting
    pub async fn set_trickle_level(
        &mut self,
        trickle_level: ChargerTrickleLevelSelect,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgvtricklesel()
            .write_async(|reg| reg.set_bchgvtricklesel(trickle_level))
            .await
    }

    /// Set the battery charging termination current level
    ///
    /// Termination current is active in the constant voltage phase of charging.
    /// Charging stops when the charging current reduces to this value.
    ///
    /// # Arguments
    ///
    /// * `termination_current_level` - The desired termination current level setting
    pub async fn set_termination_current_level(
        &mut self,
        termination_current_level: ChargerTerminationCurrentLevelSelect,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgitermsel()
            .write_async(|reg| reg.set_bchgitermsel(termination_current_level))
            .await
    }

    /// Set an NTC temperature threshold for the charger device
    ///
    /// Sets the threshold based on the desired and reference (25°C) resistances of the NTC thermistor.
    ///
    /// # Arguments
    ///
    /// * `region` - The temperature region to set the threshold for (Cold, Cool, Warm, or Hot)
    /// * `desired_resistance` - The NTC resistance at the desired threshold temperature
    /// * `reference_resistance_25c` - The NTC resistance at 25 degrees Celsius
    ///
    /// # Errors
    ///
    /// Returns `NPM1300Error::InvalidNtcThreshold` if the computed threshold is outside the 10-bit allowable range
    pub async fn set_ntc_threshold(
        &mut self,
        region: NtcThresholdRegion,
        desired_resistance: u32,
        reference_resistance_25c: u32,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Calculate the 10-bit threshold
        let threshold = roundf(
            1024.0 * (desired_resistance as f32)
                / (desired_resistance as f32 + reference_resistance_25c as f32),
        );

        // Ensure the threshold fits within a 10-bit range
        if !(0.0..=1023.0).contains(&threshold) {
            return Err(crate::NPM1300Error::InvalidNtcThreshold);
        }

        // Convert the threshold to a 10-bit unsigned integer
        let threshold = threshold as u16;

        // Extract MSB (upper 8 bits) and LSB (lower 2 bits)
        let msb = (threshold >> 2) as u8;
        let lsb = (threshold & 0x03) as u8;

        // Write MSB and LSB to respective registers based on the temperature region
        match region {
            NtcThresholdRegion::Cold => {
                self.device
                    .charger()
                    .ntccold()
                    .write_async(|reg| reg.set_ntccoldlvlmsb(msb))
                    .await?;
                self.device
                    .charger()
                    .ntccoldlsb()
                    .write_async(|reg| reg.set_ntccoldlvllsb(lsb))
                    .await
            }
            NtcThresholdRegion::Cool => {
                self.device
                    .charger()
                    .ntccool()
                    .write_async(|reg| reg.set_ntccoollvlmsb(msb))
                    .await?;
                self.device
                    .charger()
                    .ntccoollsb()
                    .write_async(|reg| reg.set_ntccoollvllsb(lsb))
                    .await
            }
            NtcThresholdRegion::Warm => {
                self.device
                    .charger()
                    .ntcwarm()
                    .write_async(|reg| reg.set_ntcwarmlvlmsb(msb))
                    .await?;
                self.device
                    .charger()
                    .ntcwarmlsb()
                    .write_async(|reg| reg.set_ntcwarmlvllsb(lsb))
                    .await
            }
            NtcThresholdRegion::Hot => {
                self.device
                    .charger()
                    .ntchot()
                    .write_async(|reg| reg.set_ntchotlvlmsb(msb))
                    .await?;
                self.device
                    .charger()
                    .ntchotlsb()
                    .write_async(|reg| reg.set_ntchotlvllsb(lsb))
                    .await
            }
        }
    }

    /// Get the configured NTC threshold
    ///
    /// # Arguments
    ///
    /// * `region` - The temperature region to get the threshold for (Cold, Cool, Warm, or Hot)
    ///
    /// # Returns
    ///
    /// * `Ok(u16)` - The NTC threshold for the specified temperature region
    /// * `Err(NPM1300Error)` - An error occurred while reading the NTC threshold
    pub async fn get_ntc_threshold(
        &mut self,
        region: NtcThresholdRegion,
    ) -> Result<u16, crate::NPM1300Error<I2c::Error>> {
        match region {
            NtcThresholdRegion::Cold => {
                let msb = self
                    .device
                    .charger()
                    .ntccold()
                    .read_async()
                    .await?
                    .ntccoldlvlmsb();
                let lsb = self
                    .device
                    .charger()
                    .ntccoldlsb()
                    .read_async()
                    .await?
                    .ntccoldlvllsb();
                Ok((msb as u16) << 2 | (lsb as u16))
            }
            NtcThresholdRegion::Cool => {
                let msb = self
                    .device
                    .charger()
                    .ntccool()
                    .read_async()
                    .await?
                    .ntccoollvlmsb();
                let lsb = self
                    .device
                    .charger()
                    .ntccoollsb()
                    .read_async()
                    .await?
                    .ntccoollvllsb();
                Ok((msb as u16) << 2 | (lsb as u16))
            }
            NtcThresholdRegion::Warm => {
                let msb = self
                    .device
                    .charger()
                    .ntcwarm()
                    .read_async()
                    .await?
                    .ntcwarmlvlmsb();
                let lsb = self
                    .device
                    .charger()
                    .ntcwarmlsb()
                    .read_async()
                    .await?
                    .ntcwarmlvllsb();
                Ok((msb as u16) << 2 | (lsb as u16))
            }
            NtcThresholdRegion::Hot => {
                let msb = self
                    .device
                    .charger()
                    .ntchot()
                    .read_async()
                    .await?
                    .ntchotlvlmsb();
                let lsb = self
                    .device
                    .charger()
                    .ntchotlsb()
                    .read_async()
                    .await?
                    .ntchotlvllsb();
                Ok((msb as u16) << 2 | (lsb as u16))
            }
        }
    }

    /// Set the die temperature threshold for the charger device
    ///
    /// Sets the temperature threshold based on the desired temperature in degrees Celsius.
    ///
    /// # Arguments
    ///
    /// * `threshold_type` - The type of threshold to set (Stop or Resume)
    /// * `temperature_celsius` - The desired temperature in degrees Celsius
    ///
    /// # Errors
    ///
    /// Returns `NPM1300Error::InvalidDieTemperatureThreshold` if the computed threshold is outside the 10-bit allowable range
    pub async fn set_die_temperature_threshold(
        &mut self,
        threshold_type: DieTemperatureThresholdType,
        temperature_celsius: u16,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Check if the stop threshold is within a valid range.
        // No range is specified in the datasheet, but the given
        // examples are within the range 50-110 degrees Celsius.
        // This check also ensures the computed threshold fits within
        // a 10-bit range.
        if !(50..=110).contains(&temperature_celsius) {
            return Err(crate::NPM1300Error::InvalidDieTemperatureThreshold);
        }

        // Calculate the 10-bit threshold
        let k_die_temp = roundf((394.67 - temperature_celsius as f32) / 0.7926);

        // Convert the threshold to a 10-bit unsigned integer
        let k_die_temp = k_die_temp as u16;

        // Extract MSB (upper 8 bits) and LSB (lower 2 bits)
        let msb = (k_die_temp >> 2) as u8;
        let lsb = (k_die_temp & 0x03) as u8;

        // Write MSB and LSB to respective registers
        match threshold_type {
            DieTemperatureThresholdType::Stop => {
                self.device
                    .charger()
                    .dietempstop()
                    .write_async(|reg| reg.set_dietempstopchg(msb))
                    .await?;
                self.device
                    .charger()
                    .dietempstoplsb()
                    .write_async(|reg| reg.set_dietempstopchglsb(lsb))
                    .await
            }
            DieTemperatureThresholdType::Resume => {
                self.device
                    .charger()
                    .dietempresume()
                    .write_async(|reg| reg.set_dietempresumechg(msb))
                    .await?;
                self.device
                    .charger()
                    .dietempresumelsb()
                    .write_async(|reg| reg.set_dietempresumechglsb(lsb))
                    .await
            }
        }
    }

    /// Get the discharge current limit active status
    ///
    /// Returns true if discharge current limit is active, false if not
    pub async fn get_discharge_current_limit_active(
        &mut self,
    ) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.charger().bchgilimstatus().read_async().await?;
        match status.bchgilimbatactive() {
            Bchgilimbatactive::Inactive => Ok(false),
            Bchgilimbatactive::Active => Ok(true),
        }
    }

    // TODO: test this behavior
    /// Get the NTC current region
    pub async fn get_ntc_current_region(
        &mut self,
    ) -> Result<NtcThresholdRegion, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.charger().ntcstatus().read_async().await?;
        if status.ntccold() == 1 {
            Ok(NtcThresholdRegion::Cold)
        } else if status.ntccool() == 1 {
            Ok(NtcThresholdRegion::Cool)
        } else if status.ntcwarm() == 1 {
            Ok(NtcThresholdRegion::Warm)
        } else if status.ntchot() == 1 {
            Ok(NtcThresholdRegion::Hot)
        } else {
            core::unreachable!();
        }
    }

    /// Check if the die temperature exceeds charging threshold
    ///
    /// # Returns
    ///
    /// `true` if the die temperature exceeds the configured DIETEMPSTOP threshold,
    /// indicating charging should be disabled. `false` if the temperature is within
    /// normal operating range.
    ///
    /// # Returns
    ///
    /// `true` if the die temperature exceeds the configured DIETEMPSTOP threshold,
    /// indicating charging should be disabled. `false` if the temperature is within
    /// normal operating range.
    pub async fn is_die_temperature_above_charging_threshold(
        &mut self,
    ) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.charger().dietempstatus().read_async().await?;
        Ok(status.dietemphigh() == Dietemphigh::High)
    }

    /// Get the charger status
    ///
    /// # Returns
    ///
    /// A `ChargerStatus` struct containing the current status of the charger
    pub async fn get_charger_status(
        &mut self,
    ) -> Result<ChargerStatus, crate::NPM1300Error<I2c::Error>> {
        let status = self
            .device
            .charger()
            .bchgchargestatus()
            .read_async()
            .await?;

        Ok(ChargerStatus {
            is_battery_present: status.batterydetected() == 1,
            is_charging_complete: status.completed() == 1,
            is_trickle_charging: status.tricklecharge() == 1,
            is_constant_current_charging: status.constantcurrent() == 1,
            is_constant_voltage_charging: status.constantvoltage() == 1,
            needs_recharge: status.recharge() == 1,
            is_charging_paused_by_die_temperature: status.dietemphighchgpaused() == 1,
            is_supplement_mode_active: status.supplementactive() == 1,
        })
    }

    /// Get the charger error reason and sensor value
    ///
    /// # Returns
    ///
    /// A tuple containing the charger error reason and sensor value
    pub async fn get_charger_error_reason_and_sensor_value(
        &mut self,
    ) -> Result<(ChargerErrorReason, ChargerSensorValueDuringError), crate::NPM1300Error<I2c::Error>>
    {
        let status = self.device.charger().bchgerrreason().read_async().await?;
        let sensor = self.device.charger().bchgerrsensor().read_async().await?;

        Ok((
            ChargerErrorReason {
                ntc_sensor_error: status.ntcsensorerror() == 1,
                vbat_sensor_error: status.vbatsensorerror() == 1,
                vbat_low_error: status.vbatlow() == 1,
                vtrickle_error: status.vtrickle() == 1,
                measurement_timeout_error: status.meastimeout() == 1,
                charge_timeout_error: status.chargetimeout() == 1,
                trickle_timeout_error: status.trickletimeout() == 1,
            },
            ChargerSensorValueDuringError {
                sensor_ntc_cold: sensor.sensorntccold() == 1,
                sensor_ntc_cool: sensor.sensorntccool() == 1,
                sensor_ntc_warm: sensor.sensorntcwarm() == 1,
                sensor_ntc_hot: sensor.sensorntchot() == 1,
                sensor_vterm: sensor.sensorvterm() == 1,
                sensor_recharge: sensor.sensorrecharge() == 1,
                sensor_vtrickle: sensor.sensorvtrickle() == 1,
                sensor_vbat_low: sensor.sensorvbatlow() == 1,
            },
        ))
    }

    /// Enable or disable charging when battery is warm
    ///
    /// Controls whether charging is allowed to continue when battery temperature is in the warm range.
    ///
    /// # Arguments
    ///
    /// * `enable` - If true, allows charging if battery is warm. If false, disables charging.
    pub async fn set_charge_if_battery_warm(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .charger()
            .bchgconfig()
            .write_async(|reg| {
                reg.set_disablechargewarm(if enable {
                    ChargerConfigDisableChargeWarm::ENABLED
                } else {
                    ChargerConfigDisableChargeWarm::DISABLED
                })
            })
            .await
    }

    /// Get the current discharge current limit
    ///
    /// #returns
    /// DischargeCurrentLimit enum
    pub async fn get_discharge_current_limit(
        &mut self,
    ) -> Result<DischargeCurrentLimit, crate::NPM1300Error<I2c::Error>> {
        let msb = self
            .device
            .charger()
            .bchgisetdischargemsb()
            .read_async()
            .await?
            .bchgisetdischargemsb();
        let lsb = self
            .device
            .charger()
            .bchgisetdischargelsb()
            .read_async()
            .await?
            .bchgisetdischargelsb();

        match (msb, lsb) {
            (42, 0) => Ok(DischargeCurrentLimit::Low),   // 200mA case
            (207, 1) => Ok(DischargeCurrentLimit::High), // 1000mA case
            _ => Err(NPM1300Error::InvalidDischargeCurrentValue { msb, lsb }),
        }
    }
}

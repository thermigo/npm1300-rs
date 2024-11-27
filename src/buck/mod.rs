use crate::{
    common::Task,
    gpios::{Gpio, GpioMode, GpioPolarity},
    Buck1Autoctrlsel, Buck1Enpulldown, Buck1Swctrlsel, Buck2Autoctrlsel, Buck2Enpulldown,
    Buck2Swctrlsel, Buckstatus, Buckvoutstatus,
};

mod types;

// Re-export everything in types.rs
pub use types::*;

/// Convert a GPIO enum value to its register index
///
/// GPIOs are 1-indexed in the nPM1300 so we subtract 1 from the GPIO number
/// to get the register index
fn gpio_to_register_index(gpio: Gpio) -> usize {
    usize::from(u8::from(gpio) - 1)
}

pub struct Config {
    /// GPIO to enable/disable BUCK regulators
    pub gpio_buck_enable_control: Gpio,
    /// GPIO enable/disable polarity
    pub gpio_buck_enable_control_polarity: GpioPolarity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gpio_buck_enable_control: Gpio::None,
            gpio_buck_enable_control_polarity: GpioPolarity::NotInverted,
        }
    }
}

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Enable or disable a BUCK regulator
    ///
    /// # Arguments
    ///
    /// * `buck_index` - Index of the BUCK regulator (0 for BUCK1, 1 for BUCK2)
    /// * `enable` - true to enable the regulator, false to disable it
    async fn control_buck_power(
        &mut self,
        buck_index: u8,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        if enable {
            // Enable the regulator
            self.device
                .buck()
                .buckenaset(buck_index.into())
                .dispatch_async(|command| command.set_taskbuckenaset(Task::Trigger))
                .await
        } else {
            // Disable the regulator
            self.device
                .buck()
                .buckenaclr(buck_index.into())
                .dispatch_async(|command| command.set_taskbuckenaclr(Task::Trigger))
                .await
        }
    }

    /// Enable BUCK1
    pub async fn enable_buck1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_buck_power(0, true).await
    }

    /// Disable BUCK1
    pub async fn disable_buck1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_buck_power(0, false).await
    }

    /// Enable BUCK2
    pub async fn enable_buck2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_buck_power(1, true).await
    }

    /// Disable BUCK2
    pub async fn disable_buck2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_buck_power(1, false).await
    }

    /// Enable or disable forced PWM mode for a BUCK regulator
    ///
    /// # Arguments
    ///
    /// * `buck_index` - Index of the BUCK regulator (0 for BUCK1, 1 for BUCK2)
    /// * `enable` - true to enable forced PWM mode, false to return to auto mode
    async fn set_buck_forced_pwm_mode(
        &mut self,
        buck_index: u8,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        if enable {
            // Enable forced PWM mode
            self.device
                .buck()
                .buckpwmset(buck_index.into())
                .dispatch_async(|command| command.set_taskbuckpwmset(Task::Trigger))
                .await
        } else {
            // Disable forced PWM mode
            self.device
                .buck()
                .buckpwmclr(buck_index.into())
                .dispatch_async(|command| command.set_taskbuckpwmclr(Task::Trigger))
                .await
        }
    }

    /// Enable BUCK1 forced PWM mode
    pub async fn enable_buck1_forced_pwm_mode(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.set_buck_forced_pwm_mode(0, true).await
    }

    /// Disable BUCK1 forced PWM mode and return to auto mode
    pub async fn disable_buck1_forced_pwm_mode(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.set_buck_forced_pwm_mode(0, false).await
    }

    /// Enable BUCK2 forced PWM mode
    pub async fn enable_buck2_forced_pwm_mode(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.set_buck_forced_pwm_mode(1, true).await
    }

    /// Disable BUCK2 forced PWM mode and return to auto mode
    pub async fn disable_buck2_forced_pwm_mode(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.set_buck_forced_pwm_mode(1, false).await
    }

    /// Set BUCK1 normal mode output voltage
    ///
    /// # Arguments
    ///
    /// * `voltage` - The voltage to set for BUCK1. See [`BuckVoltage`] for available values.
    pub async fn set_buck1_normal_voltage(
        &mut self,
        voltage: BuckVoltage,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Set BUCK1 normal mode output voltage
        self.device
            .buck()
            .buck_1_normvout()
            .write_async(|reg| reg.set_value(voltage))
            .await?;
        // Allow SW to override VSET pin
        self.device
            .buck()
            .buckswctrlsel()
            .write_async(|reg| reg.set_buck_1_swctrlsel(Buck1Swctrlsel::Swctrl))
            .await
    }

    /// Allow VSET pins to set BUCK1 VOUT
    pub async fn enable_buck1_vset_voltage(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckswctrlsel()
            .write_async(|reg| reg.set_buck_1_swctrlsel(Buck1Swctrlsel::Vsetandswctrl))
            .await
    }

    /// Allow VSET pins to set BUCK2 VOUT
    pub async fn enable_buck2_vset_voltage(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckswctrlsel()
            .write_async(|reg| reg.set_buck_2_swctrlsel(Buck2Swctrlsel::Vsetandswctrl))
            .await
    }

    /// Set BUCK2 normal mode output voltage
    pub async fn set_buck2_normal_voltage(
        &mut self,
        voltage: BuckVoltage,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Set BUCK2 normal mode output voltage
        self.device
            .buck()
            .buck_2_normvout()
            .write_async(|reg| reg.set_value(voltage))
            .await?;
        // Allow SW to override VSET pin
        self.device
            .buck()
            .buckswctrlsel()
            .write_async(|reg| reg.set_buck_2_swctrlsel(Buck2Swctrlsel::Swctrl))
            .await
    }

    /// Configure retention mode for a BUCK regulator
    ///
    /// # Arguments
    ///
    /// * `buck_index` - Index of the BUCK regulator (1 for BUCK1, 2 for BUCK2)
    /// * `voltage` - The voltage to set for the BUCK. See [`BuckVoltage`] for available values.
    /// * `gpio` - The GPIO to set for retention mode. See [`Gpio`] for available values.
    /// * `polarity` - The polarity of the GPIO. See [`GpioPolarity`] for available values.
    async fn configure_buck_retention_mode(
        &mut self,
        buck_index: u8,
        voltage: BuckVoltage,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Configure GPIO mode as input
        self.device
            .gpios()
            .gpiomode(gpio_to_register_index(gpio))
            .write_async(|reg| reg.set_gpiomode(GpioMode::GpiInput))
            .await?;
        // Set retention mode GPIO and its polarity
        self.device
            .buck()
            .buckvretctrl()
            .modify_async(|reg| match buck_index {
                1 => {
                    reg.set_buck_1_vretgpisel(gpio);
                    reg.set_buck_1_vretgpiinv(polarity);
                }
                2 => {
                    reg.set_buck_2_vretgpisel(gpio);
                    reg.set_buck_2_vretgpiinv(polarity);
                }
                _ => panic!("Invalid BUCK index"),
            })
            .await?;
        // Set retention mode output voltage
        match buck_index {
            1 => {
                self.device
                    .buck()
                    .buck_1_retvout()
                    .write_async(|reg| reg.set_value(voltage))
                    .await?
            }
            2 => {
                self.device
                    .buck()
                    .buck_2_retvout()
                    .write_async(|reg| reg.set_value(voltage))
                    .await?
            }
            _ => panic!("Invalid BUCK index"),
        }
        // Allow SW to override VSET pin
        self.device
            .buck()
            .buckswctrlsel()
            .write_async(|reg| match buck_index {
                1 => reg.set_buck_1_swctrlsel(Buck1Swctrlsel::Swctrl),
                2 => reg.set_buck_2_swctrlsel(Buck2Swctrlsel::Swctrl),
                _ => panic!("Invalid BUCK index"),
            })
            .await
    }

    /// Configure BUCK1 retention mode
    ///
    /// # Arguments
    ///
    /// * `voltage` - The voltage to set for the BUCK. See [`BuckVoltage`] for available values.
    /// * `gpio` - The GPIO to set for retention mode. See [`Gpio`] for available values.
    /// * `polarity` - The polarity of the GPIO. See [`GpioPolarity`] for available values.
    pub async fn configure_buck1_retention_mode(
        &mut self,
        voltage: BuckVoltage,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_buck_retention_mode(1, voltage, gpio, polarity)
            .await
    }

    /// Configure BUCK2 retention mode
    ///
    /// # Arguments
    ///
    /// * `voltage` - The voltage to set for the BUCK. See [`BuckVoltage`] for available values.
    /// * `gpio` - The GPIO to set for retention mode. See [`Gpio`] for available values.
    /// * `polarity` - The polarity of the GPIO. See [`GpioPolarity`] for available values.
    pub async fn configure_buck2_retention_mode(
        &mut self,
        voltage: BuckVoltage,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_buck_retention_mode(2, voltage, gpio, polarity)
            .await
    }

    /// Disable BUCK1 retention mode
    pub async fn disable_buck1_retention(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckvretctrl()
            .write_async(|reg| reg.set_buck_1_vretgpisel(Gpio::None))
            .await
    }

    /// Disable BUCK2 retention mode
    pub async fn disable_buck2_retention(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckvretctrl()
            .write_async(|reg| reg.set_buck_2_vretgpisel(Gpio::None))
            .await
    }

    /// Configure BUCK GPIO enable control
    ///
    /// # Arguments
    /// * `buck_index` - BUCK index
    /// * `gpio` - GPIO to enable
    /// * `polarity` - Polarity of GPIO
    async fn configure_buck_gpio_enable_control(
        &mut self,
        buck_index: u8,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        if gpio != Gpio::None {
            // Configure GPIO mode as input
            self.device
                .gpios()
                .gpiomode(gpio_to_register_index(gpio))
                .write_async(|reg| reg.set_gpiomode(GpioMode::GpiInput))
                .await?;

            // Configure GPIO and its polarity
            self.device
                .buck()
                .buckenctrl()
                .write_async(|reg| match buck_index {
                    1 => {
                        reg.set_buck_1_engpisel(gpio);
                        reg.set_buck_1_engpiinv(polarity);
                    }
                    2 => {
                        reg.set_buck_2_engpisel(gpio);
                        reg.set_buck_2_engpiinv(polarity);
                    }
                    _ => panic!("Invalid BUCK index"),
                })
                .await?;
        }
        Ok(())
    }

    /// Configure BUCK1 GPIO enable control
    ///
    /// # Arguments
    /// * `gpio` - GPIO to enable
    /// * `polarity` - Polarity of GPIO
    pub async fn set_buck1_gpio_control(
        &mut self,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_buck_gpio_enable_control(1, gpio, polarity)
            .await
    }

    /// Configure BUCK2 GPIO enable control
    ///
    /// # Arguments
    /// * `gpio` - GPIO to enable
    /// * `polarity` - Polarity of GPIO
    pub async fn set_buck2_gpio_control(
        &mut self,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_buck_gpio_enable_control(2, gpio, polarity)
            .await
    }

    /// Configure BUCK GPIO forced PWM mode control
    ///
    /// # Arguments
    /// * `buck_index` - BUCK index
    /// * `gpio` - GPIO to enable
    /// * `polarity` - Polarity of GPIO
    async fn configure_buck_forced_pwm_mode_control(
        &mut self,
        buck_index: u8,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        if gpio != Gpio::None {
            // Configure GPIO mode as input
            self.device
                .gpios()
                .gpiomode(gpio_to_register_index(gpio))
                .write_async(|reg| reg.set_gpiomode(GpioMode::GpiInput))
                .await?;

            // Configure GPIO and its polarity
            self.device
                .buck()
                .buckpwmctrl()
                .write_async(|reg| match buck_index {
                    1 => {
                        reg.set_buck_1_pwmgpisel(gpio);
                        reg.set_buck_1_pwmgpiinv(polarity);
                    }
                    2 => {
                        reg.set_buck_2_pwmgpisel(gpio);
                        reg.set_buck_2_pwmgpiinv(polarity);
                    }
                    _ => panic!("Invalid BUCK index"),
                })
                .await?;
        }
        Ok(())
    }

    /// Configure BUCK1 GPIO forced PWM mode control
    ///
    /// # Arguments
    /// * `gpio` - GPIO to enable
    /// * `polarity` - Polarity of GPIO
    pub async fn set_buck1_gpio_forced_pwm_mode_control(
        &mut self,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_buck_forced_pwm_mode_control(1, gpio, polarity)
            .await
    }

    /// Configure BUCK2 GPIO forced PWM mode control
    ///
    /// # Arguments
    /// * `gpio` - GPIO to enable
    /// * `polarity` - Polarity of GPIO
    pub async fn set_buck2_gpio_forced_pwm_mode_control(
        &mut self,
        gpio: Gpio,
        polarity: GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_buck_forced_pwm_mode_control(2, gpio, polarity)
            .await
    }

    /// Get BUCK1 VOUT status
    ///
    /// Note: the current voltage output setting can be read, but it is not measured by an ADC.
    ///
    /// # Returns
    /// * `Ok(Buckvoutstatus)` - The current BUCK1 VOUT status
    /// * `Err(NPM1300Error)` - An error occurred while reading the BUCK1 VOUT status
    pub async fn get_buck1_vout_status(
        &mut self,
    ) -> Result<Buckvoutstatus, crate::NPM1300Error<I2c::Error>> {
        self.device.buck().buckvoutstatus(0).read_async().await
    }

    /// Get BUCK2 VOUT status
    ///
    /// Note: the current voltage output setting can be read, but it is not measured by an ADC.
    ///
    /// # Returns
    /// * `Ok(Buckvoutstatus)` - The current BUCK2 VOUT status
    /// * `Err(NPM1300Error)` - An error occurred while reading the BUCK2 VOUT status
    pub async fn get_buck2_vout_status(
        &mut self,
    ) -> Result<Buckvoutstatus, crate::NPM1300Error<I2c::Error>> {
        self.device.buck().buckvoutstatus(1).read_async().await
    }

    /// Configure BUCK1 operating mode
    ///
    /// # Arguments
    /// * `mode` - The operating mode to set for BUCK1 see [`Buck1Autoctrlsel`] for available values.
    pub async fn configure_buck1_operating_mode(
        &mut self,
        mode: Buck1Autoctrlsel,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckctrl_0()
            .write_async(|reg| reg.set_buck_1_autoctrlsel(mode))
            .await
    }

    /// Configure BUCK1 operating mode
    ///
    /// # Arguments
    /// * `mode` - The operating mode to set for BUCK2 see [`Buck2Autoctrlsel`] for available values.
    pub async fn configure_buck2_operating_mode(
        &mut self,
        mode: Buck2Autoctrlsel,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckctrl_0()
            .write_async(|reg| reg.set_buck_2_autoctrlsel(mode))
            .await
    }

    /// Enable BUCK1 pull down
    pub async fn enable_buck1_pull_down(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckctrl_0()
            .write_async(|reg| reg.set_buck_1_enpulldown(Buck1Enpulldown::High))
            .await
    }

    /// Disable BUCK1 pull down
    pub async fn disable_buck1_pull_down(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckctrl_0()
            .write_async(|reg| reg.set_buck_1_enpulldown(Buck1Enpulldown::Low))
            .await
    }

    /// Enable BUCK2 pull down
    pub async fn enable_buck2_pull_down(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckctrl_0()
            .write_async(|reg| reg.set_buck_2_enpulldown(Buck2Enpulldown::High))
            .await
    }

    /// Disable BUCK2 pull down
    pub async fn disable_buck2_pull_down(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .buck()
            .buckctrl_0()
            .write_async(|reg| reg.set_buck_2_enpulldown(Buck2Enpulldown::Low))
            .await
    }

    /// Get BUCK status
    pub async fn get_buck_status(&mut self) -> Result<Buckstatus, crate::NPM1300Error<I2c::Error>> {
        self.device.buck().buckstatus().read_async().await
    }
}

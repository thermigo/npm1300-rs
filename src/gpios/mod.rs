mod types;

// Re-export everything in types.rs
pub use types::*;

/// Builder pattern for GPIO configuration
pub struct GpioConfigBuilder {
    config: GpioConfig,
}

/// GPIO Configuration structure
///
/// Pull-down is prioritized if both pull-up and pull-down are activated on a GPIO pin at the same time.
#[derive(Debug, Clone)]
pub struct GpioConfig {
    mode: GpioMode,
    drive_strength: GpioDriveStrength,
    pull_up: GpioPullUp,
    pull_down: GpioPullDown,
    open_drain: GpioOpenDrain,
    debounce: GpioDebounce,
}

impl Default for GpioConfig {
    fn default() -> Self {
        Self {
            mode: GpioMode::GpiInput,
            drive_strength: GpioDriveStrength::Drive1mA,
            pull_up: GpioPullUp::Disable,
            pull_down: GpioPullDown::Enable,
            open_drain: GpioOpenDrain::Disable,
            debounce: GpioDebounce::Disable,
        }
    }
}

impl GpioConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: GpioConfig::default(),
        }
    }

    pub fn mode(mut self, mode: GpioMode) -> Self {
        self.config.mode = mode;
        self
    }

    pub fn drive_strength(mut self, strength: GpioDriveStrength) -> Self {
        self.config.drive_strength = strength;
        self
    }

    pub fn pull_up(mut self, pull_up: GpioPullUp) -> Self {
        self.config.pull_up = pull_up;
        self
    }

    pub fn pull_down(mut self, pull_down: GpioPullDown) -> Self {
        self.config.pull_down = pull_down;
        self
    }

    pub fn open_drain(mut self, open_drain: GpioOpenDrain) -> Self {
        self.config.open_drain = open_drain;
        self
    }

    pub fn debounce(mut self, debounce: GpioDebounce) -> Self {
        self.config.debounce = debounce;
        self
    }

    pub fn build(self) -> GpioConfig {
        self.config
    }
}

impl Default for GpioConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    pub async fn configure_gpio(
        &mut self,
        pin: usize,
        config: GpioConfig,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        if pin > 4 {
            panic!("GPIO pin number must be between 0 and 4");
        }
        // GPIO mode configuration
        self.device
            .gpios()
            .gpiomode(pin)
            .write_async(|reg| reg.set_gpiomode(config.mode))
            .await?;

        // GPIO drive strength configuration
        self.device
            .gpios()
            .gpiodrive(pin)
            .write_async(|reg| reg.set_gpiodrive(config.drive_strength))
            .await?;

        // GPIO pull-up enable configuration
        self.device
            .gpios()
            .gpiopuen(pin)
            .write_async(|reg| reg.set_gpiopuen(config.pull_up))
            .await?;

        // GPIO pull-down enable configuration
        self.device
            .gpios()
            .gpiopden(pin)
            .write_async(|reg| reg.set_gpiopden(config.pull_down))
            .await?;

        // GPIO open drain configuration
        self.device
            .gpios()
            .gpioopendrain(pin)
            .write_async(|reg| reg.set_gpioopendrain(config.open_drain))
            .await?;

        // GPIO debounce configuration
        self.device
            .gpios()
            .gpiodebounce(pin)
            .write_async(|reg| reg.set_gpiodebounce(config.debounce))
            .await?;
        Ok(())
    }

    /// Get GPIO status
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin number (0-4)
    pub async fn get_gpio_status(
        &mut self,
        pin: usize,
    ) -> Result<GpioStatus, crate::NPM1300Error<I2c::Error>> {
        if pin > 4 {
            panic!("GPIO pin number must be between 0 and 4");
        }
        let status = self.device.gpios().gpiostatus().read_async().await?;

        Ok(match pin {
            0 => status.gpio_0_status().unwrap(),
            1 => status.gpio_1_status().unwrap(),
            2 => status.gpio_2_status().unwrap(),
            3 => status.gpio_3_status().unwrap(),
            4 => status.gpio_4_status().unwrap(),
            _ => unreachable!(), // We already checked pin range above
        })
    }
}

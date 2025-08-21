use crate::{
    common::Task,
    field_sets::Ldswstatus,
    //gpios::{Gpio, GpioPolarity},
};

mod types;

// Re-export everything in types.rs
pub use types::*;

/// Convert a GPIO enum value to its register index
///
/// GPIOs are 1-indexed in the nPM1300 so we subtract 1 from the GPIO number
/// to get the register index
/*
fn gpio_to_register_index(gpio: Gpio) -> usize {
    usize::from(u8::from(gpio) - 1)
}

pub struct Config {
    /// GPIO to enable/disable LDSW regulators
    pub gpio_ldsw_enable_control: Gpio,
    /// GPIO enable/disable polarity
    pub gpio_ldsw_enable_control_polarity: GpioPolarity,
}


impl Default for Config {
    fn default() -> Self {
        Self {
            gpio_ldsw_enable_control: Gpio::None,
            gpio_ldsw_enable_control_polarity: GpioPolarity::NotInverted,
        }
    }
}
*/

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Enable or disable a LDSW regulator
    ///
    /// # Arguments
    ///
    /// * `ldsw_index` - Index of the LDSW regulator (0 for LDSW1, 1 for LDSW2)
    /// * `enable` - true to enable the regulator, false to disable it
    async fn control_ldsw_power(
        &mut self,
        ldsw_index: u8,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        if enable {
            // Enable the regulator
            self.device
                .ldsw()
                .ldswset(ldsw_index.into())
                .dispatch_async(|command| command.set_taskldswset(Task::Trigger))
                .await
        } else {
            // Disable the regulator
            self.device
                .ldsw()
                .ldswclr(ldsw_index.into())
                .dispatch_async(|command| command.set_taskldswclr(Task::Trigger))
                .await
        }
    }

    /// Enable LDSW1
    pub async fn enable_ldsw1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(0, true).await
    }

    /// Disable LDSW1
    pub async fn disable_ldsw1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(0, false).await
    }

    /// Enable LDSW2
    pub async fn enable_ldsw2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(1, true).await
    }

    /// Disable LDSW2
    pub async fn disable_ldsw2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(1, false).await
    }

    /// Get LDSW status
    pub async fn get_ldsw_status(&mut self) -> Result<Ldswstatus, crate::NPM1300Error<I2c::Error>> {
        self.device.ldsw().ldswstatus().read_async().await
    }
}

use crate::common::Task;

mod types;

// Re-export everything in types.rs
pub use types::*;

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Configure LED driver
    ///
    /// # Arguments
    /// * `led` - LED number
    /// * `mode` - LED mode configuration see [`LedMode`]
    async fn configure_led_mode(
        &mut self,
        led: usize,
        mode: LedMode,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Configure LED mode
        self.device
            .leddrv()
            .leddrvmodesel(led)
            .write_async(|reg| reg.set_leddrvmodesel(mode))
            .await?;
        Ok(())
    }

    /// Configure LED0 mode
    ///
    /// # Arguments
    /// * `mode` - LED mode configuration, see [`LedMode`]
    pub async fn configure_led0_mode(
        &mut self,
        mode: LedMode,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_led_mode(0, mode).await
    }

    /// Configure LED1 mode
    ///
    /// # Arguments
    /// * `mode` - LED mode configuration, see [`LedMode`]
    pub async fn configure_led1_mode(
        &mut self,
        mode: LedMode,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_led_mode(1, mode).await
    }

    /// Configure LED2 mode
    ///
    /// # Arguments
    /// * `mode` - LED mode configuration, see [`LedMode`]
    pub async fn configure_led2_mode(
        &mut self,
        mode: LedMode,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.configure_led_mode(2, mode).await
    }

    /// Enable LED0
    ///
    /// Only works when LED0 is configured in host mode
    pub async fn enable_led0(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .leddrv()
            .leddrvset(0)
            .dispatch_async(|command| command.set_leddrvset(Task::Trigger))
            .await
    }

    /// Disable LED0
    ///
    /// Only works when LED0 is configured in host mode
    pub async fn disable_led0(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .leddrv()
            .leddrvclr(0)
            .dispatch_async(|command| command.set_leddrvclr(Task::Trigger))
            .await
    }

    /// Enable LED1
    ///
    /// Only works when LED1 is configured in host mode
    pub async fn enable_led1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .leddrv()
            .leddrvset(1)
            .dispatch_async(|command| command.set_leddrvset(Task::Trigger))
            .await
    }

    /// Disable LED1
    ///
    /// Only works when LED1 is configured in host mode
    pub async fn disable_led1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .leddrv()
            .leddrvclr(1)
            .dispatch_async(|command| command.set_leddrvclr(Task::Trigger))
            .await
    }

    /// Enable LED2
    ///
    /// Only works when LED2 is configured in host mode
    pub async fn enable_led2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .leddrv()
            .leddrvset(2)
            .dispatch_async(|command| command.set_leddrvset(Task::Trigger))
            .await
    }

    /// Disable LED2
    ///
    /// Only works when LED2 is configured in host mode
    pub async fn disable_led2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .leddrv()
            .leddrvclr(2)
            .dispatch_async(|command| command.set_leddrvclr(Task::Trigger))
            .await
    }
}

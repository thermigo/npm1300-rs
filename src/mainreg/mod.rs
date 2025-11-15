mod types;

// Re-export everything in types.rs
pub use types::*;

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    pub async fn set_vbusin0_event_mask(&mut self, mask: Vbusin0EventMask) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().eventsvbusin_0_set().write_async(|reg| {
            reg.set_eventvbusdetected(mask.contains(Vbusin0EventMask::VBUS_DETECTED));
            reg.set_eventvbusremoved(mask.contains(Vbusin0EventMask::VBUS_REMOVED));
            reg.set_eventvbusovrvoltdetected(mask.contains(Vbusin0EventMask::OVRVOLT_DETECTED));
            reg.set_eventvbusovrvoltremoved(mask.contains(Vbusin0EventMask::OVRVOLT_REMOVED));
            reg.set_eventvbusundervoltdetected(mask.contains(Vbusin0EventMask::UNDERVOLT_DETECTED));
            reg.set_eventvbusundervoltremoved(mask.contains(Vbusin0EventMask::UNDERVOLT_REMOVED));
        }).await
    }

    pub async fn clear_vbusin0_event_mask(&mut self, mask: Vbusin0EventMask) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().eventsvbusin_0_clr().write_async(|reg| {
            reg.set_eventvbusdetected(mask.contains(Vbusin0EventMask::VBUS_DETECTED));
            reg.set_eventvbusremoved(mask.contains(Vbusin0EventMask::VBUS_REMOVED));
            reg.set_eventvbusovrvoltdetected(mask.contains(Vbusin0EventMask::OVRVOLT_DETECTED));
            reg.set_eventvbusovrvoltremoved(mask.contains(Vbusin0EventMask::OVRVOLT_REMOVED));
            reg.set_eventvbusundervoltdetected(mask.contains(Vbusin0EventMask::UNDERVOLT_DETECTED));
            reg.set_eventvbusundervoltremoved(mask.contains(Vbusin0EventMask::UNDERVOLT_REMOVED));
        }).await
    }

    pub async fn enable_vbusin0_interrupts(&mut self, mask: Vbusin0EventMask) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().inteneventsvbusin_0_set().write_async(|reg| {
            reg.set_eventvbusdetected(mask.contains(Vbusin0EventMask::VBUS_DETECTED));
            reg.set_eventvbusremoved(mask.contains(Vbusin0EventMask::VBUS_REMOVED));
            reg.set_eventvbusovrvoltdetected(mask.contains(Vbusin0EventMask::OVRVOLT_DETECTED));
            reg.set_eventvbusovrvoltremoved(mask.contains(Vbusin0EventMask::OVRVOLT_REMOVED));
            reg.set_eventvbusundervoltdetected(mask.contains(Vbusin0EventMask::UNDERVOLT_DETECTED));
            reg.set_eventvbusundervoltremoved(mask.contains(Vbusin0EventMask::UNDERVOLT_REMOVED));
        }).await
    }

    pub async fn disable_vbusin0_interrupts(&mut self, mask: Vbusin0EventMask) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().inteneventsvbusin_0_clr().write_async(|reg| {
            reg.set_eventvbusdetected(mask.contains(Vbusin0EventMask::VBUS_DETECTED));
            reg.set_eventvbusremoved(mask.contains(Vbusin0EventMask::VBUS_REMOVED));
            reg.set_eventvbusovrvoltdetected(mask.contains(Vbusin0EventMask::OVRVOLT_DETECTED));
            reg.set_eventvbusovrvoltremoved(mask.contains(Vbusin0EventMask::OVRVOLT_REMOVED));
            reg.set_eventvbusundervoltdetected(mask.contains(Vbusin0EventMask::UNDERVOLT_DETECTED));
            reg.set_eventvbusundervoltremoved(mask.contains(Vbusin0EventMask::UNDERVOLT_REMOVED));
        }).await
    }

    pub async fn set_adc_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsadcset()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn clear_adc_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsadcclr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn enable_adc_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsadcset()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn disable_adc_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsadcclr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn set_bcharger0_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsbcharger_0_set()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn clear_bcharger0_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsbcharger_0_clr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn enable_bcharger0_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsbcharger_0_set()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn disable_bcharger0_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsbcharger_0_clr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn set_bcharger1_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsbcharger_1_set()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn clear_bcharger1_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsbcharger_1_clr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn enable_bcharger1_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsbcharger_1_set()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn disable_bcharger1_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsbcharger_1_clr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn set_bcharger2_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsbcharger_2_set()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn clear_bcharger2_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsbcharger_2_clr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn enable_bcharger2_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsbcharger_2_set()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn disable_bcharger2_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsbcharger_2_clr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn set_shphld_event(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsshphldset()
            .write_async(|reg| reg.set_eventshphld(1))
            .await
    }

    pub async fn clear_shphld_event(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsshphldclr()
            .write_async(|reg| reg.set_eventshphld(1))
            .await
    }

    pub async fn enable_shphld_interrupt(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsshphldset()
            .write_async(|reg| reg.set_eventshphld(1))
            .await
    }

    pub async fn disable_shphld_interrupt(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsshphldclr()
            .write_async(|reg| reg.set_eventshphld(1))
            .await
    }

        pub async fn set_vbusin1_event_mask(
        &mut self,
        mask: Vbusin1EventMask,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().eventsvbusin_1_set()
            .write_async(|reg| reg.set_value(mask.bits()))
            .await
    }

    pub async fn clear_vbusin1_event_mask(
        &mut self,
        mask: Vbusin1EventMask,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().eventsvbusin_1_clr()
            .write_async(|reg| reg.set_value(mask.bits()))
            .await
    }

    pub async fn enable_vbusin1_interrupts(
        &mut self,
        mask: Vbusin1EventMask,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().inteneventsvbusin_1_set()
            .write_async(|reg| reg.set_value(mask.bits()))
            .await
    }

    pub async fn disable_vbusin1_interrupts(
        &mut self,
        mask: Vbusin1EventMask,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device.main().inteneventsvbusin_1_clr()
            .write_async(|reg| reg.set_value(mask.bits()))
            .await
    }

    pub async fn set_gpio_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsgpioset()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn clear_gpio_event_mask(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .eventsgpioclr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn enable_gpio_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsgpioset()
            .write_async(|reg| reg.set_value(mask))
            .await
    }

    pub async fn disable_gpio_interrupts(
        &mut self,
        mask: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .main()
            .inteneventsgpioclr()
            .write_async(|reg| reg.set_value(mask))
            .await
    }
}

mod types;

// Re-export everything in types.rs
pub use types::*;

use crate::{common::Task, Vbussuspendena};

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Set VBUS input current limit
    ///
    /// This function sets the input current limit and triggers a switch from VBUSINILIMSTARTUP to VBUSINILIM0.
    /// The VBUS current limit reverts to its default value (100 mA) when the following occur:
    ///
    /// * A reset
    /// * The USB cable is unplugged and plugged back in
    ///
    /// # Arguments
    ///
    /// * `current_limit` - The desired input current limit setting
    pub async fn set_vbus_in_current_limit(
        &mut self,
        current_limit: VbusInCurrentLimit,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Set the input current limit
        self.device
            .vbusin()
            .vbusinilim_0()
            .write_async(|reg| reg.set_vbusinilim_0(current_limit))
            .await?;
        // Trigger a switch from VBUSINILIMSTARTUP to VBUSINILIM0
        self.device
            .vbusin()
            .taskupdateilimsw()
            .dispatch_async(|command| command.set_taskupdateilim(Task::Trigger))
            .await
    }

    /// Set VBUS input startup current limit
    ///
    /// # Arguments
    ///
    /// * `current_limit` - The desired input startup current limit setting
    pub async fn set_vbus_in_startup_current_limit(
        &mut self,
        current_limit: VbusInCurrentLimit,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Set the input current limit
        self.device
            .vbusin()
            .vbusinilimstartup()
            .write_async(|reg| reg.set_vbusinilimstartup(current_limit))
            .await
    }

    /// Set VBUS suspend mode
    ///
    /// # Arguments
    ///
    /// * `suspend` - If true, suspends the VBUS input. If false, resumes the VBUS input.
    pub async fn set_vbus_mode(
        &mut self,
        suspend: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .vbusin()
            .vbussuspend()
            .write_async(|reg| {
                reg.set_vbussuspendena(if suspend {
                    Vbussuspendena::Suspend
                } else {
                    Vbussuspendena::Normal
                })
            })
            .await
    }

    /// Get VBUS CC comparator flags status
    ///
    /// # Returns
    ///
    /// * `Ok(VbusDetectStatus)` - The current VBUS detect status containing:
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBUS detect status
    pub async fn get_vbus_cc_status(
        &mut self,
    ) -> Result<VbusCcStatus, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.vbusin().usbcdetectstatus().read_async().await?;
        // Since we successfully read the register, we can safely unwrap the CC1/CC2 status values
        Ok(VbusCcStatus {
            vbusin_cc1_status: status.vbusincc_1_cmp().unwrap(),
            vbusin_cc2_status: status.vbusincc_2_cmp().unwrap(),
        })
    }

    /// Get VBUS input status
    ///
    /// # Returns
    ///
    /// * `Ok(VbusInStatus)` - The current VBUS input status containing:
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBUS input status
    pub async fn get_vbus_in_status(
        &mut self,
    ) -> Result<VbusInStatus, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.vbusin().vbusinstatus().read_async().await?;
        Ok(VbusInStatus {
            is_vbus_in_present: status.vbusinpresent() == 1,
            is_vbus_in_current_limit_active: status.vbusincurrlimactive() == 1,
            is_vbus_out_active: status.vbusinvbusoutactive() == 1,
            is_vbus_undervoltage_detected: status.vbusinundervoltage() == 1,
            is_vbus_in_suspended: status.vbusinsuspendmodeactive() == 1,
            is_vbus_in_overvoltage_protection_active: status.vbusinovrprotactive() == 1,
        })
    }
}

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Vbusin0EventMask: u8 {
        const VBUS_DETECTED          = 1 << 0;
        const VBUS_REMOVED           = 1 << 1;
        const OVRVOLT_DETECTED       = 1 << 2;
        const OVRVOLT_REMOVED        = 1 << 3;
        const UNDERVOLT_DETECTED     = 1 << 4;
        const UNDERVOLT_REMOVED      = 1 << 5;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Vbusin1EventMask: u8 {
        const THERMAL_WARN_DETECTED      = 1 << 0;
        const THERMAL_WARN_REMOVED       = 1 << 1;
        const THERMAL_SHUTDOWN_DETECTED  = 1 << 2;
        const THERMAL_SHUTDOWN_REMOVED   = 1 << 3;
        const CC1_STATE_CHANGE           = 1 << 4;
        const CC2_STATE_CHANGE           = 1 << 5;
    }
}

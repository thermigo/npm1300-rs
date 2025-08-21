/// Battery charger ibat status codes
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IbatStatuscodes {
    IbatStatDischarge = 4,      // 0x04
    IbatStatChargeError = 8,    //0x08 undocumented in driver but occurs at no batt
    IbatStatChargeTrickle = 12, // 0x0C
    IbatStatChargeCool = 13,    // 0x0D
    IbatStatChargeNormal = 15,  // 0x0F
}
// Add conversion from u8
impl From<u8> for IbatStatuscodes {
    fn from(value: u8) -> Self {
        match value {
            4 => Self::IbatStatDischarge,
            8 => Self::IbatStatChargeError,
            12 => Self::IbatStatChargeTrickle,
            13 => Self::IbatStatChargeCool,
            15 => Self::IbatStatChargeNormal,
            _ => panic!("Invalid value"),
        }
    }
}

// Add conversion to u8
impl From<IbatStatuscodes> for u8 {
    fn from(value: IbatStatuscodes) -> Self {
        value as u8
    }
}

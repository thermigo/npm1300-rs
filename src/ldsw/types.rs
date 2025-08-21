/// LDO regulator voltages available on the nPM1300
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum LdswVoltage {
    V1_0 = 0,
    V1_1 = 1,
    V1_2 = 2,
    V1_3 = 3,
    V1_4 = 4,
    V1_5 = 5,
    V1_6 = 6,
    V1_7 = 7,
    V1_8 = 8,
    V1_9 = 9,
    V2_0 = 10,
    V2_1 = 11,
    V2_2 = 12,
    V2_3 = 13,
    V2_4 = 14,
    V2_5 = 15,
    V2_6 = 16,
    V2_7 = 17,
    V2_8 = 18,
    V2_9 = 19,
    V3_0 = 20,
    V3_1 = 21,
    V3_2 = 22,
    V3_3 = 23,
}

// Add conversion from u8 to LdswVoltage
impl TryFrom<u8> for LdswVoltage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::V1_0),
            1 => Ok(Self::V1_1),
            2 => Ok(Self::V1_2),
            3 => Ok(Self::V1_3),
            4 => Ok(Self::V1_4),
            5 => Ok(Self::V1_5),
            6 => Ok(Self::V1_6),
            7 => Ok(Self::V1_7),
            8 => Ok(Self::V1_8),
            9 => Ok(Self::V1_9),
            10 => Ok(Self::V2_0),
            11 => Ok(Self::V2_1),
            12 => Ok(Self::V2_2),
            13 => Ok(Self::V2_3),
            14 => Ok(Self::V2_4),
            15 => Ok(Self::V2_5),
            16 => Ok(Self::V2_6),
            17 => Ok(Self::V2_7),
            18 => Ok(Self::V2_8),
            19 => Ok(Self::V2_9),
            20 => Ok(Self::V3_0),
            21 => Ok(Self::V3_1),
            22 => Ok(Self::V3_2),
            23 => Ok(Self::V3_3),
            _ => Err(()),
        }
    }
}

// Add conversion from LdswVoltage to u8
impl From<LdswVoltage> for u8 {
    fn from(voltage: LdswVoltage) -> Self {
        voltage as u8
    }
}

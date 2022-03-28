//! USB Hub protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubSpeed {
    /// Full Speed Hub.
    FullSpeed,

    /// High Speed Hub with single TT.
    HighSpeedSingle,

    /// High Speed Hub with multiple TT.
    HighSpeedMultiple,
}

impl super::USBProtocol for HubSpeed {}

impl core::convert::TryFrom<u8> for HubSpeed {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<HubSpeed, USBParseError> {
        match byte {
            0x00 => Ok(HubSpeed::FullSpeed),
            0x01 => Ok(HubSpeed::HighSpeedSingle),
            0x02 => Ok(HubSpeed::HighSpeedMultiple),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for HubSpeed {
    fn into(self) -> u8 {
        match self {
            HubSpeed::FullSpeed         => 0x00,
            HubSpeed::HighSpeedSingle   => 0x01,
            HubSpeed::HighSpeedMultiple => 0x02,
        }
    }
}

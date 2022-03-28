//! Smart Card protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmartCardProtocol {
    /// Bulk Transfer with optional interrupt IN.
    Bulk,

    /// Control transfers, no interrupt IN.
    ControlWithoutInterrupt,

    /// Control transfers, optional IN.
    ControlOptionalInterrupt,
}

impl super::USBProtocol for SmartCardProtocol {}

impl core::convert::TryFrom<u8> for SmartCardProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<SmartCardProtocol, USBParseError> {
        match byte {
            0x00 => Ok(SmartCardProtocol::Bulk),
            0x01 => Ok(SmartCardProtocol::ControlWithoutInterrupt),
            0x02 => Ok(SmartCardProtocol::ControlOptionalInterrupt),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for SmartCardProtocol {
    fn into(self) -> u8 {
        match self {
            SmartCardProtocol::Bulk                     => 0x00,
            SmartCardProtocol::ControlWithoutInterrupt  => 0x01,
            SmartCardProtocol::ControlOptionalInterrupt => 0x02,
        }
    }
}

//! USB Test and Mesaurement Device protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBTestAndMeasureProtocol {
    /// Default protocol.
    Default,

    /// USB488 conformant protocol.
    USB488,
}

impl super::USBProtocol for USBTestAndMeasureProtocol {}

impl core::convert::TryFrom<u8> for USBTestAndMeasureProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<USBTestAndMeasureProtocol, USBParseError> {
        match byte {
            0x00 => Ok(USBTestAndMeasureProtocol::Default),
            0x01 => Ok(USBTestAndMeasureProtocol::USB488),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for USBTestAndMeasureProtocol {
    fn into(self) -> u8 {
        match self {
            USBTestAndMeasureProtocol::Default => 0x00,
            USBTestAndMeasureProtocol::USB488  => 0x01,
        }
    }
}

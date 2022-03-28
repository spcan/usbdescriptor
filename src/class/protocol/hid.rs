//! Human Device Interface protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIDProtocol {
    /// Keyboard protocol.
    Keyboard,

    /// Mouse protocol.
    Mouse,
}

impl super::USBProtocol for HIDProtocol {}

impl core::convert::TryFrom<u8> for HIDProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<HIDProtocol, USBParseError> {
        match byte {
            0x01 => Ok(HIDProtocol::Keyboard),
            0x02 => Ok(HIDProtocol::Mouse),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for HIDProtocol {
    fn into(self) -> u8 {
        match self {
            HIDProtocol::Keyboard => 0x01,
            HIDProtocol::Mouse    => 0x02,
        }
    }
}

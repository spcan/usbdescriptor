//! IRDA Bridge Device protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRDABridgeProtocol {
    /// IRDA Bridge Device.
    Default,
}

impl super::USBProtocol for IRDABridgeProtocol {}

impl core::convert::TryFrom<u8> for IRDABridgeProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<IRDABridgeProtocol, USBParseError> {
        match byte {
            0x00 => Ok(IRDABridgeProtocol::Default),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for IRDABridgeProtocol {
    fn into(self) -> u8 {
        0x01
    }
}

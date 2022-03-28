//! Still Imaging protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StillImageProtocol {
    /// Default and only value allowed.
    Default,
}

impl super::USBProtocol for StillImageProtocol {}

impl core::convert::TryFrom<u8> for StillImageProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<StillImageProtocol, USBParseError> {
        match byte {
            0x01 => Ok(StillImageProtocol::Default),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for StillImageProtocol {
    fn into(self) -> u8 {
        0x01
    }
}

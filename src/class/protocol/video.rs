//! Video Device protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoProtocol {
    /// Protocol 15.
    Protocol15,
}

impl super::USBProtocol for VideoProtocol {}

impl core::convert::TryFrom<u8> for VideoProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<VideoProtocol, USBParseError> {
        match byte {
            0x01 => Ok(VideoProtocol::Protocol15),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for VideoProtocol {
    fn into(self) -> u8 {
        0x01
    }
}

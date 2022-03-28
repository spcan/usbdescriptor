//! Wireless base protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirmwareUpgradeProtocol {
    /// Device Firmware Upgrade.
    Default,
}

impl super::USBProtocol for FirmwareUpgradeProtocol {}

impl core::convert::TryFrom<u8> for FirmwareUpgradeProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<FirmwareUpgradeProtocol, USBParseError> {
        match byte {
            0x01 => Ok(FirmwareUpgradeProtocol::Default),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for FirmwareUpgradeProtocol {
    fn into(self) -> u8 {
        0x01
    }
}

//! Mass Storage Transport protocols.



use crate::error::USBParseError;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MassStorageProtocol {
    /// CBI with Command Completion interrupt.
    CBIWithInterrupt,

    /// CBI without Command Completion interrupt.
    CBIWithoutInterrupt,

    /// USB Mass Storage Class Bulk-Only (BBB) Transport.
    BulkOnly,

    /// Allocated for UAS.
    UAS,


    /// Vendor Specific.
    VendorSpecific,
}

impl super::USBProtocol for MassStorageProtocol {}

impl core::convert::TryFrom<u8> for MassStorageProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<MassStorageProtocol, USBParseError> {
        match byte {
            0x00 => Ok(MassStorageProtocol::CBIWithInterrupt),
            0x01 => Ok(MassStorageProtocol::CBIWithoutInterrupt),
            0x50 => Ok(MassStorageProtocol::BulkOnly),
            0x62 => Ok(MassStorageProtocol::UAS),
            0xFF => Ok(MassStorageProtocol::VendorSpecific),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for MassStorageProtocol {
    fn into(self) -> u8 {
        match self {
            MassStorageProtocol::CBIWithInterrupt    => 0x00,
            MassStorageProtocol::CBIWithoutInterrupt => 0x01,
            MassStorageProtocol::BulkOnly            => 0x50,
            MassStorageProtocol::UAS                 => 0x62,
            MassStorageProtocol::VendorSpecific      => 0xFF,
        }
    }
}

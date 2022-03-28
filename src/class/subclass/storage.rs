//! Collection of Mass Storage sub classes.



use crate::error::*;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MassStorageSubClass {
    /// SCSE Command Set not reported.
    NotReported,

    /// RBC.
    RBC,

    /// MMC-5 ATAPI.
    MMC5,

    /// Floppy Disk Drives to USB.
    UFI,

    /// SCSI Transparent Command Set.
    SCSI,

    /// LSD FS.
    LSDFS,

    /// IEEE 1667.
    IEEE1667,

    /// Vendor specific.
    VendorSpecific,
}

impl super::USBSubClass for MassStorageSubClass {}

impl core::convert::TryFrom<u8> for MassStorageSubClass {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<MassStorageSubClass, USBParseError> {
        match byte {
            0x00 => Ok(MassStorageSubClass::NotReported),
            0x01 => Ok(MassStorageSubClass::RBC),
            0x02 => Ok(MassStorageSubClass::MMC5),
            0x04 => Ok(MassStorageSubClass::UFI),
            0x06 => Ok(MassStorageSubClass::SCSI),
            0x07 => Ok(MassStorageSubClass::LSDFS),
            0x08 => Ok(MassStorageSubClass::IEEE1667),
            0xFF => Ok(MassStorageSubClass::VendorSpecific),

            _ => Err( USBParseError::UnknownSubClass(byte) ),
        }
    }
}

impl core::convert::Into<u8> for MassStorageSubClass {
    fn into(self) -> u8 {
        match self {
            MassStorageSubClass::NotReported    => 0x00,
            MassStorageSubClass::RBC            => 0x01,
            MassStorageSubClass::MMC5           => 0x02,
            MassStorageSubClass::UFI            => 0x04,
            MassStorageSubClass::SCSI           => 0x06,
            MassStorageSubClass::LSDFS          => 0x07,
            MassStorageSubClass::IEEE1667       => 0x08,
            MassStorageSubClass::VendorSpecific => 0xFF,
        }
    }
}

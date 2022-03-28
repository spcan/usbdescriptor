//! Printer protocols.



use crate::error::USBParseError;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrinterProtocol {
    /// Unidirectional interface.
    Unidirectional,

    /// Bidirectional interface.
    Bidirectional,

    /// 1284.4 compatible bidirectional interface.
    Bidirectional1284,

    /// Vendor Specific.
    VendorSpecific,
}

impl super::USBProtocol for PrinterProtocol {}

impl core::convert::TryFrom<u8> for PrinterProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<PrinterProtocol, USBParseError> {
        match byte {
            0x01 => Ok(PrinterProtocol::Unidirectional),
            0x02 => Ok(PrinterProtocol::Bidirectional),
            0x03 => Ok(PrinterProtocol::Bidirectional1284),

            0xFF => Ok(PrinterProtocol::VendorSpecific),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for PrinterProtocol {
    fn into(self) -> u8 {
        match self {
            PrinterProtocol::Unidirectional    => 0x01,
            PrinterProtocol::Bidirectional     => 0x02,
            PrinterProtocol::Bidirectional1284 => 0x03,
            PrinterProtocol::VendorSpecific    => 0xFF,
        }
    }
}

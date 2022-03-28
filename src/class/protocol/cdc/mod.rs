//! CDC Control and CDC Data protocols.



use crate::error::*;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDCDataProtocol {
    /// USB specification.
    USB,

    /// Network Transfer Block.
    NTB,

    /// Physical interface protocol for ISDN BRI.
    I430,

    /// ISO/IEC 3309-1993.
    HDLC,

    /// Management protocol for Q.921 data link protocol.
    Q921M,

    /// Data link protocol for Q.931.
    Q921,

    /// TEI-multiplexor for Q.921 data link protocol.
    Q921TM,

    /// V.24 rate adaptation to ISDN.
    V42bis,

    /// Euro-ISDN protocol control.
    Q931,

    /// V.24 adaptation to ISDN.
    V120,

    /// CAPI Commands.
    CAPI2,

    /// Host Based Driver.
    HostBased,

    /// The protocol are described using a Protocol Unit Functional Descriptors.
    Described,

    /// Vendor specific.
    VendorSpecific,
}




#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDCControlProtocol {
    /// USB specification.
    USB,

    /// AT Commands: V.250.
    V250,

    /// AT Commands defined by PCCA-101.
    PCCA101,

    /// AT Commands defined by PCCA-101 and Annex O.
    PCCA101Annex,

    /// AT Commands defined by GSM 07.07.
    GSM,

    /// AT Commands defined by 3GPP 27.007.
    GPP,

    /// AT Commands defined by TIA for CDMA.
    CDMA,

    /// Ethernet Emulation Model.
    EEM,

    /// External Protocol: Commands defined by Command Set Functional Descriptor.
    External,

    /// Vendor specific.
    VendorSpecific,
}

impl super::USBProtocol for CDCControlProtocol {}

impl TryFrom<u8> for CDCControlProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<Self, USBParseError> {
        match byte {
            0x00 => Ok( CDCControlProtocol::USB ),
            0x01 => Ok( CDCControlProtocol::V250 ),
            0x02 => Ok( CDCControlProtocol::PCCA101 ),
            0x03 => Ok( CDCControlProtocol::PCCA101Annex ),
            0x04 => Ok( CDCControlProtocol::GSM ),
            0x05 => Ok( CDCControlProtocol::GPP ),
            0x06 => Ok( CDCControlProtocol::CDMA ),
            0x07 => Ok( CDCControlProtocol::EEM ),

            0xFE => Ok( CDCControlProtocol::External ),

            0xFF => Ok( CDCControlProtocol::VendorSpecific ),
        }
    }
}

impl Into<u8> for CDCControlProtocol {
    fn into(self) -> u8 {
        match self {
            CDCControlProtocol::USB          => 0x00,
            CDCControlProtocol::V250         => 0x01,
            CDCControlProtocol::PCCA101      => 0x02,
            CDCControlProtocol::PCCA101Annex => 0x03,
            CDCControlProtocol::GSM          => 0x04,
            CDCControlProtocol::GPP          => 0x05,
            CDCControlProtocol::CDMA         => 0x06,
            CDCControlProtocol::EEM          => 0x07,

            CDCControlProtocol::External => 0xFE,

            CDCControlProtocol::VendorSpecific => 0xFF,
        }
    }
}

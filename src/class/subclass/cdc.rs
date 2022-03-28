//! CDC Control and CDC Data sub classes.



use crate::error::*;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDCControlSubClass {
    /// Direct Line Control Model.
    DirectLine,

    /// Abstract Contol Model.
    Abstract,

    /// Telephone Control Model.
    Telephone,

    /// Multi-Channel Control Model.
    MultiChannel,

    /// CAPI Control Mode.
    CAPI,

    /// Ethernet Networking Control Model.
    EthernetNetworking,

    /// ATM Networking Control Model.
    ATMNetworking,

    /// Wireless Handset Control Model.
    WirelessHandset,

    /// Device Management.
    DeviceManagement,

    /// Mobile Direct Line Model.
    MobileDirectLine,

    /// OBEX.
    OBEX,

    /// Ethernet Emulation Model.
    EthernetEmulation,

    /// Network Control Model.
    NetworkControl,

    /// Vendor specific.
    VendorSpecific,
}

impl super::USBSubClass for CDCControlSubClass {}

impl TryFrom<u8> for CDCControlSubClass {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<Self, USBParseError> {
        match byte {
            0x01 => Ok( CDCControlSubClass::DirectLine ),
            0x02 => Ok( CDCControlSubClass::Abstract ),
            0x03 => Ok( CDCControlSubClass::Telephone ),
            0x04 => Ok( CDCControlSubClass::MultiChannel ),
            0x05 => Ok( CDCControlSubClass::CAPI ),
            0x06 => Ok( CDCControlSubClass::EthernetNetworking ),
            0x07 => Ok( CDCControlSubClass::ATMNetworking ),
            0x08 => Ok( CDCControlSubClass::WirelessHandset ),
            0x09 => Ok( CDCControlSubClass::DeviceManagement ),
            0x0A => Ok( CDCControlSubClass::MobileDirectLine ),
            0x0B => Ok( CDCControlSubClass::OBEX ),
            0x0C => Ok( CDCControlSubClass::EthernetEmulation ),
            0x0D => Ok( CDCControlSubClass::NetworkControl ),

            0x80..=0xFF => Ok( CDCControlSubClass::VendorSpecific ),

            s => Err( USBParseError::UnknownSubClass(s) ),
        }
    }
}

impl Into<u8> for CDCControlSubClass {
    fn into(self) -> u8 {
        match self {
            CDCControlSubClass::DirectLine         => 0x01,
            CDCControlSubClass::Abstract           => 0x02,
            CDCControlSubClass::Telephone          => 0x03,
            CDCControlSubClass::MultiChannel       => 0x04,
            CDCControlSubClass::CAPI               => 0x05,
            CDCControlSubClass::EthernetNetworking => 0x06,
            CDCControlSubClass::ATMNetworking      => 0x07,
            CDCControlSubClass::WirelessHandset    => 0x08,
            CDCControlSubClass::DeviceManagement   => 0x09,
            CDCControlSubClass::MobileDirectLine   => 0x0A,
            CDCControlSubClass::OBEX               => 0x0B,
            CDCControlSubClass::EthernetEmulation  => 0x0C,
            CDCControlSubClass::NetworkControl     => 0x0D,

            CDCControlSubClass::VendorSpecific => 0xFF,
        }
    }
}

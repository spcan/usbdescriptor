//! Wireless base protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WirelessAdapterProtocol {
    /// Host Wire Adapter Control / Data interface.
    HostWireAdapterCDI,

    /// Device Wire Adapter Control / Data interface.
    DeviceWireAdapterCDI,

    /// Device Wire Adapter Isochronous interface.
    DeviceWireAdapterII,
}

impl super::USBProtocol for WirelessAdapterProtocol {}

impl core::convert::TryFrom<u8> for WirelessAdapterProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<WirelessAdapterProtocol, USBParseError> {
        match byte {
            0x01 => Ok(WirelessAdapterProtocol::HostWireAdapterCDI),
            0x02 => Ok(WirelessAdapterProtocol::DeviceWireAdapterCDI),
            0x03 => Ok(WirelessAdapterProtocol::DeviceWireAdapterII),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for WirelessAdapterProtocol {
    fn into(self) -> u8 {
        match self {
            WirelessAdapterProtocol::HostWireAdapterCDI   => 0x01,
            WirelessAdapterProtocol::DeviceWireAdapterCDI => 0x02,
            WirelessAdapterProtocol::DeviceWireAdapterII  => 0x03,
        }
    }
}

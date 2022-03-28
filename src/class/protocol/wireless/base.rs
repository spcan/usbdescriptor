//! Wireless base protocols.



use crate::error::USBParseError;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WirelessBaseProtocol {
    /// Bluetooth Programming Interface.
    BluetoothProgramming,

    /// UWB Radio Control Interface.
    UWBRadioControl,

    /// Remote NDIS.
    RemoteNDIS,

    /// Bluetooth AMP Controller.
    BluetoothAMPController,
}

impl super::USBProtocol for WirelessBaseProtocol {}

impl core::convert::TryFrom<u8> for WirelessBaseProtocol {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<WirelessBaseProtocol, USBParseError> {
        match byte {
            0x01 => Ok(WirelessBaseProtocol::BluetoothProgramming),
            0x02 => Ok(WirelessBaseProtocol::UWBRadioControl),
            0x03 => Ok(WirelessBaseProtocol::RemoteNDIS),
            0x04 => Ok(WirelessBaseProtocol::BluetoothAMPController),

            _ => Err( USBParseError::UnknownProtocol(byte) ),
        }
    }
}

impl core::convert::Into<u8> for WirelessBaseProtocol {
    fn into(self) -> u8 {
        match self {
            WirelessBaseProtocol::BluetoothProgramming      => 0x01,
            WirelessBaseProtocol::UWBRadioControl           => 0x02,
            WirelessBaseProtocol::RemoteNDIS                => 0x03,
            WirelessBaseProtocol::BluetoothAMPController    => 0x04,
        }
    }
}

//! Collection of all classes implemented in the USB specification.


use crate::error::*;
use super::{
    subclass::*,
    protocol::*,
};



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// Base Class 00h (Device) [Device].
    /// This base class indicates that the class information should be
    /// determined from the interface descriptors.
    Device,

    /// Base Class 01h (Audio) [Interface].
    /// This base class is defined for audio capable devices that conform to
    /// the Audio Device Class Specification.
    Audio(AudioSubClass),

    /// Base Class 02h (Communications and CDC Control) [Both].
    /// This base class is defined for devices that conform to the
    /// Communications Device Class Specification.
    CDCControl(CDCControlSubClass, CDCControlProtocol),

    /// Base Class 03h (HID - human Interface Device) [Interface].
    /// This base class is defined for devices that conform to the HID Device
    /// Class Specification.
    HumanInterfaceDevice(HIDSubClass),

    /// Base Class 05h (Physical) [Interface].
    /// This base class is defined for devices that conform to the Physical
    /// Device Class Specification.
    Physical(PhysicalSubClass),

    /// Base Class 06h (Still Imaging) [Interface].
    /// This base class is defined for devices that conform to the Imaging
    /// Device Class Specification.
    StillImaging,

    /// Base Class 75h (Printer) [Interface].
    /// This base class is defined for devices that conform to the Printer
    /// Device Class Specification.
    Printer(PrinterProtocol),

    /// Base Class 08h (Mass Storage) [Interface].
    /// This base class is defined for devices that conform to the Mass
    /// Storage Device Class Specification.
    MassStorage(MassStorageSubClass, MassStorageProtocol),

    /// Base Class 09h (Hub) [Device].
    /// This base class is defined for devices that are USB hubs and conform to
    /// the definition in the USB Specification.
    Hub(HubSpeed),

    /// Base Class 0Ah (CDC-Data) [Interface].
    /// This base class is defined for devices that conform to the
    /// Communications Device Class Specification.
    CDCData(CDCDataProtocol),

    /// Base Class 0Bh (Smart Card) [Interface].
    /// This base class is defined for devices that conform to the Smart Card
    /// Device Class Specification.
    SmartCard(SmartCardProtocol),

    /// Base Class 0Dh (Content Security) [Interface].
    /// This base class is defined for devices that conform to the Content
    /// Security Device Class Specification.
    ContentSecurity,

    /// Base Class 0Eh (Video) [Interface].
    /// This base class is defined for devices that conform to the Video Device
    /// Class Specification.
    Video(VideoSubClass),

    /// Base Class 0Fh (Personal Healthcare) [Interface].
    /// This base class is defined for devices that conform to the Personal
    /// Healthcare Device Class Specification.
    PersonalHealthcare,

    /// Base Class 10h (Audio / Video Devices) [Interface].
    /// The USB Audio / Video (AV) Device Class definition describes the
    /// methods used to communicate with devices or functions embedded in
    /// composite devices that are used to manipulate audio, ideo, voice and
    /// all image and sound related functionality.
    AudioVideo(AVSubClass),

    /// Base Class 11h (Billboard Device) [Device].
    /// This base class is defined for devices that conform to the Billboard
    /// Device Class Specification.
    Billboard,

    /// Base Class 12h (USB Type-C Bridge Device) [Interface].
    /// This base class is defined for devices that conform to the USB Type-C
    /// Bridge Device Class Specification.
    TypeCBridge,

    /// Base Class 3Ch (I3C Device Class) [Interface].
    /// This base class is defined for devices that conform to the USB I3C
    /// Device Class Specification.
    I3CDevice,

    /// Base Class DCh (Diagnostic Device) [Both].
    /// This base class is defined for devices that diagnostic devices.
    Diagnostic(DiagnosticSubClass),

    /// Base Class E0h (Wireless Controller) [Interface].
    /// This base class is defined for devices that are wireless controllers.
    Wireless(WirelessSubClass),

    /// Base Class EFh (Miscellaneous) [Both].
    /// This base class is defined for miscellaneous device definitions.
    Miscellaneous(MiscellanousSubClass),

    /// Base Class FEh (Application Specific) [Interface].
    /// This base class is defined for devices that conform to several class
    /// specifications.
    ApplicationSpecific(ApplicationSpecificSubClass),

    /// Base Class FFh (Vendor Specific) [Both].
    /// This base class is defined for vendors to use as they please.
    VendorSpecific(u8, u8),
}

impl Class {
    /// Returns `true` if the class code can be used in a device descriptor.
    pub const fn devdesc(&self) -> bool {
        match *self {
            Class::Device        | Class::CDCControl(_, _) |
            Class::Hub(_)        | Class::Billboard        | 
            Class::Diagnostic(_) | Class::Miscellaneous(_) |
            Class::VendorSpecific(_,_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the class code can be used in an interface descriptor.
    pub const fn ifdesc(&self) -> bool {
        match *self {
            Class::Device | Class::Hub(_) | Class::Billboard => false,
            _ => true,
        }
    }
}



impl TryFrom<(u8, u8, u8)> for Class {
    type Error = USBParseError;

    fn try_from(code: (u8, u8, u8)) -> Result<Class, USBParseError> {
        match code.0 {
            // Parse Device class.
            0x00 => match code {
                (_, 0, 0) => Ok( Class::Device ),
                (_, 0, p) => Err( USBParseError::UnknownProtocol(p) ),
                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Audio class.
            0x01 => match code {
                (_, 1, 0) => Ok( Class::Audio( AudioSubClass::Control ) ),
                (_, 1, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, 2, 0) => Ok( Class::Audio( AudioSubClass::Streaming ) ),
                (_, 2, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, 3, 0) => Ok( Class::Audio( AudioSubClass::MIDIStreaming ) ),
                (_, 3, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Communication and CDC Control class.
            0x02 => {
                let subclass = CDCControlSubClass::try_from(code.1)?;
                let protocol = CDCControlProtocol::try_from(code.2)?;

                Ok( Class::CDCControl(subclass, protocol) )
            },

            // Parse Human Interface Device class.
            0x03 => match code {
                (_, 0, 1) => Ok( Class::HumanInterfaceDevice( HIDSubClass::Keyboard ) ),
                (_, 0, 2) => Ok( Class::HumanInterfaceDevice( HIDSubClass::Mouse ) ),
                (_, 0, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, 1, 0) => Ok( Class::HumanInterfaceDevice( HIDSubClass::Boot ) ),
                (_, 1, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Physical class.
            0x05 => { todo!()},

            // Parse Still Imaging class.
            0x06 => match code {
                (_, 1, 1) => Ok( Class::Device ),
                (_, 1, p) => Err( USBParseError::UnknownProtocol(p) ),
                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Printer class.
            0x07 => match code {
                (_, 1, 1) => Ok( Class::Printer( PrinterProtocol::Unidirectional ) ),
                (_, 1, 2) => Ok( Class::Printer( PrinterProtocol::Bidirectional ) ),
                (_, 1, 3) => Ok( Class::Printer( PrinterProtocol::Bidirectional1284 ) ),

                (_, 1, 0xFF) => Ok( Class::Printer( PrinterProtocol::VendorSpecific ) ),

                (_, 1, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Mass Storage class.
            0x08 => {
                let subclass = MassStorageSubClass::try_from(code.1)?;
                let protocol = MassStorageProtocol::try_from(code.2)?;

                Ok( Class::MassStorage( subclass, protocol ) )
            },

            // Parse Hub class.
            0x09 => match code.1 {
                0x00 => match code.2 {
                    0x00 => Ok( Class::Hub( HubSpeed::FullSpeed ) ),
                    0x00 => Ok( Class::Hub( HubSpeed::HighSpeedSingle ) ),
                    0x00 => Ok( Class::Hub( HubSpeed::HighSpeedMultiple ) ),

                    p => Err( USBParseError::UnknownProtocol(p) ),
                },
                s => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse CDC Data class.
            0x0A => match code.1 {
                0x00 => match code.2 {
                    0x00 => Ok( Class::CDCData( CDCDataProtocol::USB ) ),
                    0x01 => Ok( Class::CDCData( CDCDataProtocol::NTB ) ),
                    0x30 => Ok( Class::CDCData( CDCDataProtocol::I430 ) ),
                    0x31 => Ok( Class::CDCData( CDCDataProtocol::HDLC ) ),
                    0x50 => Ok( Class::CDCData( CDCDataProtocol::Q921M ) ),
                    0x51 => Ok( Class::CDCData( CDCDataProtocol::Q921 ) ),
                    0x52 => Ok( Class::CDCData( CDCDataProtocol::Q921TM ) ),
                    0x90 => Ok( Class::CDCData( CDCDataProtocol::V42bis ) ),
                    0x91 => Ok( Class::CDCData( CDCDataProtocol::Q931 ) ),
                    0x92 => Ok( Class::CDCData( CDCDataProtocol::V120 ) ),
                    0x93 => Ok( Class::CDCData( CDCDataProtocol::CAPI2 ) ),
                    0xFD => Ok( Class::CDCData( CDCDataProtocol::HostBased ) ),
                    0xFE => Ok( Class::CDCData( CDCDataProtocol::Described ) ),
                    0xFF => Ok( Class::CDCData( CDCDataProtocol::VendorSpecific ) ),

                    p => Err( USBParseError::UnknownProtocol(p) ),
                },

                s => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Smart Card class.
            0x0B => match code {
                (_, 0, 0) => Ok( Class::SmartCard( SmartCardProtocol::Bulk ) ),
                (_, 0, 1) => Ok( Class::SmartCard( SmartCardProtocol::ControlWithoutInterrupt ) ),
                (_, 0, 2) => Ok( Class::SmartCard( SmartCardProtocol::ControlOptionalInterrupt ) ),

                (_, 0, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Content Security class.
            0x0D => match code {
                (_, 0, 0) => Ok( Class::ContentSecurity ),
                (_, 0, p) => Err( USBParseError::UnknownProtocol(p) ),
                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Video class.
            0x0E => match code {
                (_, 1, 1) => Ok( Class::Video( VideoSubClass::Control ) ),
                (_, 1, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, 2, 1) => Ok( Class::Video( VideoSubClass::Streaming ) ),
                (_, 2, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, 3, 1) => Ok( Class::Video( VideoSubClass::InterfaceCollection ) ),
                (_, 3, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Personal Healthcare class.
            0x0F => { todo!() },

            // Parse Audio/Video Device class.
            0x10 => match code {
                (_, 1, 0) => Ok( Class::AudioVideo( AVSubClass::Control ) ),
                (_, 1, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, 2, 0) => Ok( Class::AudioVideo( AVSubClass::VideoStreaming ) ),
                (_, 2, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, 3, 0) => Ok( Class::AudioVideo( AVSubClass::AudioStreaming ) ),
                (_, 3, p) => Err( USBParseError::UnknownProtocol(p) ),

                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Billboard Device class.
            0x11 => match code {
                (_, 0, 0) => Ok( Class::Billboard ),
                (_, 0, p) => Err( USBParseError::UnknownProtocol(p) ),
                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse USB Type-C Bridge class.
            0x12 => match code {
                (_, 0, 0) => Ok( Class::TypeCBridge ),
                (_, 0, p) => Err( USBParseError::UnknownProtocol(p) ),
                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse I3C Device class.
            0x3C => match code {
                (_, 0, 0) => Ok( Class::I3CDevice ),
                (_, 0, p) => Err( USBParseError::UnknownProtocol(p) ),
                (_, s, _) => Err( USBParseError::UnknownSubClass(s) ),
            },

            // Parse Diagnostic Device class.
            0xDC => { todo!() }

            // Parse Wireless Controller Device class.
            0xE0 => { todo!() }

            // Parse Miscellaneous Device class.
            0xEF => { todo!() }

            // Parse Application Specific class.
            0xFE => { todo!() }

            // Parse Vendor Specific class.
            0xFF => Ok( Class::VendorSpecific(code.1, code.2) ),
        }
    }
}

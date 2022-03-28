//! Collection of Audio and Video sub classes.



use crate::error::*;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVSubClass {
    /// AV Control Interface.
    Control,

    /// AV Video Streaming Interface.
    VideoStreaming,

    /// AV Audio Streaming Interface.
    AudioStreaming,
}

impl super::USBSubClass for AVSubClass {}

impl core::convert::TryFrom<u8> for AVSubClass {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<AVSubClass, USBParseError> {
        match byte {
            0x01 => Ok(AVSubClass::Control),
            0x02 => Ok(AVSubClass::VideoStreaming),
            0x03 => Ok(AVSubClass::AudioStreaming),

            _ => Err( USBParseError::UnknownSubClass(byte) ),
        }
    }
}

impl core::convert::Into<u8> for AVSubClass {
    fn into(self) -> u8 {
        match self {
            AVSubClass::Control        => 0x01,
            AVSubClass::VideoStreaming => 0x02,
            AVSubClass::AudioStreaming => 0x03,
        }
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSubClass {
    /// Video Control Interface.
    Control,

    /// Audio Streaming Interface.
    Streaming,

    /// MIDI Streaming Interface.
    MIDIStreaming,
}

impl super::USBSubClass for AudioSubClass {}

impl core::convert::TryFrom<u8> for AudioSubClass {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<AudioSubClass, USBParseError> {
        match byte {
            0x01 => Ok(AudioSubClass::Control),
            0x02 => Ok(AudioSubClass::Streaming),
            0x03 => Ok(AudioSubClass::MIDIStreaming),

            _ => Err( USBParseError::UnknownSubClass(byte) ),
        }
    }
}

impl core::convert::Into<u8> for AudioSubClass {
    fn into(self) -> u8 {
        match self {
            AudioSubClass::Control       => 0x01,
            AudioSubClass::Streaming     => 0x02,
            AudioSubClass::MIDIStreaming => 0x03,
        }
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoSubClass {
    /// Video Control Interface.
    Control,

    /// Video Streaming Interface.
    Streaming,

    /// Video Interface Collection.
    InterfaceCollection,
}

impl super::USBSubClass for VideoSubClass {}

impl core::convert::TryFrom<u8> for VideoSubClass {
    type Error = USBParseError;

    fn try_from(byte: u8) -> Result<VideoSubClass, USBParseError> {
        match byte {
            0x01 => Ok(VideoSubClass::Control),
            0x02 => Ok(VideoSubClass::Streaming),
            0x03 => Ok(VideoSubClass::InterfaceCollection),

            _ => Err( USBParseError::UnknownSubClass(byte) ),
        }
    }
}

impl core::convert::Into<u8> for VideoSubClass {
    fn into(self) -> u8 {
        match self {
            VideoSubClass::Control             => 0x01,
            VideoSubClass::Streaming           => 0x02,
            VideoSubClass::InterfaceCollection => 0x03,
        }
    }
}

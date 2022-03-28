//! USB Parse errors.




#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBParseError {
    /// Unknown protocol code.
    UnknownProtocol(u8),

    /// Unknown sub class code.
    UnknownSubClass(u8),

    /// Unknown class code.
    UnknownClass(u8),
}

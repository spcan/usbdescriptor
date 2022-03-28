//! Human Device Interface subclasses.



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIDSubClass {
    /// Boot sub class.
    Boot,

    /// Keyboard protocol.
    Keyboard,

    /// Mouse protocol.
    Mouse,
}

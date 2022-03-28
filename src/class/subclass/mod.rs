//! Collection of all subclasses implemented in the USB specification.




mod av;
mod cdc;
mod hid;
mod storage;



pub use av::*;
pub use cdc::*;
pub use hid::*;
pub use storage::*;



/// Common trait for all protocols.
pub trait USBSubClass: TryFrom<u8> + Into<u8> {}

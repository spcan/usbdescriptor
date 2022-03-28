//! Collection of all protocols implemented in the USB specification.



mod app;
mod cdc;
mod hid;
mod hub;
mod printer;
mod smartcard;
mod stillimage;
mod storage;
mod video;
mod wireless;



pub use self::app::*;
pub use self::cdc::*;
pub use self::hid::*;
pub use self::hub::*;
pub use self::printer::*;
pub use self::smartcard::*;
pub use self::stillimage::*;
pub use self::storage::*;
pub use self::video::*;
pub use self::wireless::*;



/// Common trait for all protocols.
pub trait USBProtocol: TryFrom<u8> + Into<u8> {}

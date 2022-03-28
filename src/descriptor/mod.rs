//! USB Device descriptor.
//! A structure representing the USB device descriptor.
//! Documentation: Section 9.6.1 of the USB 3.0 specification.





pub struct DeviceDescriptor {
    /// Size of this descriptor in bytes.
    bLength: u8,

    /// Descriptor type.
    /// Its type is enumerated in `DescriptorType`.
    bDescriptorType: u8,

    /// USB Specification release number in binary-coded decimal.
    /// Its type is abstracted in `USBVersion`.
    bcdUSB: u16,

    /// USB-IF class code, subclass code and protocol code.
    /// Its type is enumerated in `ClassCode`.
    class: u8,
    subclass: u8,
    protocol: u8,

    /// Maximum packet size for endpoint 0.
    bMaxPacketSize0: u8,

    /// Vendor ID.
    idVendor: u16,

    /// Product ID.
    idProduct: u16,

    /// Device release number in binary coded decimal.
    bcdDevice: u16,

    /// Index of string descriptor describing manufacturer.
    iManufacturer: u8,

    /// Index of string descriptor describing product.
    iProduct: u8,

    /// Index of string descriptor containing device serial number.
    iSerialNumber: u8,

    /// number of possible configuration.
    bNumcConfigurations: u8,
}


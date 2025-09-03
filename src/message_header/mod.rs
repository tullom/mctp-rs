mod mctp_control_message_header;
mod mctp_vendor_defined_pci_message_header;

use crate::mctp_message_type::MctpMessageType;
use bit_register::bit_register;
pub use mctp_control_message_header::MctpControlMessageHeader;
pub use mctp_vendor_defined_pci_message_header::MctpVendorDefinedPciMessageHeader;

bit_register! {
    /// Generic message header for all MCTP messages. Based off of message_type, the header
    /// can be interpreted as a more specific header type, such as MctpControlMessageHeader
    #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
    pub struct MctpMessageHeader: little_endian u32 {
        pub integrity_check: u8 => [31],
        pub message_type: MctpMessageType => [24:30],
        pub rest: u32 => [0:23],
    }
}

use crate::{MctpMessageHeader, mctp_message_type::MctpMessageType};
use bit_register::bit_register;

bit_register! {
    #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
    pub struct MctpVendorDefinedPciMessageHeader: little_endian u32 {
        pub integrity_check: u8 => [31],
        pub message_type: MctpMessageType => [24:30],
        pub pci_vendor_id: u16 => [8:23],
    }
}

impl From<MctpVendorDefinedPciMessageHeader> for MctpMessageHeader {
    fn from(header: MctpVendorDefinedPciMessageHeader) -> Self {
        let as_u32: u32 = header.try_into().unwrap();
        MctpMessageHeader::try_from(as_u32).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mctp_vendor_defined_pci_message_header_bit_register() {
        let as_be_bytes = [
            0x7E, // vendor defined pci message type
            0x12, 0x34, // pci vendor id
            0x00, // unused
        ];
        let as_struct = MctpVendorDefinedPciMessageHeader {
            integrity_check: 0,
            message_type: MctpMessageType::VendorDefinedPci,
            pci_vendor_id: 0x1234,
        };

        assert_eq!(
            MctpVendorDefinedPciMessageHeader::try_from(u32::from_be_bytes(as_be_bytes)).unwrap(),
            as_struct
        );

        assert_eq!(
            TryInto::<u32>::try_into(as_struct).unwrap().to_be_bytes(),
            as_be_bytes
        );
    }
}

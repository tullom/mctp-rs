use crate::mctp_command_code::MctpCommandCode;
use crate::mctp_completion_code::MctpCompletionCode;
use crate::mctp_message_type::MctpMessageType;
use bit_register::bit_register;

bit_register! {
    /// if message_type is MctpMessageType::MctpControl, then the header is a MctpControlMessageHeader
    /// with message specific fields.
    #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
    pub struct MctpControlMessageHeader: little_endian u32 {
        pub integrity_check: u8 => [31],
        pub message_type: MctpMessageType => [24:30],
        pub request_bit: u8 => [23],
        pub datagram_bit: u8 => [22],
        pub reserved: u8 => [21],
        pub instance_id: u8 => [16:20],
        pub command_code: MctpCommandCode => [8:15],
        pub completion_code: MctpCompletionCode => [0:7],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mctp_control_message_header_bit_register() {
        assert_eq!(
            MctpControlMessageHeader::try_from(u32::from_be_bytes([
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]))
            .unwrap(),
            MctpControlMessageHeader {
                integrity_check: 0,
                ..Default::default()
            }
        );
        assert_eq!(
            MctpControlMessageHeader::try_from(u32::from_be_bytes([
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]))
            .unwrap(),
            MctpControlMessageHeader {
                integrity_check: 1,
                ..Default::default()
            }
        );
    }
}

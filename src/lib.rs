#![no_std]
#![allow(dead_code)]

mod control_command;
mod deserialize;
mod endpoint_id;
mod error;
mod mctp_command_code;
mod mctp_completion_code;
mod mctp_control_message_header;
mod mctp_message_tag;
mod mctp_message_type;
mod mctp_packet_context;
mod mctp_sequence_number;
mod mctp_transport_header;
mod medium;
mod serialize;
#[cfg(test)]
mod test_util;
use bit_register::bit_register;
use endpoint_id::EndpointId;

pub use crate::error::MctpPacketError;
use crate::mctp_control_message_header::MctpControlMessageHeader;
use crate::mctp_message_tag::MctpMessageTag;
use crate::mctp_message_type::MctpMessageType;
pub use crate::mctp_packet_context::MctpPacketContext;
use crate::mctp_packet_context::MctpReplyContext;
use crate::mctp_sequence_number::MctpSequenceNumber;
use crate::medium::MctpMedium;

#[cfg(test)]
mod tests_1 {
    extern crate std;
    use crate::{mctp_message_tag::MctpMessageTag, mctp_transport_header::MctpTransportHeader};

    #[test]
    fn test_mctp_transport_header_bit_register() {
        assert_eq!(
            MctpTransportHeader::try_from(u32::from_be_bytes([
                0b1111_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]))
            .unwrap(),
            MctpTransportHeader {
                reserved: 0b1111,
                ..Default::default()
            }
        );
        assert_eq!(
            MctpTransportHeader::try_from(u32::from_be_bytes([
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0011,
            ]))
            .unwrap(),
            MctpTransportHeader {
                message_tag: MctpMessageTag::try_from(3).unwrap(),
                ..Default::default()
            }
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MctpMessage<'buffer, M: MctpMedium> {
    pub reply_context: MctpReplyContext<M>,
    pub header_and_body: MctpMessageHeaderAndBody<'buffer>,
    pub message_integrity_check: Option<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MctpMessageHeaderAndBody<'buffer> {
    Control {
        header: MctpControlMessageHeader,
        body: &'buffer [u8],
    },
    VendorDefinedPci {
        header: MctpMessageHeader,
        body: &'buffer [u8],
    },
    VendorDefinedIana {
        header: MctpMessageHeader,
        body: &'buffer [u8],
    },
}

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

/// The body of the message
pub enum MctpMessageData {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AssemblyState {
    Idle,
    Receiving(AssemblingState),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct AssemblingState {
    message_tag: MctpMessageTag,
    tag_owner: u8,
    source_endpoint_id: EndpointId,
    packet_sequence_number: MctpSequenceNumber,
    packet_assembly_buffer_index: usize,
}

#[cfg(test)]
mod mctp_context_tests {
    use super::*;
    use crate::{
        error::ProtocolError, mctp_command_code::MctpCommandCode,
        mctp_packet_context::MctpPacketContext, test_util::*,
    };
    use pretty_assertions::assert_eq;

    struct Packet(&'static [u8]);
    const GET_ENDPOINT_ID_PACKET_NO_EOM: Packet = Packet(&[
        // test medium frame (header + trailer): 0 bytes
        // transport header:
        0b0000_0001, // mctp reserved, header version
        0b0000_1001, // destination endpoint id (9)
        0b0001_0110, // source endpoint id (22)
        0b1000_0011, // som, eom, seq (0), to, tag (3)
        // message header:
        0b0000_0000, // integrity check (off) / message type (mctp control message)
        0b0000_0000, // rq, d, rsvd, instance id
        0b0000_0010, // command code (2: get endpoint id)
        0b0000_0000, // completion code
        // message body:
        0b0000_1111, // endpoint id (15)
        0b0000_0001, // endpoint type (simple = 0b00) / endpoint id type (static eid supported = 0b01)
        0b1111_0000, // medium specific
    ]);

    const EMPTY_PACKET_EOM: Packet = Packet(&[
        // transport header:
        0b0000_0001, // mctp reserved, header version
        0b0000_1001, // destination endpoint id (9)
        0b0001_0110, // source endpoint id (14)
        0b0101_0011, // som, eom, seq (1), to, tag (3)
    ]);

    #[test]
    fn split_over_two_packets() {
        let mut buffer = [0; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut buffer);

        assert_eq!(
            context
                .deserialize_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0)
                .unwrap(),
            None
        );

        assert_eq!(
            context
                .deserialize_packet(EMPTY_PACKET_EOM.0)
                .unwrap()
                .unwrap(),
            MctpMessage {
                reply_context: MctpReplyContext {
                    destination_endpoint_id: EndpointId::Id(9),
                    source_endpoint_id: EndpointId::Id(22),
                    packet_sequence_number: MctpSequenceNumber::new(1),
                    message_tag: MctpMessageTag::try_from(3).unwrap(),
                    medium_context: (),
                },
                header_and_body: MctpMessageHeaderAndBody::Control {
                    header: MctpControlMessageHeader {
                        integrity_check: 0,
                        message_type: MctpMessageType::MctpControl,
                        command_code: MctpCommandCode::GetEndpointId,
                        ..Default::default()
                    },
                    body: &[
                        0b0000_1111, // endpoint id (15)
                        0b0000_0001, // endpoint type (simple = 0b00) / endpoint id type (static eid supported = 0b01)
                        0b1111_0000, // medium specific
                    ],
                },
                message_integrity_check: None,
            }
        );
    }

    #[test]
    fn lacking_start_of_message() {
        let mut buffer = [0; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut buffer);

        assert_eq!(
            context.deserialize_packet(&[
                // transport header:
                0b0000_0000, // mctp reserved, header version
                0b0000_0000, // destination endpoint id
                0b0000_0000, // source endpoint id
                0b0000_0000, // som, eom, seq (0), to, tag
            ]),
            Err(MctpPacketError::ProtocolError(
                ProtocolError::ExpectedStartOfMessage,
            ))
        );
    }

    #[test]
    fn repeated_start_of_message() {
        let mut buffer = [0; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut buffer);

        context
            .deserialize_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0)
            .unwrap();

        assert_eq!(
            context.deserialize_packet(&[
                // transport header:
                0b0000_0000, // mctp reserved, header version
                0b0000_0000, // destination endpoint id
                0b0000_0000, // source endpoint id
                0b1000_0000, // som, eom, seq (0), to, tag
            ]),
            Err(MctpPacketError::ProtocolError(
                ProtocolError::UnexpectedStartOfMessage,
            ))
        );
    }

    #[test]
    fn message_tag_mismatch() {
        let mut buffer = [0; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut buffer);

        // message tag = 0
        context
            .deserialize_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0)
            .unwrap();

        // message tag = 1
        assert_eq!(
            context.deserialize_packet(&[
                // transport header:
                0b0000_0000, // mctp reserved, header version
                0b0000_0000, // destination endpoint id
                0b0000_0000, // source endpoint id
                0b0101_0010, // som, eom, seq (1), to, tag (2)
            ]),
            Err(MctpPacketError::ProtocolError(
                ProtocolError::MessageTagMismatch(
                    MctpMessageTag::try_from(3).unwrap(),
                    MctpMessageTag::try_from(2).unwrap(),
                ),
            ))
        );
    }

    #[test]
    fn test_send_packet() {
        let mut buffer = [0; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(
            TestMedium::new().with_headers(&[0xA, 0xB], &[0xC, 0xD]),
            &mut buffer,
        );

        let reply_context = MctpReplyContext {
            destination_endpoint_id: EndpointId::try_from(236).unwrap(),
            source_endpoint_id: EndpointId::try_from(192).unwrap(),
            packet_sequence_number: MctpSequenceNumber::new(1),
            message_tag: MctpMessageTag::try_from(3).unwrap(),
            medium_context: (),
        };

        let mut state = context.serialize_packet(reply_context, &[0xA5]).unwrap();

        let packet = state.next().unwrap().unwrap();
        assert_eq!(
            packet,
            &[
                // test header - 2 bytes
                0xA,
                0xB,
                // mctp transport header
                0b0000_0001, // mctp reserved, header version
                192,         // destination endpoint id
                236,         // source endpoint id
                0b1110_0011, // som (1), eom (1), seq (2), tag owner (0), message tag (3)
                // mctp body data - 1 byte
                0xA5,
                // test trailer - 2 bytes
                0xC,
                0xD,
            ]
        );
    }

    #[test]
    fn test_send_packet_multi() {
        let mut buffer = [0; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(
            TestMedium::new()
                .with_headers(&[0xA, 0xB], &[0xC, 0xD])
                // 4 bytes transport header + 4 bytes of data
                .with_mtu(12),
            &mut buffer,
        );

        let reply_context = MctpReplyContext {
            destination_endpoint_id: EndpointId::try_from(236).unwrap(),
            source_endpoint_id: EndpointId::try_from(192).unwrap(),
            packet_sequence_number: MctpSequenceNumber::new(1),
            message_tag: MctpMessageTag::try_from(3).unwrap(),
            medium_context: (),
        };

        // 10 byte to send over 3 packets
        let data_to_send = [0xA5, 0xB6, 0xC7, 0xD8, 0xE9, 0xFA, 0x0B, 0x1C, 0x2D, 0x3E];

        let mut state = context
            .serialize_packet(reply_context, &data_to_send)
            .unwrap();

        // First packet
        let packet1 = state.next().unwrap().unwrap();
        assert_eq!(
            packet1,
            &[
                // test header - 2 bytes
                0xA,
                0xB,
                // mctp transport header - 4 bytes
                0b0000_0001, // mctp reserved, header version
                192,         // destination endpoint id
                236,         // source endpoint id
                0b1010_0011, // som (1), eom (0), seq (2), tag owner (0), message tag (3)
                // mctp body data - 4 bytes
                0xA5,
                0xB6,
                0xC7,
                0xD8,
                // test trailer - 2 bytes
                0xC,
                0xD,
            ]
        );

        // Second packet (middle packet with 4 bytes of data)
        let packet2 = state.next().unwrap().unwrap();
        assert_eq!(
            packet2,
            &[
                // test header - 2 bytes
                0xA,
                0xB,
                // mctp transport header - 4 bytes
                0b0000_0001, // mctp reserved, header version
                192,         // destination endpoint id
                236,         // source endpoint id
                0b0011_0011, // som (0), eom (0), seq (3), tag owner (0), message tag (3)
                // mctp body data - 4 bytes
                0xE9,
                0xFA,
                0x0B,
                0x1C,
                // test trailer - 2 bytes
                0xC,
                0xD,
            ]
        );

        // Third packet (final packet with 2 bytes of data)
        let packet3 = state.next().unwrap().unwrap();
        assert_eq!(
            packet3,
            &[
                // test header - 2 bytes
                0xA,
                0xB,
                // mctp transport header - 4 bytes
                0b0000_0001, // mctp reserved, header version
                192,         // destination endpoint id
                236,         // source endpoint id
                0b0100_0011, // som (0), eom (1), seq (0), tag owner (0), message tag (3)
                // mctp body data - 2 bytes
                0x2D,
                0x3E,
                // test trailer - 2 bytes
                0xC,
                0xD,
            ]
        );

        // Verify no more packets
        assert!(state.next().is_none(), "Expected exactly 3 packets");
    }

    #[test]
    fn test_buffer_overflow_protection() {
        // Test that buffer overflow is properly prevented
        let mut small_buffer = [0u8; 16]; // Very small buffer
        let mut context =
            MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut small_buffer);

        // Create a packet that would cause overflow without protection
        let large_packet = [
            // transport header:
            0b0000_0001, // mctp reserved, header version
            0b0000_1001, // destination endpoint id (9)
            0b0001_0110, // source endpoint id (22)
            0b1000_0011, // som, eom, seq (0), to, tag (3)
            // message header:
            0b0000_0000, // integrity check (off) / message type (mctp control message)
            0b0000_0000, // rq, d, rsvd, instance id
            0b0000_0010, // command code (2: get endpoint id)
            0b0000_0000, // completion code
            // Large message body that would overflow small buffer
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
            0x09,
            0x0A,
            0x0B,
            0x0C,
            0x0D,
            0x0E,
            0x0F,
            0x10,
        ];

        // This should return an error instead of panicking
        let result = context.deserialize_packet(&large_packet);
        assert!(result.is_err());

        if let Err(MctpPacketError::HeaderParseError(msg)) = result {
            assert!(msg.contains("buffer overflow"));
        } else {
            panic!("Expected HeaderParseError with buffer overflow message");
        }
    }

    #[test]
    fn test_multi_packet_buffer_overflow() {
        // Test buffer overflow with multiple packets
        let mut small_buffer = [0u8; 20]; // Small buffer that can fit first packet but not second
        let mut context =
            MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut small_buffer);

        // First packet - fits in buffer
        let first_packet = [
            // transport header:
            0b0000_0001, // mctp reserved, header version
            0b0000_1001, // destination endpoint id (9)
            0b0001_0110, // source endpoint id (22)
            0b1000_0011, // som (1), eom (0), seq (0), to, tag (3)
            // message header:
            0b0000_0000, // integrity check (off) / message type (mctp control message)
            0b0000_0000, // rq, d, rsvd, instance id
            0b0000_0010, // command code (2: get endpoint id)
            0b0000_0000, // completion code
            // Small message body
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
        ];

        // First packet should succeed
        let result1 = context.deserialize_packet(&first_packet);
        assert!(result1.is_ok());
        assert!(result1.unwrap().is_none()); // No complete message yet

        // Second packet - would cause overflow
        let second_packet = [
            // transport header:
            0b0000_0001, // mctp reserved, header version
            0b0000_1001, // destination endpoint id (9)
            0b0001_0110, // source endpoint id (22)
            0b0101_0011, // som (0), eom (1), seq (1), to, tag (3) - correct sequence number
            // Large continuation that would overflow
            0x09,
            0x0A,
            0x0B,
            0x0C,
            0x0D,
            0x0E,
            0x0F,
            0x10,
            0x11,
            0x12,
            0x13,
            0x14,
            0x15,
            0x16,
            0x17,
            0x18,
        ];

        // Second packet should fail with buffer overflow
        let result2 = context.deserialize_packet(&second_packet);
        assert!(result2.is_err());

        if let Err(MctpPacketError::HeaderParseError(msg)) = result2 {
            assert!(msg.contains("buffer overflow"));
        } else {
            panic!("Expected HeaderParseError with buffer overflow message");
        }
    }

    #[test]
    fn test_transport_header_underflow() {
        // Test transport header parsing with insufficient bytes
        let mut buffer = [0u8; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut buffer);

        // Packet too short for transport header (only 3 bytes)
        let short_packet = [0x01, 0x02, 0x03];

        let result = context.deserialize_packet(&short_packet);
        assert!(result.is_err());

        if let Err(MctpPacketError::HeaderParseError(msg)) = result {
            assert!(msg.contains("cannot parse transport header"));
        } else {
            panic!("Expected HeaderParseError for short transport header");
        }
    }

    #[test]
    fn test_message_header_underflow() {
        // Test message body parsing with insufficient bytes for message header
        let mut buffer = [0u8; 1024];
        let mut context = MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut buffer);

        // Packet with transport header but no message header
        let incomplete_packet = [
            // transport header only (4 bytes)
            0b0000_0001, // mctp reserved, header version
            0b0000_1001, // destination endpoint id (9)
            0b0001_0110, // source endpoint id (22)
            0b1110_0011, // som (1), eom (1), seq (0), to, tag (3)
                         // No message header (need 4 more bytes)
        ];

        let result = context.deserialize_packet(&incomplete_packet);
        assert!(result.is_err());

        if let Err(MctpPacketError::HeaderParseError(msg)) = result {
            assert!(msg.contains("4 bytes for message header"));
        } else {
            panic!("Expected HeaderParseError for short message header");
        }
    }

    #[test]
    fn test_serialize_buffer_underflow() {
        // Test serialization with buffer too small for transport header
        let mut tiny_buffer = [0u8; 3]; // Too small for 4-byte transport header
        let mut context = MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut tiny_buffer);

        let reply_context = MctpReplyContext {
            destination_endpoint_id: EndpointId::try_from(236).unwrap(),
            source_endpoint_id: EndpointId::try_from(192).unwrap(),
            packet_sequence_number: MctpSequenceNumber::new(1),
            message_tag: MctpMessageTag::try_from(3).unwrap(),
            medium_context: (),
        };

        let state_result = context.serialize_packet(reply_context, &[0xA5]);
        assert!(state_result.is_ok());

        let mut state = state_result.unwrap();
        let packet_result = state.next().unwrap();

        // Should fail because buffer is too small for transport header
        assert!(packet_result.is_err());
        if let Err(MctpPacketError::SerializeError(msg)) = packet_result {
            assert!(msg.contains("assembly buffer too small"));
        } else {
            panic!("Expected SerializeError for small buffer");
        }
    }

    #[test]
    fn test_zero_size_assembly_buffer() {
        // Test with zero-size assembly buffer
        let mut empty_buffer = [0u8; 0];
        let mut context =
            MctpPacketContext::<TestMedium>::new(TestMedium::new(), &mut empty_buffer);

        let packet = [
            0b0000_0001, // mctp reserved, header version
            0b0000_1001, // destination endpoint id (9)
            0b0001_0110, // source endpoint id (22)
            0b1110_0011, // som (1), eom (1), seq (0), to, tag (3)
            0b0000_0000, // message header
            0b0000_0000,
            0b0000_0010,
            0b0000_0000,
        ];

        let result = context.deserialize_packet(&packet);
        assert!(result.is_err());

        if let Err(MctpPacketError::HeaderParseError(msg)) = result {
            assert!(msg.contains("buffer overflow"));
        } else {
            panic!("Expected buffer overflow error for zero-size buffer");
        }
    }
}

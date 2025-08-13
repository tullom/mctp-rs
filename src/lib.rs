#![no_std]
#![allow(dead_code)]

mod control_command;
mod endpoint_id;
mod mctp_command_code;
mod mctp_completion_code;
mod mctp_message_tag;
mod mctp_message_type;
mod mctp_sequence_number;
mod mctp_transport_header;
mod medium;

use bit_register::bit_register;
use endpoint_id::EndpointId;

use crate::mctp_command_code::MctpCommandCode;
use crate::mctp_completion_code::MctpCompletionCode;
use crate::mctp_message_tag::MctpMessageTag;
use crate::mctp_message_type::MctpMessageType;
use crate::mctp_sequence_number::MctpSequenceNumber;
use crate::mctp_transport_header::MctpTransportHeader;
use crate::medium::{MctpMedium, MctpMediumFrame, MediumOrGenericError};

#[cfg(test)]
mod tests_1 {
    extern crate std;
    use crate::mctp_message_tag::MctpMessageTag;

    use super::*;
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

/// Represents the state needed to construct a repsonse to a request:
/// the MCTP transport source/destination, the sequence number to use for
/// the reply, and the medium-specific context that came with the request.
#[derive(Debug, PartialEq, Eq)]
pub struct MctpReplyContext<M: MctpMedium> {
    pub destination_endpoint_id: EndpointId,
    pub source_endpoint_id: EndpointId,
    pub packet_sequence_number: MctpSequenceNumber,
    pub message_tag: MctpMessageTag,
    pub medium_context: M::ReplyContext,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MctpMessageHeaderAndBody<'buffer> {
    Control {
        header: MctpControlMessageHeaderBitRegister,
        body: &'buffer [u8],
    },
    VendorDefinedPci {
        header: MctpMessageHeaderBitRegister,
        body: &'buffer [u8],
    },
    VendorDefinedIana {
        header: MctpMessageHeaderBitRegister,
        body: &'buffer [u8],
    },
}

bit_register! {
    #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
    pub struct MctpMessageHeaderBitRegister: little_endian u32 {
        pub integrity_check: u8 => [31],
        pub message_type: MctpMessageType => [24:30],
        pub rest: u32 => [0:23],
    }
}

bit_register! {
    /// if message_type is MctpMessageType::MctpControl, then the header is a MctpControlMessageHeaderBitRegister
    /// with message specific fields.
    #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
    pub struct MctpControlMessageHeaderBitRegister: little_endian u32 {
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
mod tests_2 {
    use super::*;
    #[test]
    fn test_mctp_control_message_header_bit_register() {
        assert_eq!(
            MctpControlMessageHeaderBitRegister::try_from(u32::from_be_bytes([
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]))
            .unwrap(),
            MctpControlMessageHeaderBitRegister {
                integrity_check: 0,
                ..Default::default()
            }
        );
        assert_eq!(
            MctpControlMessageHeaderBitRegister::try_from(u32::from_be_bytes([
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]))
            .unwrap(),
            MctpControlMessageHeaderBitRegister {
                integrity_check: 1,
                ..Default::default()
            }
        );
    }
}

/// The body of the message
pub enum MctpMessageData {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ProtocolError {
    ExpectedStartOfMessage,
    UnexpectedStartOfMessage,
    MessageTagMismatch(MctpMessageTag, MctpMessageTag),
    TagOwnerMismatch(u8, u8),
    SourceEndpointIdMismatch(EndpointId, EndpointId),
    UnexpectedPacketSequenceNumber(MctpSequenceNumber, MctpSequenceNumber),
    CompletionCodeOnRequestMessage(MctpCompletionCode),
    SendMessageWhileAssembling,
    SendingMessageWhileReceiving,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MctpPacketError<MediumError: core::fmt::Debug + Copy + Clone + PartialEq + Eq> {
    HeaderParseError(&'static str),
    CommandParseError(&'static str),
    SerializeError(&'static str),
    ProtocolError(ProtocolError),
    MediumError(MediumError),
}

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

#[derive(Debug, PartialEq, Eq)]
struct SerializePacketState<'source, 'assembly, M: MctpMedium> {
    medium: &'assembly M,
    reply_context: MctpReplyContext<M>,
    current_packet_num: u8,
    source_buffer: &'source [u8],
    assembly_buffer: &'assembly mut [u8],
}

impl<'source, 'assembly, M: MctpMedium> SerializePacketState<'source, 'assembly, M> {
    fn next(&mut self) -> Option<Result<&[u8], MctpPacketError<M::Error>>> {
        if self.source_buffer.is_empty() {
            return None;
        }

        let packet = self
            .medium
            .serialize(
                self.reply_context.medium_context.clone(),
                self.assembly_buffer,
                |buffer: &mut [u8]| {
                    let packet_size = self.medium.max_message_body_size().min(buffer.len());
                    if packet_size < 4 {
                        return Err(MctpPacketError::SerializeError(
                            "assembly buffer too small for mctp transport header",
                        ));
                    }

                    let body_size = (packet_size - 4).min(self.source_buffer.len());
                    let body = &self.source_buffer[..body_size];
                    self.source_buffer = &self.source_buffer[body_size..];

                    let start_of_message = if self.current_packet_num == 0 { 1 } else { 0 };
                    let end_of_message = if self.source_buffer.is_empty() { 1 } else { 0 };
                    let packet_sequence_number = self.reply_context.packet_sequence_number.inc();
                    let transport_header = MctpTransportHeader {
                        reserved: 0,
                        header_version: 1,
                        start_of_message,
                        end_of_message,
                        packet_sequence_number,
                        tag_owner: 0,
                        message_tag: self.reply_context.message_tag,
                        source_endpoint_id: self.reply_context.destination_endpoint_id,
                        destination_endpoint_id: self.reply_context.source_endpoint_id,
                    };
                    let transport_header_value: u32 = transport_header
                        .try_into()
                        .map_err(MctpPacketError::SerializeError)?;

                    // transport header is the first 4 bytes of the buffer
                    buffer[0..4].copy_from_slice(&transport_header_value.to_be_bytes());
                    // message body is the rest of the buffer, up to the packet size
                    buffer[4..4 + body_size].copy_from_slice(body);
                    Ok(4 + body_size)
                },
            )
            // TODO - .into() for these error types
            .map_err(|err| match err {
                MediumOrGenericError::Medium(e) => MctpPacketError::MediumError(e),
                MediumOrGenericError::Generic(e) => e,
            });

        // Increment packet number for next call
        self.current_packet_num += 1;

        Some(packet)
    }
}

struct MctpPacketContext<'buf, M: MctpMedium> {
    assembly_state: AssemblyState,
    medium: M,
    packet_assembly_buffer: &'buf mut [u8],
}

impl<'buf, M: MctpMedium> MctpPacketContext<'buf, M> {
    pub fn new(medium: M, packet_assembly_buffer: &'buf mut [u8]) -> Self {
        Self {
            medium,
            assembly_state: AssemblyState::Idle,
            packet_assembly_buffer,
        }
    }

    pub fn receive_packet<'packet>(
        &mut self,
        packet: &'packet [u8],
    ) -> Result<Option<MctpMessage<'_, M>>, MctpPacketError<M::Error>> {
        let (medium_frame, packet) = self
            .medium
            .deserialize(packet)
            .map_err(MctpPacketError::MediumError)?;
        let (transport_header, packet) = self.deserialize_transport_header(packet)?;

        let mut state = match self.assembly_state {
            AssemblyState::Idle => {
                if transport_header.start_of_message == 0 {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::ExpectedStartOfMessage,
                    ));
                }

                AssemblingState {
                    message_tag: transport_header.message_tag,
                    tag_owner: transport_header.tag_owner,
                    source_endpoint_id: transport_header.source_endpoint_id,
                    packet_sequence_number: transport_header.packet_sequence_number,
                    packet_assembly_buffer_index: 0,
                }
            }
            AssemblyState::Receiving(assembling_state) => {
                if transport_header.start_of_message != 0 {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::UnexpectedStartOfMessage,
                    ));
                }
                if assembling_state.message_tag != transport_header.message_tag {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::MessageTagMismatch(
                            assembling_state.message_tag,
                            transport_header.message_tag,
                        ),
                    ));
                }
                if assembling_state.tag_owner != transport_header.tag_owner {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::TagOwnerMismatch(
                            assembling_state.tag_owner,
                            transport_header.tag_owner,
                        ),
                    ));
                }
                if assembling_state.source_endpoint_id != transport_header.source_endpoint_id {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::SourceEndpointIdMismatch(
                            assembling_state.source_endpoint_id,
                            transport_header.source_endpoint_id,
                        ),
                    ));
                }
                let expected_sequence_number = assembling_state.packet_sequence_number.next();
                if expected_sequence_number != transport_header.packet_sequence_number {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::UnexpectedPacketSequenceNumber(
                            expected_sequence_number,
                            transport_header.packet_sequence_number,
                        ),
                    ));
                }
                assembling_state
            }
        };

        let buffer_idx = state.packet_assembly_buffer_index;
        let packet_size = medium_frame.packet_size();
        if packet_size < 4 {
            return Err(MctpPacketError::HeaderParseError(
                "transport frame indicated packet length < 4",
            ));
        }
        let packet_size = packet_size - 4; // to account for the transport header
        if packet.len() < packet_size {
            return Err(MctpPacketError::HeaderParseError(
                "packet.len() < packet_size",
            ));
        }
        self.packet_assembly_buffer[buffer_idx..buffer_idx + packet_size]
            .copy_from_slice(&packet[..packet_size]);
        state.packet_assembly_buffer_index += packet_size;

        let message = if transport_header.end_of_message == 1 {
            self.assembly_state = AssemblyState::Idle;
            let (message_body, message_integrity_check) = self.parse_message_body(
                &self.packet_assembly_buffer[..state.packet_assembly_buffer_index],
            )?;
            Some(MctpMessage {
                reply_context: MctpReplyContext {
                    destination_endpoint_id: transport_header.destination_endpoint_id,
                    source_endpoint_id: transport_header.source_endpoint_id,
                    packet_sequence_number: transport_header.packet_sequence_number,
                    message_tag: transport_header.message_tag,
                    medium_context: medium_frame.reply_context(),
                },
                header_and_body: message_body,
                message_integrity_check,
            })
        } else {
            self.assembly_state = AssemblyState::Receiving(state);
            None
        };

        Ok(message)
    }

    fn deserialize_transport_header<'packet>(
        &mut self,
        packet: &'packet [u8],
    ) -> Result<(MctpTransportHeader, &'packet [u8]), MctpPacketError<M::Error>> {
        let transport_header_value = u32::from_be_bytes(packet[0..4].try_into().map_err(|_| {
            MctpPacketError::HeaderParseError("Packet is too small, cannot parse transport header")
        })?);
        let transport_header = MctpTransportHeader::try_from(transport_header_value)
            .map_err(|_| MctpPacketError::HeaderParseError("Invalid transport header"))?;
        let packet = &packet[4..];
        Ok((transport_header, packet))
    }

    fn parse_message_body<'s>(
        &'s self,
        packet: &'s [u8],
    ) -> Result<(MctpMessageHeaderAndBody<'s>, Option<u8>), MctpPacketError<M::Error>> {
        // first four bytes are the message header, parse with MctpMessageHeaderBitRegister
        // to figure out the type, then based on that, parse the type specific header
        let header_u32 = u32::from_be_bytes(packet[0..4].try_into().map_err(|_| {
            MctpPacketError::HeaderParseError("packet < 4 bytes for message header")
        })?);
        let header = MctpMessageHeaderBitRegister::try_from(header_u32)
            .map_err(MctpPacketError::HeaderParseError)?;
        let packet = &packet[4..];

        let header_and_body = match header.message_type {
            MctpMessageType::MctpControl => {
                let header = MctpControlMessageHeaderBitRegister::try_from(header_u32)
                    .map_err(MctpPacketError::HeaderParseError)?;

                // completion code is only present on reponse message
                if header.request_bit == 1 && header.completion_code != MctpCompletionCode::Success
                {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::CompletionCodeOnRequestMessage(header.completion_code),
                    ));
                }

                MctpMessageHeaderAndBody::Control {
                    header,
                    body: packet,
                }
            }
            MctpMessageType::VendorDefinedIana => MctpMessageHeaderAndBody::VendorDefinedIana {
                header,
                body: packet,
            },
            MctpMessageType::VendorDefinedPci => MctpMessageHeaderAndBody::VendorDefinedPci {
                header,
                body: packet,
            },
            _ => return Err(MctpPacketError::HeaderParseError("Invalid message type")),
        };

        // TODO - compute message integrity check if header.integrity_check is set
        Ok((header_and_body, None))
    }

    fn serialize_packet<'source>(
        &'buf mut self,
        reply_context: MctpReplyContext<M>,
        message: &'source [u8],
    ) -> Result<SerializePacketState<'source, 'buf, M>, MctpPacketError<M::Error>> {
        match self.assembly_state {
            AssemblyState::Idle => {}
            _ => {
                return Err(MctpPacketError::ProtocolError(
                    ProtocolError::SendMessageWhileAssembling,
                ));
            }
        };
        Ok(SerializePacketState {
            medium: &self.medium,
            reply_context,
            current_packet_num: 0,
            source_buffer: message,
            assembly_buffer: self.packet_assembly_buffer,
        })
    }
}

#[cfg(test)]
mod mctp_context_tests {
    use super::*;
    use crate::medium::MctpMediumFrame;
    use pretty_assertions::assert_eq;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    struct TestMedium {
        header: &'static [u8],
        trailer: &'static [u8],
        mtu: usize,
    }
    impl TestMedium {
        fn new() -> Self {
            Self {
                header: &[],
                trailer: &[],
                mtu: 32,
            }
        }
        fn with_headers(mut self, header: &'static [u8], trailer: &'static [u8]) -> Self {
            self.header = header;
            self.trailer = trailer;
            self
        }
        fn with_mtu(mut self, mtu: usize) -> Self {
            self.mtu = mtu;
            self
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    struct TestMediumFrame(usize);

    impl MctpMedium for TestMedium {
        type Frame = TestMediumFrame;
        type Error = ();
        type ReplyContext = ();

        fn deserialize<'buf>(
            &self,
            packet: &'buf [u8],
        ) -> Result<(Self::Frame, &'buf [u8]), Self::Error> {
            let packet_len = packet.len();

            // check that header / trailer is present and correct
            if packet[0..self.header.len()] != *self.header {
                panic!("header mismatch");
            }
            if packet[packet_len - self.trailer.len()..packet_len] != *self.trailer {
                panic!("trailer mismatch");
            }

            let packet = &packet[self.header.len()..packet_len - self.trailer.len()];
            Ok((TestMediumFrame(packet_len), packet))
        }
        fn max_message_body_size(&self) -> usize {
            self.mtu
        }
        fn serialize<'buf, E, F>(
            &self,
            _: Self::ReplyContext,
            buffer: &'buf mut [u8],
            message_writer: F,
        ) -> Result<&'buf [u8], MediumOrGenericError<Self::Error, E>>
        where
            F: for<'a> FnOnce(&'a mut [u8]) -> Result<usize, E>,
        {
            let header_len = self.header.len();
            let trailer_len = self.trailer.len();

            // Ensure buffer can fit at least headers and trailers
            if buffer.len() < header_len + trailer_len {
                return Err(MediumOrGenericError::Medium(()));
            }

            // Calculate available space for message (respecting MTU)
            let max_packet_size = self.mtu.min(buffer.len());
            if max_packet_size < header_len + trailer_len {
                return Err(MediumOrGenericError::Medium(()));
            }
            let max_message_size = max_packet_size - header_len - trailer_len;

            buffer[0..header_len].copy_from_slice(self.header);
            let size = message_writer(&mut buffer[header_len..header_len + max_message_size])
                .map_err(MediumOrGenericError::Generic)?;
            let len = header_len + size;
            buffer[len..len + trailer_len].copy_from_slice(self.trailer);
            Ok(&buffer[..len + trailer_len])
        }
    }

    impl MctpMediumFrame<TestMedium> for TestMediumFrame {
        fn packet_size(&self) -> usize {
            self.0
        }
        fn reply_context(&self) -> <TestMedium as MctpMedium>::ReplyContext {
            ()
        }
    }

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
                .receive_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0)
                .unwrap(),
            None
        );

        assert_eq!(
            context.receive_packet(EMPTY_PACKET_EOM.0).unwrap().unwrap(),
            MctpMessage {
                reply_context: MctpReplyContext {
                    destination_endpoint_id: EndpointId::Id(9),
                    source_endpoint_id: EndpointId::Id(22),
                    packet_sequence_number: MctpSequenceNumber::new(1),
                    message_tag: MctpMessageTag::try_from(3).unwrap(),
                    medium_context: (),
                },
                header_and_body: MctpMessageHeaderAndBody::Control {
                    header: MctpControlMessageHeaderBitRegister {
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
            context.receive_packet(&[
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
            .receive_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0)
            .unwrap();

        assert_eq!(
            context.receive_packet(&[
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
            .receive_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0)
            .unwrap();

        // message tag = 1
        assert_eq!(
            context.receive_packet(&[
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
}

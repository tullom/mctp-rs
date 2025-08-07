#![no_std]
#![allow(dead_code)]
// #![allow(unused_variables)]

mod endpoint_id;
mod mctp_command_code;
mod mctp_completion_code;
mod mctp_message_type;
mod medium;

use bit_register::bit_register;
use core::marker::PhantomData;
use endpoint_id::EndpointId;

use crate::mctp_command_code::MctpCommandCode;
use crate::mctp_completion_code::MctpCompletionCode;
use crate::mctp_message_type::MctpMessageType;
use crate::medium::{MctpMedium, MctpMediumFrame};

bit_register! {
    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct MctpTransportHeader: little_endian u32 {
        pub reserved: u8 => [28:31],
        pub header_version: u8 => [24:27],
        pub destination_endpoint_id: EndpointId => [16:23],
        pub source_endpoint_id: EndpointId => [8:15],
        pub start_of_message: u8 => [7],
        pub end_of_message: u8 => [6],
        pub packet_sequence_number: u8 => [4:5],
        pub tag_owner: u8 => [3],
        pub message_tag: u8 => [0:2],
    }
}

#[cfg(test)]
mod tests_1 {
    extern crate std;
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
                message_tag: 3,
                ..Default::default()
            }
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MctpMessageBody<'packet> {
    pub header_and_body: MctpMessageHeaderAndBody<'packet>,
    pub message_integrity_check: Option<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MctpMessageHeaderAndBody<'packet> {
    Control {
        header: MctpControlMessageHeaderBitRegister,
        body: MctpControlMessageBody,
    },
    VendorDefinedPci {
        body: &'packet [u8],
    },
    VendorDefinedIana {
        body: &'packet [u8],
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum MctpControlMessageBody {
    GetEndpointId,
    SetEndpointId,
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
    MessageTagMismatch(u8, u8),
    TagOwnerMismatch(u8, u8),
    SourceEndpointIdMismatch(EndpointId, EndpointId),
    UnexpectedPacketSequenceNumber(u8, u8),
    CompletionCodeOnRequestMessage(MctpCompletionCode),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MctpPacketError<MediumError: core::fmt::Debug + Copy + Clone + PartialEq + Eq> {
    ParseError(&'static str),
    ProtocolError(ProtocolError),
    MediumError(MediumError),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AssemblyState {
    Idle,
    Assembling(AssemblingState),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct AssemblingState {
    message_tag: u8,
    tag_owner: u8,
    source_endpoint_id: EndpointId,
    packet_sequence_number: u8,
    packet_assembly_buffer_index: usize,
}

struct MctpPacketContext<'buf, M: MctpMedium> {
    packet_assembly_buffer: &'buf mut [u8],
    assembly_state: AssemblyState,
    phantom: PhantomData<M>,
}

impl<'buf, M: MctpMedium> MctpPacketContext<'buf, M> {
    pub fn new(packet_assembly_buffer: &'buf mut [u8]) -> Self {
        Self {
            packet_assembly_buffer,
            assembly_state: AssemblyState::Idle,
            phantom: PhantomData,
        }
    }

    pub fn on_receive_packet<'packet>(
        &mut self,
        packet: &'packet [u8],
    ) -> Result<Option<MctpMessageBody<'_>>, MctpPacketError<M::Error>> {
        let (medium_frame, packet) =
            M::deserialize(packet).map_err(MctpPacketError::MediumError)?;
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
            AssemblyState::Assembling(assembling_state) => {
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
                let expected_sequence_number = (assembling_state.packet_sequence_number + 1) % 4;
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
            return Err(MctpPacketError::ParseError(
                "transport frame indicated packet length < 4",
            ));
        }
        let packet_size = packet_size - 4; // to account for the transport header
        if packet.len() < packet_size {
            return Err(MctpPacketError::ParseError("packet.len() < packet_size"));
        }
        self.packet_assembly_buffer[buffer_idx..buffer_idx + packet_size]
            .copy_from_slice(&packet[..packet_size]);
        state.packet_assembly_buffer_index += packet_size;

        let message_body = if transport_header.end_of_message == 1 {
            self.assembly_state = AssemblyState::Idle;
            let message_body = self.parse_message_body(
                &self.packet_assembly_buffer[..state.packet_assembly_buffer_index],
            )?;
            Some(message_body)
        } else {
            self.assembly_state = AssemblyState::Assembling(state);
            None
        };

        Ok(message_body)
    }

    fn deserialize_transport_header<'packet>(
        &mut self,
        packet: &'packet [u8],
    ) -> Result<(MctpTransportHeader, &'packet [u8]), MctpPacketError<M::Error>> {
        let transport_header_value = u32::from_be_bytes(packet[0..4].try_into().map_err(|_| {
            MctpPacketError::ParseError("Packet is too small, cannot parse transport header")
        })?);
        let transport_header = MctpTransportHeader::try_from(transport_header_value)
            .map_err(|_| MctpPacketError::ParseError("Invalid transport header"))?;
        let packet = &packet[4..];
        Ok((transport_header, packet))
    }

    fn parse_message_body<'s>(
        &'s self,
        packet: &'s [u8],
    ) -> Result<MctpMessageBody<'s>, MctpPacketError<M::Error>> {
        // first four bytes are the message header, parse with MctpMessageHeaderBitRegister
        // to figure out the type, then based on that, parse the type specific header
        let header_u32 = u32::from_be_bytes(
            packet[0..4]
                .try_into()
                .map_err(|_| MctpPacketError::ParseError("packet < 4 bytes for message header"))?,
        );
        let header = MctpMessageHeaderBitRegister::try_from(header_u32)
            .map_err(MctpPacketError::ParseError)?;
        let packet = &packet[4..];

        let header_and_body = match header.message_type {
            MctpMessageType::MctpControl => {
                let header = MctpControlMessageHeaderBitRegister::try_from(header_u32)
                    .map_err(MctpPacketError::ParseError)?;

                extern crate std;
                std::println!("header: {:#X?}", header);

                // completion code is only present on reponse message
                if header.request_bit == 1 && header.completion_code != MctpCompletionCode::Success
                {
                    return Err(MctpPacketError::ProtocolError(
                        ProtocolError::CompletionCodeOnRequestMessage(header.completion_code),
                    ));
                }

                MctpMessageHeaderAndBody::Control {
                    header,
                    body: self.parse_control_message_body(header.command_code, packet)?,
                }
            }
            MctpMessageType::VendorDefinedIana => {
                MctpMessageHeaderAndBody::VendorDefinedIana { body: packet }
            }
            MctpMessageType::VendorDefinedPci => {
                MctpMessageHeaderAndBody::VendorDefinedPci { body: packet }
            }
            _ => return Err(MctpPacketError::ParseError("Invalid message type")),
        };

        // TODO - compute message integrity check if header.integrity_check is set
        Ok(MctpMessageBody {
            header_and_body,
            message_integrity_check: None,
        })
    }

    fn parse_control_message_body(
        &self,
        command_code: MctpCommandCode,
        _body: &[u8],
    ) -> Result<MctpControlMessageBody, MctpPacketError<M::Error>> {
        // TODO - parse the body of the message
        Ok(match command_code {
            MctpCommandCode::GetEndpointId => MctpControlMessageBody::GetEndpointId,
            MctpCommandCode::SetEndpointId => MctpControlMessageBody::SetEndpointId,
            _ => {
                return Err(MctpPacketError::ParseError("Invalid control command code"));
            }
        })
    }
}

#[cfg(test)]
mod tests_3 {
    use crate::medium::MctpMediumFrame;

    use super::*;

    struct TestMedium;
    struct TestMediumFrame(usize);

    impl MctpMedium for TestMedium {
        type Frame = TestMediumFrame;
        type Error = ();
        fn deserialize(packet: &[u8]) -> Result<(Self::Frame, &[u8]), Self::Error> {
            Ok((TestMediumFrame(packet.len()), packet))
        }
        fn serialize<F>(
            _frame: Self::Frame,
            buffer: &mut [u8],
            message_writer: F,
        ) -> Result<&[u8], Self::Error>
        where
            F: Fn(&mut [u8]) -> Result<&[u8], Self::Error>,
        {
            message_writer(buffer)
        }
    }

    impl MctpMediumFrame<TestMedium> for TestMediumFrame {
        fn packet_size(&self) -> usize {
            self.0
        }
    }

    struct Packet(&'static [u8]);
    const GET_ENDPOINT_ID_PACKET_NO_EOM: Packet = Packet(&[
        // test medium frame (header + trailer): 0 bytes
        // transport header:
        0b0000_0000, // mctp reserved, header version
        0b0000_0000, // destination endpoint id
        0b0000_0000, // source endpoint id
        0b1000_0000, // som, eom, seq (0), to, tag
        // message header:
        0b0000_0000, // integrity check (off) / message type (mctp control message)
        0b0000_0000, // rq, d, rsvd, instance id
        0b0000_0010, // command code (2: get endpoint id)
        0b0000_0000, // completion code
    ]);

    const EMPTY_PACKET_EOM: Packet = Packet(&[
        // transport header:
        0b0000_0000, // mctp reserved, header version
        0b0000_0000, // destination endpoint id
        0b0000_0000, // source endpoint id
        0b0101_0000, // som, eom, seq (1), to, tag
    ]);

    #[test]
    fn test_mctp_packet_context_split_over_two_packets() {
        let mut buffer = [0; 1024];
        let mut context: MctpPacketContext<'_, TestMedium> =
            MctpPacketContext::<TestMedium>::new(&mut buffer);

        assert_eq!(
            context.on_receive_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0),
            Ok(None)
        );

        assert_eq!(
            context.on_receive_packet(EMPTY_PACKET_EOM.0),
            Ok(Some(MctpMessageBody {
                header_and_body: MctpMessageHeaderAndBody::Control {
                    header: MctpControlMessageHeaderBitRegister {
                        integrity_check: 0,
                        message_type: MctpMessageType::MctpControl,
                        command_code: MctpCommandCode::GetEndpointId,
                        ..Default::default()
                    },
                    body: MctpControlMessageBody::GetEndpointId,
                },
                message_integrity_check: None,
            }))
        );
    }

    #[test]
    fn test_mctp_packet_context_lacking_start_of_message() {
        let mut buffer = [0; 1024];
        let mut context: MctpPacketContext<'_, TestMedium> =
            MctpPacketContext::<TestMedium>::new(&mut buffer);

        assert_eq!(
            context.on_receive_packet(&[
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
    fn test_mctp_packet_context_repeated_start_of_message() {
        let mut buffer = [0; 1024];
        let mut context: MctpPacketContext<'_, TestMedium> =
            MctpPacketContext::<TestMedium>::new(&mut buffer);

        context
            .on_receive_packet(GET_ENDPOINT_ID_PACKET_NO_EOM.0)
            .unwrap();

        assert_eq!(
            context.on_receive_packet(&[
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
}

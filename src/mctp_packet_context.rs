use crate::{
    AssemblingState, AssemblyState, MctpMessage, MctpPacketError,
    deserialize::{parse_message_body, parse_transport_header},
    endpoint_id::EndpointId,
    error::{MctpPacketResult, ProtocolError},
    mctp_message_tag::MctpMessageTag,
    mctp_sequence_number::MctpSequenceNumber,
    medium::{MctpMedium, MctpMediumFrame},
    serialize::SerializePacketState,
};

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

/// Context for serializing and deserializing an MCTP message, which may be split among multiple packets.
pub struct MctpPacketContext<'buf, M: MctpMedium> {
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

    pub fn deserialize_packet(
        &mut self,
        packet: &[u8],
    ) -> MctpPacketResult<Option<MctpMessage<'_, M>>, M::Error> {
        let (medium_frame, packet) = self
            .medium
            .deserialize(packet)
            .map_err(MctpPacketError::MediumError)?;
        let (transport_header, packet) = parse_transport_header::<M>(packet)?;

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
        // Check bounds to prevent buffer overflow
        if buffer_idx + packet_size > self.packet_assembly_buffer.len() {
            return Err(MctpPacketError::HeaderParseError(
                "packet assembly buffer overflow - insufficient space",
            ));
        }
        self.packet_assembly_buffer[buffer_idx..buffer_idx + packet_size]
            .copy_from_slice(&packet[..packet_size]);
        state.packet_assembly_buffer_index += packet_size;

        let message = if transport_header.end_of_message == 1 {
            self.assembly_state = AssemblyState::Idle;
            let (message_body, message_integrity_check) = parse_message_body::<M>(
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

    pub fn serialize_packet<'source>(
        &'buf mut self,
        reply_context: MctpReplyContext<M>,
        message: &'source [u8],
    ) -> MctpPacketResult<SerializePacketState<'source, 'buf, M>, M::Error> {
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

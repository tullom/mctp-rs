use crate::{
    MctpMessageHeader, MctpMessageType, MctpPacketError, error::MctpPacketResult,
    mctp_packet_context::MctpReplyContext, mctp_transport_header::MctpTransportHeader,
    medium::MctpMedium,
};

#[derive(Debug, PartialEq, Eq)]
pub struct SerializePacketState<'source, 'assembly, M: MctpMedium> {
    pub(crate) medium: &'assembly M,
    pub(crate) reply_context: MctpReplyContext<M>,
    pub(crate) current_packet_num: u8,
    pub(crate) serialized_message_header: bool,
    pub(crate) source_message_header: Option<MctpMessageHeader>,
    pub(crate) source_message_body: &'source [u8],
    pub(crate) assembly_buffer: &'assembly mut [u8],
}

pub const TRANSPORT_HEADER_SIZE: usize = 4;

impl<'source, 'assembly, M: MctpMedium> SerializePacketState<'source, 'assembly, M> {
    pub fn next(&mut self) -> Option<MctpPacketResult<&[u8], M>> {
        if self.source_message_header.is_none() && self.source_message_body.is_empty() {
            return None;
        }

        let packet = self.medium.serialize(
            self.reply_context.medium_context,
            self.assembly_buffer,
            |buffer: &mut [u8]| {
                let mut tmp_header_buffer = [0u8; 4];

                let max_packet_size = self.medium.max_message_body_size().min(buffer.len());
                if max_packet_size < TRANSPORT_HEADER_SIZE {
                    return Err(MctpPacketError::SerializeError(
                        "assembly buffer too small for mctp transport header",
                    ));
                }

                let header = if let Some(header) = self.source_message_header.take() {
                    let message_type = header.message_type;
                    let header_value: u32 = header.try_into().unwrap();
                    let header_size = match message_type {
                        MctpMessageType::MctpControl => 4,
                        MctpMessageType::VendorDefinedPci => 3,
                        MctpMessageType::VendorDefinedIana => 3,
                        _ => return Err(MctpPacketError::UnsupportedMessageType(message_type)),
                    };

                    tmp_header_buffer[0..4].copy_from_slice(&header_value.to_be_bytes());
                    &tmp_header_buffer[0..header_size]
                } else {
                    &[]
                };

                if max_packet_size < TRANSPORT_HEADER_SIZE + header.len() {
                    return Err(MctpPacketError::SerializeError(
                        "assembly buffer too small for mctp message header",
                    ));
                }

                let body_size = (max_packet_size - TRANSPORT_HEADER_SIZE - header.len())
                    .min(self.source_message_body.len());

                // if there is no room for any of the body, and the body is not empty,
                // then return an error, otherwise we infinate loop sending packets with headers and
                // no body, making it impossible to ever assemble a message
                if body_size == 0 && !self.source_message_body.is_empty() {
                    return Err(MctpPacketError::SerializeError(
                        "assembly buffer too small for non-empty message body",
                    ));
                }

                let body = &self.source_message_body[..body_size];
                self.source_message_body = &self.source_message_body[body_size..];

                let start_of_message = if self.current_packet_num == 0 { 1 } else { 0 };
                let end_of_message = if self.source_message_body.is_empty() {
                    1
                } else {
                    0
                };
                let packet_sequence_number = self.reply_context.packet_sequence_number.inc();
                let transport_header: u32 = MctpTransportHeader {
                    reserved: 0,
                    header_version: 1,
                    start_of_message,
                    end_of_message,
                    packet_sequence_number,
                    tag_owner: 0,
                    message_tag: self.reply_context.message_tag,
                    source_endpoint_id: self.reply_context.destination_endpoint_id,
                    destination_endpoint_id: self.reply_context.source_endpoint_id,
                }
                .try_into()
                .map_err(MctpPacketError::SerializeError)?;

                // write the transport header, (optional) message header, and message body
                let mut cursor = 0;
                buffer[cursor..cursor + TRANSPORT_HEADER_SIZE]
                    .copy_from_slice(&transport_header.to_be_bytes());
                cursor += TRANSPORT_HEADER_SIZE;
                buffer[cursor..cursor + header.len()].copy_from_slice(header);
                cursor += header.len();
                // message body is the rest of the buffer, up to the packet size
                buffer[cursor..cursor + body.len()].copy_from_slice(body);
                Ok(cursor + body.len())
            },
        );

        // Increment packet number for next call
        if packet.is_ok() {
            self.current_packet_num += 1;
        }

        Some(packet)
    }
}

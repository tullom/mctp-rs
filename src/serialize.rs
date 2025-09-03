use crate::{
    MctpPacketError, error::MctpPacketResult, mctp_packet_context::MctpReplyContext,
    mctp_transport_header::MctpTransportHeader, medium::MctpMedium,
};

#[derive(Debug, PartialEq, Eq)]
pub struct SerializePacketState<'source, 'assembly, M: MctpMedium> {
    pub(crate) medium: &'assembly M,
    pub(crate) reply_context: MctpReplyContext<M>,
    pub(crate) current_packet_num: u8,
    pub(crate) source_buffer: &'source [u8],
    pub(crate) assembly_buffer: &'assembly mut [u8],
}

impl<'source, 'assembly, M: MctpMedium> SerializePacketState<'source, 'assembly, M> {
    pub fn next(&mut self) -> Option<MctpPacketResult<&[u8], M::Error>> {
        if self.source_buffer.is_empty() {
            return None;
        }

        let packet = self
            .medium
            .serialize(
                self.reply_context.medium_context,
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

                    // transport header is the first 4 bytes of the buffer
                    buffer[0..4].copy_from_slice(&transport_header.to_be_bytes());
                    // message body is the rest of the buffer, up to the packet size
                    buffer[4..4 + body_size].copy_from_slice(body);
                    Ok(4 + body_size)
                },
            )
            .map_err(Into::<MctpPacketError<M::Error>>::into);

        // Increment packet number for next call
        if packet.is_ok() {
            self.current_packet_num += 1;
        }

        Some(packet)
    }
}

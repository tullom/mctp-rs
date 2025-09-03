use crate::{
    MctpMessageHeader, MctpMessageHeaderAndBody, MctpPacketError, error::ProtocolError,
    mctp_completion_code::MctpCompletionCode,
    mctp_control_message_header::MctpControlMessageHeader, mctp_message_type::MctpMessageType,
    mctp_transport_header::MctpTransportHeader, medium::MctpMedium,
};

pub(crate) fn parse_transport_header<M: MctpMedium>(
    packet: &[u8],
) -> Result<(MctpTransportHeader, &[u8]), MctpPacketError<M::Error>> {
    if packet.len() < 4 {
        return Err(MctpPacketError::HeaderParseError(
            "Packet is too small, cannot parse transport header",
        ));
    }
    let transport_header_value = u32::from_be_bytes(packet[0..4].try_into().map_err(|_| {
        MctpPacketError::HeaderParseError("Packet is too small, cannot parse transport header")
    })?);
    let transport_header = MctpTransportHeader::try_from(transport_header_value)
        .map_err(|_| MctpPacketError::HeaderParseError("Invalid transport header"))?;
    let packet = &packet[4..];
    Ok((transport_header, packet))
}

pub(crate) fn parse_message_body<M: MctpMedium>(
    packet: &[u8],
) -> Result<(MctpMessageHeaderAndBody<'_>, Option<u8>), MctpPacketError<M::Error>> {
    // first four bytes are the message header, parse with MctpMessageHeader
    // to figure out the type, then based on that, parse the type specific header
    if packet.len() < 4 {
        return Err(MctpPacketError::HeaderParseError(
            "packet < 4 bytes for message header",
        ));
    }
    let header_u32 =
        u32::from_be_bytes(packet[0..4].try_into().map_err(|_| {
            MctpPacketError::HeaderParseError("packet < 4 bytes for message header")
        })?);
    let header =
        MctpMessageHeader::try_from(header_u32).map_err(MctpPacketError::HeaderParseError)?;
    let packet = &packet[4..];

    let header_and_body = match header.message_type {
        MctpMessageType::MctpControl => {
            let header = MctpControlMessageHeader::try_from(header_u32)
                .map_err(MctpPacketError::HeaderParseError)?;

            // completion code is only present on reponse message
            if header.request_bit == 1 && header.completion_code != MctpCompletionCode::Success {
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

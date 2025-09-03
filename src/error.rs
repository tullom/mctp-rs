use crate::{
    endpoint_id::EndpointId, mctp_completion_code::MctpCompletionCode,
    mctp_message_tag::MctpMessageTag, mctp_sequence_number::MctpSequenceNumber,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProtocolError {
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
pub enum MctpPacketError<MediumError: core::fmt::Debug + Copy + Clone + PartialEq + Eq> {
    HeaderParseError(&'static str),
    CommandParseError(&'static str),
    SerializeError(&'static str),
    ProtocolError(ProtocolError),
    MediumError(MediumError),
}

// TODO - MctpPacketResult type alias

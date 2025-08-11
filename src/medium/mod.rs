mod smbus_espi;
mod util;

pub trait MctpMedium: Sized {
    /// the medium specific header and trailer for the packet
    type Frame: MctpMediumFrame<Self>;
    /// the error type for deserialization of the medium specific header
    type Error: core::fmt::Debug + Copy + Clone + PartialEq + Eq;
    // the type used for the data needed to send a reply to a request
    type ReplyContext: core::fmt::Debug + Copy + Clone + PartialEq + Eq;

    /// the maximum transmission unit for the medium
    fn mtu() -> usize;
    /// deserialize the packet into the medium specific header and remainder of the packet -
    /// this includes the mctp transport header, and mctp packet payload
    fn deserialize(packet: &[u8]) -> Result<(Self::Frame, &[u8]), Self::Error>;
    /// serialize the packet into the medium specific header and the payload
    fn serialize<'buf, E, F>(
        frame: Self::ReplyContext,
        buffer: &'buf mut [u8],
        message_writer: F,
    ) -> Result<&'buf [u8], MediumOrGenericError<Self::Error, E>>
    where
        F: for<'a> FnMut(&'a mut [u8]) -> Result<usize, E>;
}

pub enum MediumOrGenericError<M, G> {
    Medium(M),
    Generic(G),
}

pub trait MctpMediumFrame<M: MctpMedium>: Clone + Copy {
    fn packet_size(&self) -> usize;
    fn reply_context(&self) -> M::ReplyContext;
    // serialize the packet into the medium specific header and the payload
    // fn serialize<'buf>(&self, buffer: &'buf mut [u8]) -> Result<&'buf [u8], M::Error>;
    // fn serialize_frame_trailer<'buf>(&self, buffer: &'buf mut [u8])
    // -> Result<&'buf [u8], M::Error>;
}

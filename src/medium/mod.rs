mod smbus_espi;
mod util;

pub trait MctpMedium: Sized {
    /// the medium specific header and trailer for the packet
    type Frame: MctpMediumFrame<Self>;
    /// the error type for deserialization of the medium specific header
    type Error: core::fmt::Debug + Copy + Clone + PartialEq + Eq;
    /// deserialize the packet into the medium specific header and remainder of the packet -
    /// this includes the mctp transport header, and mctp packet payload
    fn deserialize(packet: &[u8]) -> Result<(Self::Frame, &[u8]), Self::Error>;
}

pub trait MctpMediumFrame<M: MctpMedium> {
    fn packet_size(&self) -> usize;
    /// serialize the packet into the medium specific header and the payload
    fn serialize_frame_header<'buf>(&self, buffer: &'buf mut [u8]) -> Result<&'buf [u8], M::Error>;
    fn serialize_frame_trailer<'buf>(&self, buffer: &'buf mut [u8])
    -> Result<&'buf [u8], M::Error>;
}

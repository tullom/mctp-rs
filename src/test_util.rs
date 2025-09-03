use crate::medium::{MctpMedium, MctpMediumFrame, MediumOrGenericError};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TestMedium {
    header: &'static [u8],
    trailer: &'static [u8],
    mtu: usize,
}
impl TestMedium {
    pub fn new() -> Self {
        Self {
            header: &[],
            trailer: &[],
            mtu: 32,
        }
    }
    pub fn with_headers(mut self, header: &'static [u8], trailer: &'static [u8]) -> Self {
        self.header = header;
        self.trailer = trailer;
        self
    }
    pub fn with_mtu(mut self, mtu: usize) -> Self {
        self.mtu = mtu;
        self
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TestMediumFrame(usize);

impl MctpMedium for TestMedium {
    type Frame = TestMediumFrame;
    type Error = &'static str;
    type ReplyContext = ();

    fn deserialize<'buf>(
        &self,
        packet: &'buf [u8],
    ) -> Result<(Self::Frame, &'buf [u8]), Self::Error> {
        let packet_len = packet.len();

        // check that header / trailer is present and correct
        if packet.len() < self.header.len() + self.trailer.len() {
            return Err("packet too short");
        }
        if packet[0..self.header.len()] != *self.header {
            return Err("header mismatch");
        }
        if packet[packet_len - self.trailer.len()..packet_len] != *self.trailer {
            return Err("trailer mismatch");
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
            return Err(MediumOrGenericError::Medium("Buffer too small for headers"));
        }

        // Calculate available space for message (respecting MTU)
        let max_packet_size = self.mtu.min(buffer.len());
        if max_packet_size < header_len + trailer_len {
            return Err(MediumOrGenericError::Medium("MTU too small for headers"));
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

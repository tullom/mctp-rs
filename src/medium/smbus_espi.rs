use crate::medium::{MctpMedium, MctpMediumFrame, MediumOrGenericError, util::Zero};
use bit_register::{NumBytes, TryFromBits, TryIntoBits, bit_register};

struct SmbusEspiMedium;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SmbusEspiReplyContext {
    destination_slave_address: u8,
    source_slave_address: u8,
}

impl MctpMedium for SmbusEspiMedium {
    type Frame = SmbusEspiMediumFrame;
    type Error = &'static str;
    type ReplyContext = SmbusEspiReplyContext;

    fn deserialize<'buf>(
        &self,
        packet: &'buf [u8],
    ) -> Result<(Self::Frame, &'buf [u8]), Self::Error> {
        let header_value = u32::from_be_bytes(
            packet[0..4]
                .try_into()
                .map_err(|_| "Packet too short to parse smbus header")?,
        );
        // strip off the smbus header
        let packet = &packet[4..];
        let header =
            SmbusEspiMediumHeader::try_from(header_value).map_err(|_| "Invalid smbus header")?;
        let pec = packet[header.byte_count as usize];
        // strip off the PEC byte
        let packet = &packet[..header.byte_count as usize];
        Ok((SmbusEspiMediumFrame { header, pec }, packet))
    }

    fn serialize<'buf, E, F>(
        &self,
        reply_context: Self::ReplyContext,
        buffer: &'buf mut [u8],
        message_writer: F,
    ) -> Result<&'buf [u8], MediumOrGenericError<Self::Error, E>>
    where
        F: for<'a> FnOnce(&'a mut [u8]) -> Result<usize, E>,
    {
        // Reserve space for header (4 bytes) and PEC (1 byte)
        if buffer.len() < 5 {
            return Err(MediumOrGenericError::Medium(
                "Buffer too small for smbus frame",
            ));
        }

        // split off a buffer where we will write the header, the rest is for body + PEC
        let (header_slice, body) = buffer.split_at_mut(4);

        // Write the body first
        let body_len = message_writer(body).map_err(MediumOrGenericError::Generic)?;

        // with the body has been written, construct the header
        let header = SmbusEspiMediumHeader {
            destination_slave_address: reply_context.source_slave_address,
            source_slave_address: reply_context.destination_slave_address,
            byte_count: body_len as u8,
            command_code: SmbusCommandCode::MCTP,
            ..Default::default()
        };
        let header_value =
            TryInto::<u32>::try_into(header).map_err(MediumOrGenericError::Medium)?;
        header_slice.copy_from_slice(&header_value.to_be_bytes());

        // with the header written, compute the PEC byte
        let pec_value = smbus_pec::pec(&buffer[0..4 + body_len]);
        buffer[4 + body_len] = pec_value as u8;

        // add 4 for frame header, add 1 for PEC byte
        Ok(&buffer[0..4 + body_len + 1])
    }

    // TODO - this is a guess, need to find the actual value from spec
    fn max_message_body_size(&self) -> usize {
        32
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Default,
)]
enum SmbusCommandCode {
    #[default]
    MCTP = 0x0F,
}
impl TryFromBits<u32> for SmbusCommandCode {
    fn try_from_bits(bits: u32) -> Result<Self, &'static str> {
        if bits > 0xFF {
            Err("Command code out of range")
        } else {
            SmbusCommandCode::try_from(bits as u8).map_err(|_| "Invalid command code")
        }
    }
}
impl TryIntoBits<u32> for SmbusCommandCode {
    fn try_into_bits(self) -> Result<u32, &'static str> {
        Ok(Into::<u8>::into(self) as u32)
    }
}
impl NumBytes for SmbusCommandCode {
    const NUM_BYTES: usize = 1;
}

bit_register! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    struct SmbusEspiMediumHeader: little_endian u32 {
        pub destination_slave_address: u8 => [25:31],
        pub _reserved1: Zero => [24],
        pub command_code: SmbusCommandCode => [16:24],
        pub byte_count: u8 => [8:15],
        pub source_slave_address: u8 => [1:7],
        pub _reserved2: Zero => [0],
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct SmbusEspiMediumFrame {
    header: SmbusEspiMediumHeader,
    pec: u8,
}

impl SmbusEspiReplyContext {
    fn new(frame: SmbusEspiMediumFrame) -> Self {
        Self {
            destination_slave_address: frame.header.destination_slave_address,
            source_slave_address: frame.header.source_slave_address,
        }
    }
}

impl MctpMediumFrame<SmbusEspiMedium> for SmbusEspiMediumFrame {
    fn packet_size(&self) -> usize {
        self.header.byte_count as usize
    }

    fn reply_context(&self) -> SmbusEspiReplyContext {
        SmbusEspiReplyContext::new(*self)
    }

    // fn serialize<'buf, E, F: Fn(&'buf mut [u8]) -> Result<&'buf [u8], E>>(
    //     &self,
    //     buffer: &'buf mut [u8],
    //     transport_serializer: F,
    // ) -> Result<&'buf [u8], <SmbusEspiMedium as MctpMedium>::Error> {
    //     let header_value = TryInto::<u32>::try_into(self.header)
    //         .map_err(|_| "Failed to serialize smbus header")?;
    //     if buffer.len() < 4 {
    //         return Err("Buffer too small to serialize smbus header");
    //     }
    //     buffer[0..4].copy_from_slice(&header_value.to_be_bytes());
    //     Ok(&buffer[4..])
    // }
    // fn serialize_frame_trailer<'buf>(
    //     &self,
    //     buffer: &'buf mut [u8],
    // ) -> Result<&'buf [u8], <SmbusEspiMedium as MctpMedium>::Error> {
    //     // compute PEC byte
    //     let pec = compute_pec
    // }
}

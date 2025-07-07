use bit_register::{NumBytes, TryFromBits, TryIntoBits, bit_register};

use crate::medium::{MctpMedium, MctpMediumFrame, util::Zero};

struct SmbusEspiMedium;

impl MctpMedium for SmbusEspiMedium {
    type Frame = SmbusEspiMediumFrame;
    type Error = &'static str;

    fn deserialize(packet: &[u8]) -> Result<(Self::Frame, &[u8]), Self::Error> {
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
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
enum SmbusCommandCode {
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
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct SmbusEspiMediumHeader: little_endian u32 {
        pub destination_slave_address: u8 => [25:31],
        pub _reserved1: Zero => [24],
        pub command_code: SmbusCommandCode => [16:24],
        pub byte_count: u8 => [8:15],
        pub source_slave_address: u8 => [1:7],
        pub _reserved2: Zero => [0],
    }
}

struct SmbusEspiMediumFrame {
    header: SmbusEspiMediumHeader,
    pec: u8,
}

impl MctpMediumFrame<SmbusEspiMedium> for SmbusEspiMediumFrame {
    fn packet_size(&self) -> usize {
        self.header.byte_count as usize
    }

    fn serialize_frame_header<'buf>(
        &self,
        buffer: &'buf mut [u8],
    ) -> Result<&'buf [u8], <SmbusEspiMedium as MctpMedium>::Error> {
        let header_value = TryInto::<u32>::try_into(self.header)
            .map_err(|_| "Failed to serialize smbus header")?;
        if buffer.len() < 4 {
            return Err("Buffer too small to serialize smbus header");
        }
        buffer[0..4].copy_from_slice(&header_value.to_be_bytes());
        Ok(&buffer[4..])
    }
    fn serialize_frame_trailer<'buf>(
        &self,
        buffer: &'buf mut [u8],
    ) -> Result<&'buf [u8], <SmbusEspiMedium as MctpMedium>::Error> {
        todo!()
    }
}

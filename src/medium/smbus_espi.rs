use crate::medium::{MctpMedium, MctpMediumFrame, MediumOrGenericError, util::Zero};
use bit_register::{NumBytes, TryFromBits, TryIntoBits, bit_register};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SmbusEspiMedium;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SmbusEspiReplyContext {
    pub destination_slave_address: u8,
    pub source_slave_address: u8,
}

impl MctpMedium for SmbusEspiMedium {
    type Frame = SmbusEspiMediumFrame;
    type Error = &'static str;
    type ReplyContext = SmbusEspiReplyContext;

    fn deserialize<'buf>(
        &self,
        packet: &'buf [u8],
    ) -> Result<(Self::Frame, &'buf [u8]), Self::Error> {
        // Check if packet has enough bytes for header
        if packet.len() < 4 {
            return Err("Packet too short to parse smbus header");
        }

        let header_value = u32::from_be_bytes(
            packet[0..4]
                .try_into()
                .map_err(|_| "Packet too short to parse smbus header")?,
        );
        // strip off the smbus header
        let packet = &packet[4..];
        let header =
            SmbusEspiMediumHeader::try_from(header_value).map_err(|_| "Invalid smbus header")?;
        // Weirdly, byte count includes the source slave address
        if header.byte_count as usize + 1 > packet.len() + 1 {
            return Err("Packet too short to parse smbus body and PEC");
        }
        let pec = packet[header.byte_count as usize - 1];
        // strip off the PEC byte
        let packet = &packet[..header.byte_count as usize - 1];
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
            byte_count: body_len as u8 + 1, // + 1 for the source slave address
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

    fn max_message_body_size(&self) -> usize {
        64
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
    #[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
    struct SmbusEspiMediumHeader: little_endian u32 {
        pub destination_slave_address: u8 => [25:31],
        pub _reserved1: u8 => [24],
        pub command_code: SmbusCommandCode => [16:24],
        pub byte_count: u8 => [8:15],
        pub source_slave_address: u8 => [1:7],
        // Actually needs to be 1
        pub _reserved2: u8 => [0],
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SmbusEspiMediumFrame {
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
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use crate::medium::MediumOrGenericError;

    #[test]
    fn test_deserialize_valid_packet() {
        let medium = SmbusEspiMedium;

        // Create a valid SMBus packet with little-endian header
        // destination_slave_address: 0x20, source_slave_address: 0x10, command: 0x0F, byte_count: 4
        let header = SmbusEspiMediumHeader {
            destination_slave_address: 0x20,
            source_slave_address: 0x10,
            command_code: SmbusCommandCode::MCTP,
            byte_count: 4,
            ..Default::default()
        };
        let header_value: u32 = header.try_into().unwrap();
        let header_bytes = header_value.to_be_bytes();

        let payload = [0xAA, 0xBB, 0xCC, 0xDD]; // 4 bytes as specified by byte_count
        let mut combined = [0u8; 8];
        combined[0..4].copy_from_slice(&header_bytes);
        combined[4..8].copy_from_slice(&payload);
        let pec = smbus_pec::pec(&combined);

        let mut packet = [0u8; 9];
        packet[0..4].copy_from_slice(&header_bytes);
        packet[4..8].copy_from_slice(&payload);
        packet[8] = pec as u8;

        let result = medium.deserialize(&packet).unwrap();
        let (frame, body) = result;

        assert_eq!(frame.header.destination_slave_address, 0x20);
        assert_eq!(frame.header.source_slave_address, 0x10);
        assert_eq!(frame.header.command_code, SmbusCommandCode::MCTP);
        assert_eq!(frame.header.byte_count, 4);
        assert_eq!(frame.pec, pec as u8);
        assert_eq!(body, &payload);
    }

    #[test]
    fn test_deserialize_packet_too_short_header() {
        let medium = SmbusEspiMedium;
        let short_packet = [0x01, 0x02]; // Only 2 bytes, need at least 4 for header

        let result = medium.deserialize(&short_packet);
        assert_eq!(result, Err("Packet too short to parse smbus header"));
    }

    #[test]
    fn test_deserialize_packet_too_short_body() {
        let medium = SmbusEspiMedium;

        // Header indicates 10 bytes of data but we only provide 2
        let header_bytes = [
            0x20, // destination_slave_address
            0x0F, // command_code (MCTP)
            0x0A, // byte_count: 10 bytes
            0x20, // source_slave_address
        ];

        let short_payload = [0xAA, 0xBB]; // Only 2 bytes, but header says 10

        let mut packet = [0u8; 6];
        packet[0..4].copy_from_slice(&header_bytes);
        packet[4..6].copy_from_slice(&short_payload);

        let result = medium.deserialize(&packet);
        assert_eq!(result, Err("Packet too short to parse smbus body and PEC"));
    }

    #[test]
    fn test_deserialize_invalid_header() {
        let medium = SmbusEspiMedium;

        // Create invalid header with command code that's not MCTP
        let invalid_header_bytes = [
            0x20, // destination_slave_address
            0xFF, // invalid command_code (not 0x0F)
            0x04, // byte_count
            0x20, // source_slave_address
        ];

        let payload = [0xAA, 0xBB, 0xCC, 0xDD];
        let pec = 0x00; // PEC doesn't matter for this test

        let mut packet = [0u8; 9];
        packet[0..4].copy_from_slice(&invalid_header_bytes);
        packet[4..8].copy_from_slice(&payload);
        packet[8] = pec;

        let result = medium.deserialize(&packet);
        assert_eq!(result, Err("Invalid smbus header"));
    }

    #[test]
    fn test_deserialize_zero_byte_count() {
        let medium = SmbusEspiMedium;

        let header_bytes = [
            0x20, // destination_slave_address
            0x0F, // command_code (MCTP)
            0x00, // byte_count: 0 bytes
            0x20, // source_slave_address
        ];

        let pec = smbus_pec::pec(&header_bytes);

        let mut packet = [0u8; 5];
        packet[0..4].copy_from_slice(&header_bytes);
        packet[4] = pec as u8;

        let result = medium.deserialize(&packet).unwrap();
        let (frame, body) = result;

        assert_eq!(frame.header.byte_count, 0);
        assert_eq!(frame.pec, pec as u8);
        assert_eq!(body.len(), 0);
    }

    #[test]
    fn test_serialize_valid_packet() {
        let medium = SmbusEspiMedium;
        let reply_context = SmbusEspiReplyContext {
            destination_slave_address: 0x20,
            source_slave_address: 0x10,
        };

        let mut buffer = [0u8; 64];
        let test_payload = [0xAA, 0xBB, 0xCC, 0xDD];

        let result = medium
            .serialize(reply_context, &mut buffer, |buf| {
                buf[..test_payload.len()].copy_from_slice(&test_payload);
                Ok::<usize, &'static str>(test_payload.len())
            })
            .unwrap();

        // Verify the serialized packet structure
        // Header: 4 bytes + payload: 4 bytes + PEC: 1 byte = 9 bytes total
        assert_eq!(result.len(), 9);

        // Parse the header to verify correctness
        let header_value = u32::from_be_bytes([result[0], result[1], result[2], result[3]]);
        let header = SmbusEspiMediumHeader::try_from(header_value).unwrap();

        // Note: destination and source are swapped in reply
        assert_eq!(header.destination_slave_address, 0x10); // reply_context.source
        assert_eq!(header.source_slave_address, 0x20); // reply_context.destination
        assert_eq!(header.command_code, SmbusCommandCode::MCTP);
        assert_eq!(header.byte_count, 4);

        // Verify payload
        assert_eq!(&result[4..8], &test_payload);

        // Verify PEC byte
        let expected_pec = smbus_pec::pec(&result[0..8]);
        assert_eq!(result[8], expected_pec as u8);
    }

    #[test]
    fn test_serialize_buffer_too_small() {
        let medium = SmbusEspiMedium;
        let reply_context = SmbusEspiReplyContext {
            destination_slave_address: 0x20,
            source_slave_address: 0x10,
        };

        let mut small_buffer = [0u8; 4]; // Only 4 bytes, need at least 5 (header + PEC)

        let result = medium.serialize(reply_context, &mut small_buffer, |_| {
            Ok::<usize, &'static str>(0)
        });

        assert_eq!(
            result,
            Err(MediumOrGenericError::Medium(
                "Buffer too small for smbus frame"
            ))
        );
    }

    #[test]
    fn test_serialize_minimal_buffer() {
        let medium = SmbusEspiMedium;
        let reply_context = SmbusEspiReplyContext {
            destination_slave_address: 0x20,
            source_slave_address: 0x10,
        };

        let mut minimal_buffer = [0u8; 5]; // Exactly 5 bytes (4 header + 1 PEC)

        let result = medium
            .serialize(
                reply_context,
                &mut minimal_buffer,
                |_| Ok::<usize, &'static str>(0), // No payload data
            )
            .unwrap();

        assert_eq!(result.len(), 5);

        // Verify header
        let header_value = u32::from_be_bytes([result[0], result[1], result[2], result[3]]);
        let header = SmbusEspiMediumHeader::try_from(header_value).unwrap();
        assert_eq!(header.byte_count, 0);

        // Verify PEC
        let expected_pec = smbus_pec::pec(&result[0..4]);
        assert_eq!(result[4], expected_pec as u8);
    }

    #[test]
    fn test_serialize_max_payload() {
        let medium = SmbusEspiMedium;
        let reply_context = SmbusEspiReplyContext {
            destination_slave_address: 0x20,
            source_slave_address: 0x10,
        };

        // Test with maximum payload size (255 bytes as byte_count is u8)
        let max_payload = [0x55u8; 255];
        let mut buffer = [0u8; 260]; // 4 + 255 + 1 = header + max payload + PEC

        let result = medium
            .serialize(reply_context, &mut buffer, |buf| {
                let copy_len = max_payload.len().min(buf.len());
                buf[..copy_len].copy_from_slice(&max_payload[..copy_len]);
                Ok::<usize, &'static str>(copy_len)
            })
            .unwrap();

        assert_eq!(result.len(), 260); // 4 + 255 + 1

        // Verify header
        let header_value = u32::from_be_bytes([result[0], result[1], result[2], result[3]]);
        let header = SmbusEspiMediumHeader::try_from(header_value).unwrap();
        assert_eq!(header.byte_count, 255);

        // Verify payload
        assert_eq!(&result[4..259], &max_payload[..]);

        // Verify PEC
        let expected_pec = smbus_pec::pec(&result[0..259]);
        assert_eq!(result[259], expected_pec as u8);
    }

    #[test]
    fn test_serialize_message_writer_error() {
        let medium = SmbusEspiMedium;
        let reply_context = SmbusEspiReplyContext {
            destination_slave_address: 0x20,
            source_slave_address: 0x10,
        };

        let mut buffer = [0u8; 64];

        let result = medium.serialize(reply_context, &mut buffer, |_| {
            Err::<usize, &'static str>("Test error")
        });

        assert_eq!(result, Err(MediumOrGenericError::Generic("Test error")));
    }

    #[test]
    fn test_roundtrip_serialization_deserialization() {
        let medium = SmbusEspiMedium;
        let original_context = SmbusEspiReplyContext {
            destination_slave_address: 0x42,
            source_slave_address: 0x24,
        };

        let original_payload = [0x11, 0x22, 0x33, 0x44, 0x55];
        let mut buffer = [0u8; 64];

        // Serialize
        let serialized = medium
            .serialize(original_context, &mut buffer, |buf| {
                buf[..original_payload.len()].copy_from_slice(&original_payload);
                Ok::<usize, &'static str>(original_payload.len())
            })
            .unwrap();

        // Deserialize
        let (frame, deserialized_payload) = medium.deserialize(serialized).unwrap();

        // Verify roundtrip correctness
        assert_eq!(deserialized_payload, &original_payload);
        assert_eq!(frame.header.destination_slave_address, 0x24); // swapped
        assert_eq!(frame.header.source_slave_address, 0x42); // swapped
        assert_eq!(frame.header.command_code, SmbusCommandCode::MCTP);
        assert_eq!(frame.header.byte_count, original_payload.len() as u8);

        // Verify PEC is correct
        let expected_pec = smbus_pec::pec(&serialized[0..serialized.len() - 1]);
        assert_eq!(frame.pec, expected_pec as u8);
    }

    #[test]
    fn test_frame_packet_size() {
        let frame = SmbusEspiMediumFrame {
            header: SmbusEspiMediumHeader {
                byte_count: 42,
                ..Default::default()
            },
            pec: 0,
        };

        assert_eq!(frame.packet_size(), 42);
    }

    #[test]
    fn test_frame_reply_context() {
        let frame = SmbusEspiMediumFrame {
            header: SmbusEspiMediumHeader {
                destination_slave_address: 0x30,
                source_slave_address: 0x40,
                ..Default::default()
            },
            pec: 0,
        };

        let context = frame.reply_context();
        assert_eq!(context.destination_slave_address, 0x30);
        assert_eq!(context.source_slave_address, 0x40);
    }

    #[test]
    fn test_smbus_command_code_conversion() {
        // Test valid command code
        assert_eq!(
            SmbusCommandCode::try_from_bits(0x0F).unwrap(),
            SmbusCommandCode::MCTP
        );

        // Test out of range (> 0xFF)
        assert_eq!(
            SmbusCommandCode::try_from_bits(0x100),
            Err("Command code out of range")
        );

        // Test invalid command code
        assert_eq!(
            SmbusCommandCode::try_from_bits(0x10),
            Err("Invalid command code")
        );

        // Test conversion to bits
        assert_eq!(SmbusCommandCode::MCTP.try_into_bits().unwrap(), 0x0F);
    }

    #[test]
    fn test_header_bit_register_edge_cases() {
        // Test all zeros - this should use default command code
        let header = SmbusEspiMediumHeader::default();
        assert_eq!(header.destination_slave_address, 0);
        assert_eq!(header.source_slave_address, 0);
        assert_eq!(header.byte_count, 0);
        assert_eq!(header.command_code, SmbusCommandCode::MCTP); // default

        // Test valid maximum values within bit ranges
        let header = SmbusEspiMediumHeader {
            destination_slave_address: 0x7F, // 7 bits max (bits 25-31)
            source_slave_address: 0x3F,      // 6 bits max (bits 1-7, bit 0 reserved)
            byte_count: 0xFF,                // 8 bits max (bits 8-15)
            command_code: SmbusCommandCode::MCTP,
            ..Default::default()
        };

        // Verify we can convert to u32 and back
        let header_value: u32 = header.try_into().unwrap();
        let reconstructed = SmbusEspiMediumHeader::try_from(header_value).unwrap();
        assert_eq!(reconstructed, header);
    }

    #[test]
    fn test_pec_calculation_accuracy() {
        let medium = SmbusEspiMedium;
        let reply_context = SmbusEspiReplyContext {
            destination_slave_address: 0x50,
            source_slave_address: 0x30,
        };

        // Test with known data to verify PEC calculation
        let test_data = [0x01, 0x02, 0x03];
        let mut buffer = [0u8; 32];

        let result = medium
            .serialize(reply_context, &mut buffer, |buf| {
                buf[..test_data.len()].copy_from_slice(&test_data);
                Ok::<usize, &'static str>(test_data.len())
            })
            .unwrap();

        // Manually calculate expected PEC and compare
        let data_for_pec = &result[0..result.len() - 1];
        let expected_pec = smbus_pec::pec(data_for_pec);
        let actual_pec = result[result.len() - 1];

        assert_eq!(actual_pec, expected_pec as u8);
    }

    #[test]
    fn test_serialize_with_empty_payload() {
        let medium = SmbusEspiMedium;
        let reply_context = SmbusEspiReplyContext {
            destination_slave_address: 0x60,
            source_slave_address: 0x70,
        };

        let mut buffer = [0u8; 16];

        let result = medium
            .serialize(
                reply_context,
                &mut buffer,
                |_| Ok::<usize, &'static str>(0), // Empty payload
            )
            .unwrap();

        assert_eq!(result.len(), 5); // 4 bytes header + 1 byte PEC

        // Verify header
        let header_value = u32::from_be_bytes([result[0], result[1], result[2], result[3]]);
        let header = SmbusEspiMediumHeader::try_from(header_value).unwrap();
        assert_eq!(header.byte_count, 0);
        assert_eq!(header.destination_slave_address, 0x70); // swapped
        assert_eq!(header.source_slave_address, 0x60); // swapped

        // Verify PEC
        let expected_pec = smbus_pec::pec(&result[0..4]);
        assert_eq!(result[4], expected_pec as u8);
    }

    #[test]
    fn test_max_message_body_size() {
        let medium = SmbusEspiMedium;
        assert_eq!(medium.max_message_body_size(), 32);
    }

    #[test]
    fn test_address_swapping_in_reply_context() {
        // Test that addresses are properly swapped when creating reply context
        let original_frame = SmbusEspiMediumFrame {
            header: SmbusEspiMediumHeader {
                destination_slave_address: 0x2A, // Valid 7-bit address
                source_slave_address: 0x3B,      // Valid 6-bit address
                ..Default::default()
            },
            pec: 0,
        };

        let reply_context = SmbusEspiReplyContext::new(original_frame);
        assert_eq!(reply_context.destination_slave_address, 0x2A);
        assert_eq!(reply_context.source_slave_address, 0x3B);

        // Now test that when we serialize with this context, addresses are swapped back
        let medium = SmbusEspiMedium;
        let mut buffer = [0u8; 16];

        let result = medium
            .serialize(reply_context, &mut buffer, |_| Ok::<usize, &'static str>(0))
            .unwrap();

        let header_value = u32::from_be_bytes([result[0], result[1], result[2], result[3]]);
        let response_header = SmbusEspiMediumHeader::try_from(header_value).unwrap();

        // In the response, source becomes destination and vice versa
        assert_eq!(response_header.destination_slave_address, 0x3B);
        assert_eq!(response_header.source_slave_address, 0x2A);
    }

    #[test]
    fn test_deserialize_with_different_byte_counts() {
        let medium = SmbusEspiMedium;

        for byte_count in [1, 16, 32, 64, 128, 255] {
            let header_bytes = [
                0x20,       // destination_slave_address
                0x0F,       // command_code (MCTP)
                byte_count, // byte_count
                0x20,       // source_slave_address
            ];

            let payload = [0x42u8; 255];
            let payload_slice = &payload[..byte_count as usize];

            let mut combined = [0u8; 259]; // 4 header + 255 max payload
            combined[0..4].copy_from_slice(&header_bytes);
            combined[4..4 + byte_count as usize].copy_from_slice(payload_slice);
            let pec = smbus_pec::pec(&combined[0..4 + byte_count as usize]);

            let mut packet = [0u8; 260]; // 4 + 255 + 1
            packet[0..4].copy_from_slice(&header_bytes);
            packet[4..4 + byte_count as usize].copy_from_slice(payload_slice);
            packet[4 + byte_count as usize] = pec as u8;

            let packet_slice = &packet[0..4 + byte_count as usize + 1];
            let result = medium.deserialize(packet_slice).unwrap();
            let (frame, body) = result;

            assert_eq!(frame.header.byte_count, byte_count);
            assert_eq!(body.len(), byte_count as usize);
            assert_eq!(frame.pec, pec as u8);
        }
    }
}

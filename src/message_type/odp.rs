use crate::{
    MctpMedium, MctpMessageHeaderTrait, MctpMessageTrait, MctpPacketError, error::MctpPacketResult,
    mctp_completion_code::MctpCompletionCode,
};

// 5 bits total
#[derive(num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum OdpService {
    Battery = 0x01,
    Debug = 0x02,
}

// 10 bits total
#[derive(num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Debug, PartialEq, Clone, Copy)]
#[repr(u16)]
enum OdpCommandCode {
    BatteryGetBix = 0x01,
}

// 3 byte header
#[derive(Debug, PartialEq, Clone)]
struct OdpHeader {
    pub request_bit: bool,                   // [23:23] (1 bit)
    pub datagram_bit: bool,                  // [22:22] (1 bit)
    pub service: OdpService,                 // [18:21] (4 bits)
    pub command_code: OdpCommandCode,        // [8:17] (10 bits)
    pub completion_code: MctpCompletionCode, // [0:7] (8 bits)
}

#[derive(Debug, PartialEq, Clone)]
enum Odp {
    BatteryGetBixRequest { battery_id: u8 },
}

impl MctpMessageHeaderTrait for OdpHeader {
    fn serialize<M: MctpMedium>(self, buffer: &mut [u8]) -> MctpPacketResult<usize, M> {
        check_header_length(buffer)?;
        let command_code: u16 = self.command_code as u16;
        buffer[0] = (self.request_bit as u8) << 7
            | (self.datagram_bit as u8) << 6
            | ((self.service as u8) & 0b0000_1111) << 2
            | ((command_code >> 8) as u8 & 0b0000_0011);
        buffer[1] = (command_code & 0x00FF) as u8;
        buffer[2] = self.completion_code.into();
        Ok(3)
    }

    fn deserialize<M: MctpMedium>(buffer: &[u8]) -> MctpPacketResult<(Self, &[u8]), M> {
        check_header_length(buffer)?;
        let request_bit = buffer[0] & 0b1000_0000 != 0;
        let datagram_bit = buffer[0] & 0b0100_0000 != 0;
        let service = (buffer[0] & 0b0011_1100) >> 2;
        let command_code = ((buffer[0] & 0b0000_0011) as u16) << 8 | (buffer[1] as u16);

        let completion_code = buffer[2]
            .try_into()
            .map_err(|_| MctpPacketError::HeaderParseError("invalid completion code"))?;
        let service = service
            .try_into()
            .map_err(|_| MctpPacketError::HeaderParseError("invalid odp service"))?;
        let command_code = command_code
            .try_into()
            .map_err(|_| MctpPacketError::HeaderParseError("invalid odp command code"))?;

        Ok((
            OdpHeader {
                request_bit,
                datagram_bit,
                service,
                command_code,
                completion_code,
            },
            &buffer[3..],
        ))
    }
}

impl MctpMessageTrait<'_> for Odp {
    const MESSAGE_TYPE: u8 = 0x7D;
    type Header = OdpHeader;

    fn serialize<M: MctpMedium>(self, buffer: &mut [u8]) -> MctpPacketResult<usize, M> {
        match self {
            Self::BatteryGetBixRequest { battery_id } => write_to_buffer(buffer, [battery_id]),
        }
    }

    fn deserialize<M: MctpMedium>(
        header: &Self::Header,
        buffer: &'_ [u8],
    ) -> MctpPacketResult<Self, M> {
        Ok(match header.command_code {
            OdpCommandCode::BatteryGetBix => Self::BatteryGetBixRequest {
                battery_id: safe_get_u8(buffer, 0)?,
            },
        })
    }
}

fn safe_get_u8<M: MctpMedium>(buffer: &[u8], index: usize) -> MctpPacketResult<u8, M> {
    if buffer.len() < index + 1 {
        return Err(MctpPacketError::HeaderParseError(
            "buffer too small for odp message",
        ));
    }
    Ok(buffer[index])
}

fn write_to_buffer<M: MctpMedium, const N: usize>(
    buffer: &mut [u8],
    data: [u8; N],
) -> MctpPacketResult<usize, M> {
    if buffer.len() < N {
        return Err(MctpPacketError::SerializeError(
            "buffer too small for odp message",
        ));
    }
    buffer[..N].copy_from_slice(&data);
    Ok(N)
}

fn check_header_length<M: MctpMedium>(buffer: &[u8]) -> MctpPacketResult<(), M> {
    if buffer.len() < 3 {
        return Err(MctpPacketError::HeaderParseError(
            "buffer too small for odp header",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::TestMedium;
    use rstest;

    #[rstest::rstest]
    #[case(OdpHeader {
        request_bit: true,
        datagram_bit: false,
        service: OdpService::Battery,
        command_code: OdpCommandCode::BatteryGetBix,
        completion_code: MctpCompletionCode::Success
    })]
    #[case(
        OdpHeader {
        request_bit: false,
        datagram_bit: true,
                service: OdpService::Debug,
        command_code: OdpCommandCode::BatteryGetBix,
        completion_code: MctpCompletionCode::ErrorUnsupportedCmd
    })]
    #[case(
        OdpHeader {
        request_bit: true,
        datagram_bit: true,
        service: OdpService::Battery,
        command_code: OdpCommandCode::BatteryGetBix,
        completion_code: MctpCompletionCode::CommandSpecific(0x80)
    })]
    #[case(
        OdpHeader {
        request_bit: false,
        datagram_bit: false,
        service: OdpService::Debug,
        command_code: OdpCommandCode::BatteryGetBix,
        completion_code: MctpCompletionCode::Success
    })]
    fn odp_header_roundtrip_happy_path(#[case] header: OdpHeader) {
        let mut buf = [0u8; 3];
        let size = header.clone().serialize::<TestMedium>(&mut buf).unwrap();
        assert_eq!(size, 3);

        let (parsed, rest) = OdpHeader::deserialize::<TestMedium>(&buf).unwrap();
        assert_eq!(parsed, header);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn odp_header_error_on_short_buffer() {
        let header = OdpHeader {
            request_bit: false,
            datagram_bit: false,
            service: OdpService::Battery,
            command_code: OdpCommandCode::BatteryGetBix,
            completion_code: MctpCompletionCode::Success,
        };

        // Serialize works with correct buffer
        let mut buf_ok = [0u8; 3];
        header.clone().serialize::<TestMedium>(&mut buf_ok).unwrap();

        // Deserialize should fail on too-small buffer
        let err = OdpHeader::deserialize::<TestMedium>(&buf_ok[..2]).unwrap_err();
        match err {
            MctpPacketError::HeaderParseError(msg) => {
                assert_eq!(msg, "buffer too small for odp header")
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

use bit_register::bit_register;

use crate::{
    control_command::{ControlCommand, ControlCommandRequest, ControlCommandResponse},
    endpoint_id::EndpointId,
    mctp_command_code::MctpCommandCode,
};

#[derive(Debug, PartialEq, Eq)]
pub struct GetEndpointIdRequest;

bit_register! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct GetEndpointIdResponse: little_endian u32 {
        pub endpoint_id: EndpointId => [16:23],
        pub endpoint_type: EndpointType => [12:13],
        pub endpoint_id_type: EndpointIdType => [8:9],
        pub medium_specific: u8 => [0:7],
    }
}

bit_register! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum EndpointType: u8 {
        Simple = 0b00,
        BusOwnerOrBridge = 0b01,
    }
}

bit_register! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum EndpointIdType: u8 {
        Dynamic = 0b00,
        Static = 0b01,
        PresentMatchesStatic = 0b10,
        PresentDoesNotMatchStatic = 0b11,
    }
}

impl ControlCommand for GetEndpointIdRequest {
    const COMMAND_CODE: MctpCommandCode = MctpCommandCode::GetEndpointId;
    fn serialize(self, _: &mut [u8]) -> Result<&[u8], &'static str> {
        Ok(&[])
    }
    fn deserialize(_: &[u8]) -> Result<Self, &'static str> {
        Ok(Self)
    }
}

impl ControlCommand for GetEndpointIdResponse {
    const COMMAND_CODE: MctpCommandCode = MctpCommandCode::GetEndpointId;
    fn serialize(self, buffer: &mut [u8]) -> Result<&[u8], &'static str> {
        let value: u32 = self.try_into()?;
        buffer[0..3].copy_from_slice(&value.to_be_bytes()[1..4]);
        Ok(&buffer[0..3])
    }
    fn deserialize(buffer: &[u8]) -> Result<Self, &'static str> {
        let mut tmp = [0; 4];
        tmp[1..4].copy_from_slice(buffer);
        let value = u32::from_be_bytes(tmp);
        Ok(Self::try_from(value).map_err(|_| "Invalid value")?)
    }
}

impl ControlCommandRequest for GetEndpointIdRequest {
    type Response = GetEndpointIdResponse;
}

impl ControlCommandResponse for GetEndpointIdResponse {
    type Request = GetEndpointIdRequest;
}

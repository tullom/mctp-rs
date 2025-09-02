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
        if buffer.len() < 3 {
            return Err("Buffer too small for GetEndpointIdResponse");
        }
        let value: u32 = self.try_into()?;
        buffer[0..3].copy_from_slice(&value.to_be_bytes()[1..4]);
        Ok(&buffer[0..3])
    }
    fn deserialize(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 3 {
            return Err("Buffer too small for GetEndpointIdResponse");
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_endpoint_id_serialize_buffer_bounds() {
        let response = GetEndpointIdResponse::try_from(0x12345678u32).unwrap();

        // Test with buffer that's too small
        let mut small_buffer = [0u8; 2]; // Need 3 bytes, only have 2
        let result = response.serialize(&mut small_buffer);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_endpoint_id_deserialize_buffer_bounds() {
        // Test with buffer that's too small (less than 3 bytes)
        let small_buffer = [0x12, 0x34]; // Only 2 bytes, need 3
        let result = GetEndpointIdResponse::deserialize(&small_buffer);
        assert!(result.is_err());

        // Test with empty buffer
        let empty_buffer = [];
        let result_empty = GetEndpointIdResponse::deserialize(&empty_buffer);
        assert!(result_empty.is_err());
    }

    #[test]
    fn test_get_endpoint_id_serialize_exact_buffer_size() {
        let response = GetEndpointIdResponse::try_from(0x12345678u32).unwrap();

        // Test with buffer that's exactly the right size
        let mut exact_buffer = [0u8; 3]; // Exactly 3 bytes needed
        let result = response.serialize(&mut exact_buffer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_get_endpoint_id_deserialize_exact_buffer_size() {
        // Test with buffer that's exactly the right size
        // Create a valid GetEndpointIdResponse first and serialize it
        let original_value = 0x12345678u32;
        let original = GetEndpointIdResponse::try_from(original_value).unwrap();
        let mut buffer = [0u8; 3];
        let serialized = original.serialize(&mut buffer).unwrap();

        // Now deserialize it back
        let result = GetEndpointIdResponse::deserialize(serialized);
        assert!(result.is_ok());
        let deserialized = result.unwrap();
        let expected = GetEndpointIdResponse::try_from(original_value).unwrap();
        assert_eq!(deserialized, expected);
    }
}

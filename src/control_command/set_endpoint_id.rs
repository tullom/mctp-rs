use crate::{
    control_command::{ControlCommand, ControlCommandRequest, ControlCommandResponse},
    endpoint_id::EndpointId,
    mctp_command_code::MctpCommandCode,
};

#[derive(Debug, PartialEq, Eq)]
pub struct SetEndpointIdRequest {
    pub operation: SetEndpointIdOperation,
    pub endpoint_id: EndpointId,
}

#[repr(u8)]
#[derive(
    Debug, PartialEq, Eq, Copy, Clone, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
pub enum SetEndpointIdOperation {
    SetEid = 0b00,
    ForceEid = 0b01,
    ResetEid = 0b10,
    SetDiscovered = 0b11,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SetEndpointIdResponse {
    assignment_status: SetEndpointIdAssignmentStatus,
    allocation_status: SetEndpointIdAllocationStatus,
    eid_setting: EndpointId,
    eid_pool_size: u8,
}

#[repr(u8)]
#[derive(
    Debug, PartialEq, Eq, Copy, Clone, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
pub enum SetEndpointIdAssignmentStatus {
    Accepted = 0b00,
    Rejected = 0b01,
}

#[repr(u8)]
#[derive(
    Debug, PartialEq, Eq, Copy, Clone, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
pub enum SetEndpointIdAllocationStatus {
    NoEidPool = 0b00,
    RequiresEidPoolAllocation = 0b01,
    UsesEidPoolAndHasAllocation = 0b10,
}

impl ControlCommand for SetEndpointIdRequest {
    const COMMAND_CODE: MctpCommandCode = MctpCommandCode::SetEndpointId;
    fn serialize(self, _buffer: &mut [u8]) -> Result<&[u8], &'static str> {
        todo!()
    }
    fn deserialize(_buffer: &[u8]) -> Result<Self, &'static str> {
        todo!()
    }
}

impl ControlCommand for SetEndpointIdResponse {
    const COMMAND_CODE: MctpCommandCode = MctpCommandCode::SetEndpointId;
    fn serialize(self, _buffer: &mut [u8]) -> Result<&[u8], &'static str> {
        todo!()
    }

    fn deserialize(_buffer: &[u8]) -> Result<Self, &'static str> {
        todo!()
    }
}

impl ControlCommandRequest for SetEndpointIdRequest {
    type Response = SetEndpointIdResponse;
}

impl ControlCommandResponse for SetEndpointIdResponse {
    type Request = SetEndpointIdRequest;
}

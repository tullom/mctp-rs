use crate::{
    control_command::{ControlCommand, ControlCommandRequest, ControlCommandResponse},
    endpoint_id::EndpointId,
    mctp_command_code::MctpCommandCode,
};

#[derive(Debug, PartialEq, Eq)]
pub struct SetEndpointId;

#[derive(Debug, PartialEq, Eq)]
pub struct SetEndpointIdRequest {
    operation: SetEndpointIdOperation,
    endpoint_id: EndpointId,
}

#[repr(u8)]
#[derive(
    Debug, PartialEq, Eq, Copy, Clone, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
enum SetEndpointIdOperation {
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
enum SetEndpointIdAssignmentStatus {
    Accepted = 0b00,
    Rejected = 0b01,
}

#[repr(u8)]
#[derive(
    Debug, PartialEq, Eq, Copy, Clone, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
enum SetEndpointIdAllocationStatus {
    NoEidPool = 0b00,
    RequiresEidPoolAllocation = 0b01,
    UsesEidPoolAndHasAllocation = 0b10,
}

impl ControlCommand for SetEndpointId {
    type Request = SetEndpointIdRequest;
    type Response = SetEndpointIdResponse;
    const COMMAND_CODE: MctpCommandCode = MctpCommandCode::SetEndpointId;
}

impl ControlCommandRequest for SetEndpointIdRequest {
    fn serialize(self, buffer: &mut [u8]) -> Result<&[u8], &'static str> {
        todo!()
    }

    fn deserialize(buffer: &[u8]) -> Result<Self, &'static str> {
        todo!()
    }
}

impl ControlCommandResponse for SetEndpointIdResponse {
    fn serialize(self, buffer: &mut [u8]) -> Result<&[u8], &'static str> {
        todo!()
    }

    fn deserialize(buffer: &[u8]) -> Result<Self, &'static str> {
        todo!()
    }
}

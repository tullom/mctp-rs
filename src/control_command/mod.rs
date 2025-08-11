mod get_endpoint_id;
mod set_endpoint_id;

use crate::mctp_command_code::MctpCommandCode;

pub trait ControlCommand: Sized {
    const COMMAND_CODE: MctpCommandCode;
    fn serialize(self, buffer: &mut [u8]) -> Result<&[u8], &'static str>;
    fn deserialize(buffer: &[u8]) -> Result<Self, &'static str>;
}

pub trait ControlCommandRequest: ControlCommand {
    type Response: ControlCommandResponse;
}

pub trait ControlCommandResponse: ControlCommand {
    type Request: ControlCommandRequest;
}

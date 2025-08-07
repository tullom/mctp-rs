mod get_endpoint_id;
mod set_endpoint_id;

use core::marker::PhantomData;

pub use get_endpoint_id::*;
pub use set_endpoint_id::*;

use crate::mctp_command_code::MctpCommandCode;

pub trait ControlCommand {
    type Request: ControlCommandRequest;
    type Response: ControlCommandResponse;
    const COMMAND_CODE: MctpCommandCode;
}

#[derive(Debug, PartialEq, Eq)]
pub struct ControlCommandBuffer<'buf, C: ControlCommand> {
    buffer: &'buf mut [u8],
    _phantom: PhantomData<C>,
}

impl<'buf, C: ControlCommand> ControlCommandBuffer<'buf, C> {
    pub fn new(buffer: &'buf mut [u8]) -> Self {
        Self {
            buffer,
            _phantom: PhantomData,
        }
    }
}

pub trait ControlCommandRequest: Sized {
    fn serialize(self, buffer: &mut [u8]) -> Result<&[u8], &'static str>;
    fn deserialize(buffer: &[u8]) -> Result<Self, &'static str>;
}

pub trait ControlCommandResponse: Sized {
    fn serialize(self, buffer: &mut [u8]) -> Result<&[u8], &'static str>;
    fn deserialize(buffer: &[u8]) -> Result<Self, &'static str>;
}

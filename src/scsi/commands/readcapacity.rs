
use crate::{AumsError};
use scsi::commands::{Command, CommmandBlockWrapper, Direction};
use traits::{Buffer, BufferPullable, BufferPushable};


pub struct ReadCapacityCommand {}

impl Command for ReadCapacityCommand {
    fn opcode() -> u8 {
        0x25
    }
    fn length() -> u8 {
        0x10
    }
    fn wrapper(&self) -> CommmandBlockWrapper {
        CommmandBlockWrapper::new(0x8, Direction::IN, 0, ReadCapacityCommand::length())
    }
}

impl BufferPushable for ReadCapacityCommand {
    fn push_to_buffer<B : Buffer>(&self, buffer: &mut B) -> Result<usize, AumsError> {
        let mut rval = self.wrapper().push_to_buffer(buffer)?;
        rval += buffer.push_byte(ReadCapacityCommand::opcode())?;
        Ok(rval)
    }
}

pub struct ReadCapacityResponse {
    logical_block_address : u32, 
    block_length : u32
}

impl BufferPullable for ReadCapacityResponse {
    fn pull_from_buffer<B : Buffer>(buffer: &mut B) -> Result<ReadCapacityResponse, AumsError> {
        let lba_bytes = u32::pull_from_buffer(buffer)?.swap_bytes();
        let len_bytes = u32::pull_from_buffer(buffer)?.swap_bytes();
        Ok(ReadCapacityResponse {
            logical_block_address : lba_bytes, 
            block_length : len_bytes,
        })
    }
}
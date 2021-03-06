pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

use crate::processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, input) {
        // catch the error so we can print it
        return Err(error);
    }
    Ok(())
}

// pubkey= GpGUwJfyTrmkLVTiJY6QD6dPFde4qgdehd3m4mF7p53D


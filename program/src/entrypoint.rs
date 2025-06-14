use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::entrypoint;
use crate::processor::launch_asset;
use crate::state::LaunchpadInstruction;

// Program entrypoint
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = LaunchpadInstruction::try_from_slice(instruction_data).map_err(
        |_| ProgramError::InvalidInstructionData
    )?;

    match instruction {
        LaunchpadInstruction::LaunchAsset { config } => {
            launch_asset(program_id, accounts, config)
        }
    }
}

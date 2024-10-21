mod create;
mod mint;

use create::*;
use mint::*;

use steel::*;
use steel_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Mint => process_mint(accounts, data)?,
        SteelInstruction::Create => process_create(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

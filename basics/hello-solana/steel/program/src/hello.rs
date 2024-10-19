use solana_program::msg;
use steel::*;

pub fn process_hello(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;

    msg!("Hello, Solana!");

    msg!("Our program's Program ID: {}", &steel_api::ID);

    Ok(())
}

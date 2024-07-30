use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use spl_token::{instruction::transfer, state::Account as SplAccount};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Event {
    pub user_pubkey: Pubkey,
    pub amount: u64,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    let source_token_account = next_account_info(accounts_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let event: Event = Event::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let transfer_instruction = transfer(
        token_program_account.key,
        source_token_account.key,
        destination_token_account.key,
        payer_account.key,
        &[],
        event.amount,
    )?;

    invoke(
        &transfer_instruction,
        &[
            source_token_account.clone(),
            destination_token_account.clone(),
            payer_account.clone(),
            token_program_account.clone(),
        ],
    )?;

    msg!("Transferred {} $TRIB tokens to {}", event.amount, event.user_pubkey);
    Ok(())
}

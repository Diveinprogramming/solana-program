use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::Instruction;

pub struct Deposit<'info> {
    pub token_swap: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,

    pub source_a: AccountInfo<'info>,

    pub source_b: AccountInfo<'info>,

    pub into_a: AccountInfo<'info>,

    pub into_b: AccountInfo<'info>,

    pub pool_token: AccountInfo<'info>,

    pub pool_account: AccountInfo<'info>,
    pub mint_a: AccountInfo<'info>,
    pub mint_b: AccountInfo<'info>,
    pub token_program_id_a: AccountInfo<'info>,
    pub token_program_id_b: AccountInfo<'info>,
    pub pool_token_program_id: AccountInfo<'info>,
    pub swap_program_id: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DepositInstruction {
    pub instruction: u8,
    //2
    pub pool_token_amount: u64,
    pub maximum_token_a: u64,
    pub maximum_token_b: u64,
}

pub fn deposit_all_token_types_instruction(
    accounts: &Deposit,
    args: &DepositInstruction,
) -> Instruction {
    let data = args.try_to_vec().unwrap();
    Instruction {
        program_id: *accounts.swap_program_id.key,
        accounts: vec![
            AccountMeta::new_readonly(*accounts.token_swap.key, false),
            AccountMeta::new_readonly(*accounts.authority.key, false),
            AccountMeta::new_readonly(*accounts.user_transfer_authority.key, true),
            AccountMeta::new(*accounts.source_a.key, false),
            AccountMeta::new(*accounts.source_b.key, false),
            AccountMeta::new(*accounts.into_a.key, false),
            AccountMeta::new(*accounts.into_b.key, false),
            AccountMeta::new(*accounts.pool_token.key, false),
            AccountMeta::new(*accounts.pool_account.key, false),
            AccountMeta::new_readonly(*accounts.mint_a.key, false),
            AccountMeta::new_readonly(*accounts.mint_b.key, false),
            AccountMeta::new_readonly(*accounts.token_program_id_a.key, false),
            AccountMeta::new_readonly(*accounts.token_program_id_b.key, false),
            AccountMeta::new_readonly(*accounts.pool_token_program_id.key, false),
        ],
        data,
    }
}

pub fn deposit_all_token_types(accounts: &Deposit, args: &DepositInstruction) -> Result<()> {
    let cpi_instruction = deposit_all_token_types_instruction(accounts, args);
    solana_program::program::invoke(
        &cpi_instruction,
        &[
            accounts.token_swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_a.clone(),
            accounts.source_b.clone(),
            accounts.into_a.clone(),
            accounts.into_b.clone(),
            accounts.pool_token.clone(),
            accounts.pool_account.clone(),
            accounts.mint_a.clone(),
            accounts.mint_b.clone(),
            accounts.token_program_id_a.clone(),
            accounts.token_program_id_b.clone(),
            accounts.pool_token_program_id.clone(),
            accounts.swap_program_id.clone(),
        ],
    )?;

    Ok(())
}

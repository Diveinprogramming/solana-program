use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use solana_program::instruction::Instruction;
use borsh::{BorshDeserialize, BorshSerialize};

pub struct Withdraw<'info> {
    pub token_swap: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,

    pub pool_mint: AccountInfo<'info>,

    pub fee_account: AccountInfo<'info>,

    pub source_pool_account: AccountInfo<'info>,

    pub from_a: AccountInfo<'info>,

    pub from_b: AccountInfo<'info>,

    pub user_account_a: AccountInfo<'info>,

    pub user_account_b: AccountInfo<'info>,
    pub mint_a: AccountInfo<'info>,
    pub mint_b: AccountInfo<'info>,
    pub pool_token_program_id: AccountInfo<'info>,

    pub token_program_id_a: AccountInfo<'info>,
    pub token_program_id_b: AccountInfo<'info>,
    pub swap_program_id: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WithdrawInstruction {
    pub instruction: u8,
    pub pool_token_amount: u64,
    pub minimum_token_a: u64,
    pub minimum_token_b: u64,
}

pub fn withdraw_all_token_types_instruction(accounts: &Withdraw, args: &WithdrawInstruction) -> Instruction {
    let data = args.try_to_vec().unwrap();

    Instruction {
        program_id: *accounts.swap_program_id.key,
        accounts: vec![
            AccountMeta::new_readonly(*accounts.token_swap.key, false),
            AccountMeta::new_readonly(*accounts.authority.key, false),
            AccountMeta::new_readonly(*accounts.user_transfer_authority.key, true),
            AccountMeta::new(*accounts.pool_mint.key, false),
            AccountMeta::new(*accounts.source_pool_account.key, false),
            AccountMeta::new(*accounts.from_a.key, false),
            AccountMeta::new(*accounts.from_b.key, false),
            AccountMeta::new(*accounts.user_account_a.key, false),
            AccountMeta::new(*accounts.user_account_b.key, false),
            AccountMeta::new(*accounts.fee_account.key, false),
            AccountMeta::new_readonly(*accounts.mint_a.key, false),
            AccountMeta::new_readonly(*accounts.mint_b.key, false),
            AccountMeta::new_readonly(*accounts.pool_token_program_id.key, false),
            AccountMeta::new_readonly(*accounts.token_program_id_a.key, false),
            AccountMeta::new_readonly(*accounts.token_program_id_b.key, false),
        ],
        data,
    }
}

pub fn withdraw_all_token_types(accounts: &Withdraw, args: &WithdrawInstruction) -> Result<()> {
    let cpi_instruction = withdraw_all_token_types_instruction(accounts,args);
    solana_program::program::invoke(&cpi_instruction, &[
        accounts.token_swap.clone(),
        accounts.authority.clone(),
        accounts.user_transfer_authority.clone(),
        accounts.pool_mint.clone(),
        accounts.source_pool_account.clone(),
        accounts.from_a.clone(),
        accounts.from_b.clone(),
        accounts.user_account_a.clone(),
        accounts.user_account_b.clone(),
        accounts.fee_account.clone(),
        accounts.mint_a.clone(),
        accounts.mint_b.clone(),
        accounts.pool_token_program_id.clone(),
        accounts.token_program_id_a.clone(),
        accounts.token_program_id_b.clone(),
    ])?;

    Ok(())
}

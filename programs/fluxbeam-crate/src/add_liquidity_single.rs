use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::Instruction;

pub struct DepositSingle<'info> {
    pub token_swap: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,

    pub source: AccountInfo<'info>,

    pub into_a: AccountInfo<'info>,

    pub into_b: AccountInfo<'info>,

    pub pool_token: AccountInfo<'info>,

    pub pool_account: AccountInfo<'info>,
    pub source_mint: AccountInfo<'info>,
    pub source_token_program_id: AccountInfo<'info>,
    pub pool_token_program_id: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DepositSingleInstruction {
    pub source_token_amount: u64,
    pub minimum_pool_token_amount: u64,
}

pub fn deposit_single_token_type_exact_amount_in_instruction(
    accounts: &DepositSingle,
    args: &DepositSingleInstruction,
) -> Instruction {
    let mut data = vec![4]; // depositSingleTokenTypeExactAmountIn instruction
    data.append(&mut args.try_to_vec().unwrap());

    Instruction {
        program_id: *accounts.token_swap.key,
        accounts: vec![
            AccountMeta::new(*accounts.token_swap.key, false),
            AccountMeta::new(*accounts.authority.key, false),
            AccountMeta::new(*accounts.user_transfer_authority.key, true),
            AccountMeta::new(*accounts.source.key, true),
            AccountMeta::new(*accounts.into_a.key, true),
            AccountMeta::new(*accounts.into_b.key, true),
            AccountMeta::new(*accounts.pool_token.key, true),
            AccountMeta::new(*accounts.pool_account.key, true),
            AccountMeta::new(*accounts.source_mint.key, false),
            AccountMeta::new(*accounts.source_token_program_id.key, false),
            AccountMeta::new(*accounts.pool_token_program_id.key, false),
        ],
        data,
    }
}


pub fn deposit_single_token_type_exact_amount_in(
    accounts: &DepositSingle,
    args: &DepositSingleInstruction,
) -> Result<()> {


    let cpi_instruction = deposit_single_token_type_exact_amount_in_instruction(accounts, args);
    solana_program::program::invoke(&cpi_instruction, &[
        accounts.token_swap.clone(),
        accounts.authority.clone(),
        accounts.user_transfer_authority.clone(),
        accounts.source.clone(),
        accounts.into_a.clone(),
        accounts.into_b.clone(),
        accounts.pool_token.clone(),
        accounts.pool_account.clone(),
        accounts.source_mint.clone(),
        accounts.source_token_program_id.clone(),
        accounts.pool_token_program_id.clone(),
    ])?;

    Ok(())
}
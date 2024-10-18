use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use solana_program::instruction::Instruction;
use borsh::{BorshDeserialize, BorshSerialize};

pub struct WithdrawSingle<'info> {

    pub token_swap: AccountInfo<'info>,

    pub authority: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,

    pub pool_mint: AccountInfo<'info>,

    pub source_pool_account: AccountInfo<'info>,

    pub from_a: AccountInfo<'info>,

    pub from_b: AccountInfo<'info>,

    pub user_account: AccountInfo<'info>,

    pub fee_account: AccountInfo<'info>,
    pub destination_mint: AccountInfo<'info>,
    pub pool_token_program_id: AccountInfo<'info>,
    pub destination_token_program_id: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WithdrawSingleInstruction {
    pub destination_token_amount: u64,
    pub maximum_pool_token_amount: u64,
}


pub fn withdraw_single_token_type_exact_amount_out_instruction(
    accounts: &WithdrawSingle,
    args: &WithdrawSingleInstruction,
) -> Instruction {
    let mut data = vec![5]; // withdrawSingleTokenTypeExactAmountOut instruction
    data.append(&mut args.try_to_vec().unwrap());

    Instruction {
        program_id: *accounts.token_swap.key,
        accounts: vec![
            AccountMeta::new(*accounts.token_swap.key, false),
            AccountMeta::new(*accounts.authority.key, false),
            AccountMeta::new(*accounts.user_transfer_authority.key, true),
            AccountMeta::new(*accounts.pool_mint.key, true),
            AccountMeta::new(*accounts.source_pool_account.key, true),
            AccountMeta::new(*accounts.from_a.key, true),
            AccountMeta::new(*accounts.from_b.key, true),
            AccountMeta::new(*accounts.user_account.key, true),
            AccountMeta::new(*accounts.fee_account.key, true),
            AccountMeta::new(*accounts.destination_mint.key, false),
            AccountMeta::new(*accounts.pool_token_program_id.key, false),
            AccountMeta::new(*accounts.destination_token_program_id.key, false),
        ],
        data,
    }
}


pub fn withdraw_single_token_type_exact_amount_out(
    accounts: &WithdrawSingle,
    args: &WithdrawSingleInstruction,
) -> Result<()> {
    let cpi_instruction = withdraw_single_token_type_exact_amount_out_instruction(accounts,args);

    solana_program::program::invoke(&cpi_instruction, &[
        accounts.token_swap.clone(),
        accounts.authority.clone(),
        accounts.user_transfer_authority.clone(),
        accounts.pool_mint.clone(),
        accounts.source_pool_account.clone(),
        accounts.from_a.clone(),
        accounts.from_b.clone(),
        accounts.user_account.clone(),
        accounts.fee_account.clone(),
        accounts.destination_mint.clone(),
        accounts.pool_token_program_id.clone(),
        accounts.destination_token_program_id.clone(),
    ])?;

    Ok(())
}

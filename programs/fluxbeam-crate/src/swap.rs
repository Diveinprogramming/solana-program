use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::Instruction;

pub struct CreateSwap<'info> {
    pub token_swap: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,

    pub user_source: AccountInfo<'info>,

    pub pool_source: AccountInfo<'info>,

    pub pool_destination: AccountInfo<'info>,

    pub user_destination: AccountInfo<'info>,

    pub pool_mint: AccountInfo<'info>,

    pub fee_account: AccountInfo<'info>,
    pub host_fee_account: Option<AccountInfo<'info>>,
    pub source_mint: AccountInfo<'info>,
    pub destination_mint: AccountInfo<'info>,

    pub source_token_program_id: AccountInfo<'info>,
    pub destination_token_program_id: AccountInfo<'info>,
    pub pool_token_program_id: AccountInfo<'info>,
    pub swap_program_id: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateSwapInstruction {
    pub instruction: u8,
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

pub fn create_swap_instruction(
    accounts: &CreateSwap,
    args: &CreateSwapInstruction,
) -> Instruction {
    let data = args.try_to_vec().unwrap();

    Instruction {
        program_id: *accounts.swap_program_id.key,
        accounts: vec![
            AccountMeta::new_readonly(*accounts.token_swap.key, false),
            AccountMeta::new_readonly(*accounts.authority.key, false),
            AccountMeta::new_readonly(*accounts.user_transfer_authority.key, true),
            AccountMeta::new(*accounts.user_source.key, false),
            AccountMeta::new(*accounts.pool_source.key, false),
            AccountMeta::new(*accounts.pool_destination.key, false),
            AccountMeta::new(*accounts.user_destination.key, false),
            AccountMeta::new(*accounts.pool_mint.key, false),
            AccountMeta::new(*accounts.fee_account.key, false),
            AccountMeta::new_readonly(*accounts.source_mint.key, false),
            AccountMeta::new_readonly(*accounts.destination_mint.key, false),
            AccountMeta::new_readonly(*accounts.source_token_program_id.key, false),
            AccountMeta::new_readonly(*accounts.destination_token_program_id.key, false),
            AccountMeta::new_readonly(*accounts.pool_token_program_id.key, false),
        ],
        data,
    }
}

pub fn create_swap(
    accounts: &CreateSwap,
    args: &CreateSwapInstruction,
) -> Result<()> {
    let cpi_instruction = create_swap_instruction(accounts, args);

    solana_program::program::invoke(&cpi_instruction, &[
        accounts.token_swap.clone(),
        accounts.authority.clone(),
        accounts.user_transfer_authority.clone(),
        accounts.user_source.clone(),
        accounts.pool_source.clone(),
        accounts.pool_destination.clone(),
        accounts.user_destination.clone(),
        accounts.pool_mint.clone(),
        accounts.fee_account.clone(),
        accounts.source_mint.clone(),
        accounts.destination_mint.clone(),
        accounts.source_token_program_id.clone(),
        accounts.destination_token_program_id.clone(),
        accounts.pool_token_program_id.clone(),
    ])?;

    Ok(())
}

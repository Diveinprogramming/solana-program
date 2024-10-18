use anchor_lang::prelude::*;

#[program]
pub mod manage_fee {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_fee: u64) -> Result<()> {
        ctx.accounts.fee_account.fee = initial_fee;
        Ok(())
    }

    pub fn update_fee(ctx: Context<UpdateFee>, new_fee: u64) -> Result<()> {
        ctx.accounts.fee_account.fee = new_fee;
        Ok(())
    }

    pub fn get_fee(ctx: Context<GetFee>) -> Result<u64> {
        Ok(ctx.accounts.fee_account.fee)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 8, seeds = [b"fee_account"], bump)]
    pub fee_account: Account<'info, FeeAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    #[account(mut, seeds = [b"fee_account"], bump)]
    pub fee_account: Account<'info, FeeAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetFee<'info> {
    #[account(seeds = [b"fee_account"], bump)]
    pub fee_account: Account<'info, FeeAccount>,
}

#[account]
pub struct FeeAccount {
    pub fee: u64,
}

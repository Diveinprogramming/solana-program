use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeAccount, InitializeMint, Mint, Token, TokenAccount};

#[program]
pub mod token_create {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, decimals: u8) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.decimals = decimals;
        mint.is_initialized = true;

        // Deduct 0.10 SOL from the payer's balance
        let fee = 100000000; // 0.10 SOL in lamports
        **ctx.accounts.payer.try_borrow_mut_lamports()? -= fee;

        Ok(())
    }

    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        token_account.amount = 0; // Initial balance
        token_account.mint = ctx.accounts.mint.key();
        token_account.owner = ctx.accounts.owner.key();

        // Deduct 0.10 SOL from the payer's balance
        let fee = 100000000; // 0.10 SOL in lamports
        **ctx.accounts.payer.try_borrow_mut_lamports()? -= fee;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 1)] // Adjust space as needed
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>, // User's wallet
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 32)] // Adjust space for TokenAccount
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>, // User's wallet
    pub system_program: Program<'info, System>,
}

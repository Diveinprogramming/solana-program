use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[program]
pub mod mint_token {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>, decimals: u8) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.decimals = decimals;
        mint.is_initialized = true;
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        token::mint_to(ctx.accounts.mint_to_context(), amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(init, payer = authority, space = 8 + 32)] // Adjust space for Mint account
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, constraint = mint.key() == token_account.mint)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}

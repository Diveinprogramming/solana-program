use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[program]
pub mod token_airdrop {
    use super::*;

    pub fn airdrop_tokens(ctx: Context<AirdropTokens>, amount: u64) -> Result<()> {
        let fee: u64 = (0.10 * LAMPORTS_PER_SOL as f64) as u64;

        **ctx
            .accounts
            .authority
            .to_account_info()
            .try_borrow_mut_lamports()? -= fee;
        **ctx
            .accounts
            .fee_receiver
            .to_account_info()
            .try_borrow_mut_lamports()? += fee;

        let cpi_accounts = Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AirdropTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub fee_receiver: SystemAccount<'info>,
    #[account(mut, constraint = token_account.mint == mint.key(), constraint = token_account.owner == authority.key())]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, constraint = recipient_token_account.mint == mint.key())]
    pub recipient_token_account: Box<Account<'info, TokenAccount>>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

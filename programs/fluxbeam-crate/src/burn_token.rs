use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

#[program]
pub mod token_burn {
    use super::*;

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
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

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        token::burn(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub fee_receiver: SystemAccount<'info>,
    #[account(mut, constraint = token_account.mint == mint.key(), constraint = token_account.owner == authority.key())]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

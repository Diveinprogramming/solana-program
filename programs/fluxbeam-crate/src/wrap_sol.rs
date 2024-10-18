use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[program]
pub mod wrap_unwrap_sol {
    use super::*;

    pub fn wrap_sol(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.wsol_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn unwrap_sol(ctx: Context<UnwrapSol>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.wsol_account.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        **ctx.accounts.payer.try_borrow_mut_lamports()? += amount;
        **ctx.accounts.wsol_account.try_borrow_mut_lamports()? -= amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WrapSol<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub wsol_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UnwrapSol<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub wsol_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

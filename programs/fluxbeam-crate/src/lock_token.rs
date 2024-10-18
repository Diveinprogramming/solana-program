use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[program]
pub mod token_lock {
    use super::*;

    pub fn lock_tokens(ctx: Context<LockTokens>, amount: u64, lock_duration: i64) -> Result<()> {
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
            to: ctx.accounts.locked_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        ctx.accounts.lock_info.locked_until = Clock::get()?.unix_timestamp + lock_duration;

        Ok(())
    }

    pub fn unlock_tokens(ctx: Context<UnlockTokens>) -> Result<()> {
        let clock = Clock::get()?;
        if clock.unix_timestamp < ctx.accounts.lock_info.locked_until {
            return Err(ErrorCode::TokensStillLocked.into());
        }

        let cpi_accounts = Transfer {
            from: ctx.accounts.locked_token_account.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.locked_account_authority.clone(),
        };

        let seeds = &[
            b"locked",
            ctx.accounts.authority.key().as_ref(),
            &[ctx.accounts.lock_info.bump],
        ];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &[&seeds[..]],
        );
        token::transfer(cpi_ctx, ctx.accounts.locked_token_account.amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub fee_receiver: SystemAccount<'info>,
    #[account(mut, constraint = token_account.mint == mint.key(), constraint = token_account.owner == authority.key())]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"locked", authority.key().as_ref()],
        bump,
    )]
    pub locked_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub lock_info: Account<'info, LockInfo>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub lock_info: Account<'info, LockInfo>,
    #[account(mut)]
    pub locked_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, constraint = token_account.owner == authority.key())]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [b"locked", authority.key().as_ref()],
        bump = lock_info.bump,
    )]
    pub locked_account_authority: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LockInfo {
    pub locked_until: i64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Tokens are still locked.")]
    TokensStillLocked,
}

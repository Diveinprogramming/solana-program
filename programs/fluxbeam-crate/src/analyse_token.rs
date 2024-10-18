use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

#[program]
pub mod token_rug_checker {
    use super::*;

    pub fn analyze_token(ctx: Context<AnalyzeToken>) -> Result<()> {
        let fee = 0.10 * LAMPORTS_PER_SOL;
        **ctx
            .accounts
            .user
            .to_account_info()
            .try_borrow_mut_lamports()? -= fee as u64;
        **ctx
            .accounts
            .fee_receiver
            .to_account_info()
            .try_borrow_mut_lamports()? += fee as u64;

        if let Some(mint_authority) = ctx.accounts.mint.mint_authority {
            msg!("Mint authority: {}", mint_authority);
        } else {
            msg!("No mint authority.");
        }

        if let Some(freeze_authority) = ctx.accounts.mint.freeze_authority {
            msg!("Freeze authority: {}", freeze_authority);
        } else {
            msg!("No freeze authority.");
        }

        msg!("Total supply: {}", ctx.accounts.mint.supply);
        msg!("Account balance: {}", ctx.accounts.token_account.amount);

        if ctx.accounts.token_account.amount > (ctx.accounts.mint.supply / 10) {
            msg!("Warning: Account holds >10% of total supply.");
        } else {
            msg!("Token distribution looks reasonable.");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AnalyzeToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub fee_receiver: SystemAccount<'info>,
    pub mint: Account<'info, Mint>,
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

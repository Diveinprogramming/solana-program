use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

#[program]
pub mod profile_tool {
    use super::*;

    pub fn get_wallet_tokens(ctx: Context<GetWalletTokens>) -> Result<ProfileData> {
        let wallet = ctx.accounts.wallet.key();
        let token_accounts = &ctx.accounts.token_accounts;

        let mut tokens = Vec::new();
        let mut total_balance = 0.0;

        for token_account in token_accounts.iter() {
            let mint = &token_account.mint;
            let amount = token_account.amount;

            let price = get_token_price(mint)?;
            let total = (amount as f64) * price;

            tokens.push(TokenData {
                token: mint.to_string(),
                amount,
                price,
                total,
            });

            total_balance += total;
        }

        Ok(ProfileData {
            wallet: wallet.to_string(),
            tokens,
            total_balance,
        })
    }

    fn get_token_price(mint: &Pubkey) -> Result<f64> {
        Ok(1.0) // Placeholder for actual price fetching logic
    }
}

#[derive(Accounts)]
pub struct GetWalletTokens<'info> {
    pub wallet: Signer<'info>,
    #[account(mut)]
    pub token_accounts: Vec<Account<'info, TokenAccount>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenData {
    pub token: String,
    pub amount: u64,
    pub price: f64,
    pub total: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ProfileData {
    pub wallet: String,
    pub tokens: Vec<TokenData>,
    pub total_balance: f64,
}

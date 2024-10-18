use anchor_lang::prelude::*;

#[program]
pub mod manage_meta_data {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, metadata: String) -> Result<()> {
        ctx.accounts.meta_data_account.metadata = metadata;
        Ok(())
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>, new_metadata: String) -> Result<()> {
        ctx.accounts.meta_data_account.metadata = new_metadata;
        Ok(())
    }

    pub fn get_metadata(ctx: Context<GetMetadata>) -> Result<String> {
        Ok(ctx.accounts.meta_data_account.metadata.clone())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 128)] // Adjust space as needed
    pub meta_data_account: Account<'info, MetaDataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(mut)]
    pub meta_data_account: Account<'info, MetaDataAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetMetadata<'info> {
    #[account()]
    pub meta_data_account: Account<'info, MetaDataAccount>,
}

#[account]
pub struct MetaDataAccount {
    pub metadata: String,
}

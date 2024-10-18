use anchor_lang::prelude::*;

declare_id!("FJJpctqWehRo9GPggYwwjjJQunJe2kcv4JUQmDwbvaZx");

#[program]
pub mod fluxbeam_crate {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

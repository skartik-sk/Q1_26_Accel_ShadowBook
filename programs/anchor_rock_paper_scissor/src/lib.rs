use anchor_lang::prelude::*;

declare_id!("Hq2m9e8mDn95yfaeEn864uUD2uV4S8JYFj4sDWinZPpC");

#[program]
pub mod anchor_rock_paper_scissor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

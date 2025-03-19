use anchor_lang::prelude::*;

declare_id!("H56ZcsSHhx3bPSeYoAkPeuhsz21yKfLf6asvvuUd4u3u");

#[program]
pub mod drift_v1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("Do3SKatkSNF3FjKHpeMthHqTE5JfAJzTCrCC8xNBnkGP");

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

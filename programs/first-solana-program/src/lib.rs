use anchor_lang::prelude::*;

declare_id!("EfCEDNysDrrtY5StyS93txA6PmvGDZ1Fns9xfitnGY24");

#[program]
pub mod first_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

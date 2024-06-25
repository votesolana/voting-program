use anchor_lang::prelude::*;

declare_id!("6GBHN1ddUDKtf7kaJ5BwoHzVPxXDMKaATS4WuXe4TFAg");

#[program]
pub mod vote_program_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

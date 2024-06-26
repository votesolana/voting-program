use anchor_lang::prelude::*;

use anchor_spl::{
    token::{self, Mint, Token, TokenAccount}
};

declare_id!("6GBHN1ddUDKtf7kaJ5BwoHzVPxXDMKaATS4WuXe4TFAg");

pub mod constants {
    pub const TREASURY_SEED: &[u8] = b"vote_vault";
    pub const VOTE_INFO_SEED: &[u8] = b"voting_info";
    pub const TOKEN_WALLET_SEED: &[u8] = b"";
}

#[program]
pub mod vote_program_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn vote(ctx: Context<Initialize>, amount: u64) -> Result<()> { //interval (macro of time staked 1day, 7day, erc), wifTremp (boolean)
        Ok(())
    }

    pub fn collect_vote(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::TREASURY_SEED],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = treasury_account,
    )]
    pub treasury_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, Token>,
}


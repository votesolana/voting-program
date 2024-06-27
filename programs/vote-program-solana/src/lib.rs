use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, transfer}
};

use solana_program::clock::Clock;

declare_id!("8N5acTMm4h1F8gz3n88PddeC6QU4HbhvfMBvGZJZxAo3");

pub mod constants {
    pub const TREASURY_SEED: &[u8] = b"vote_vaulttremp";
    pub const VOTE_INFO_SEED: &[u8] = b"votewiftremp_info";
    pub const TOKEN_WALLET_SEED: &[u8] = b"votewiftremptoken";
}

#[program]
pub mod vote_program_solana {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, amount: u64) -> Result<()> {
        let vote_info_account = &mut ctx.accounts.vote_info_account;

        if vote_info_account.is_voted {
            return Err(ErrorCode::IsVoted.into());
        }

        if amount < 100 {
            return Err(ErrorCode::TooLow.into());
        }

        let clock = Clock::get()?;

        vote_info_account.voted_at_slot = clock.slot;
        vote_info_account.is_voted = true;

        let vote_amount = (amount)
            .checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
            .unwrap();
                //1.000000
        transfer(
            CpiContext::new(
              ctx.accounts.token_program.to_account_info(),
              Transfer {
                from:ctx.accounts.user_votewiftremp_account.to_account_info(),
                to: ctx.accounts.vote_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
              }
            ),
            vote_amount, //+reward pool amount and transfer from reward pool to vote_account
        )?;
        

        Ok(())
    }

    pub fn collect_vote(ctx: Context<CollectVote>) -> Result<()> {
        let vote_info_account = &mut ctx.accounts.vote_info_account;

        if !vote_info_account.is_voted{
            return Err(ErrorCode::NotVoted.into());
        }

        let clock = Clock::get()?;
        let _slots_pass = clock.slot - vote_info_account.voted_at_slot;

        let vote_amount = ctx.accounts.vote_account.amount; 

       // let bump = *ctx.bumps.get("treasury_account").unwrap();
       //  let signer: &[&[&[u8]]] = &[&[constants::TREASURY_SEED, &[bump]]];
 

        //use this function when vote placed to send treasury funds to the user wallet
        /*transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info();
                Transfer{
                    from: ctx.accounts.treasury_account.to_account_info(),
                    to: ctx.accounts.user_votewiftremp_account.to_account_info(),
                    authority: ctx.accounts.treasury_account.to_account_info(),
                },
                signer
            ), 
            stake_amount
        )?; */

        let voter = ctx.accounts.signer.key();
        let bump = ctx.bumps.vote_account;
        let signer: &[&[&[u8]]] = &[&[constants::TOKEN_WALLET_SEED, voter.as_ref(), &[bump]]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.vote_account.to_account_info(),
                    to: ctx.accounts.user_votewiftremp_account.to_account_info(),
                    authority: ctx.accounts.vote_account.to_account_info(),
                },
                signer
            ), 
            vote_amount,
        )?;

        vote_info_account.is_voted = false;
        vote_info_account.voted_at_slot = clock.slot;

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
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::VOTE_INFO_SEED, signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<VoteInfo>(),
    )]
    pub vote_info_account: Account<'info, VoteInfo>,

    #[account(
        init_if_needed,
        seeds = [constants::TOKEN_WALLET_SEED, signer.key().as_ref()],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = vote_account
    )]
    pub vote_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_votewiftremp_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CollectVote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::TREASURY_SEED],
        bump,
    )]
    pub treasury_account: Account<'info, TokenAccount>,


    #[account(
        mut,
        seeds = [constants::TOKEN_WALLET_SEED, signer.key().as_ref()],
        bump,
    )]
    pub vote_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [constants::VOTE_INFO_SEED, signer.key().as_ref()],
        bump,
    )]
    pub vote_info_account: Account<'info, VoteInfo>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_votewiftremp_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}



#[account]
pub struct VoteInfo {
    pub voted_at_slot: u64, // Exact time slot vote is placed
    pub is_voted: bool,
    pub wif_tremp: bool, // Voted for Tremp or Boden
}

#[error_code]
pub enum ErrorCode {
    #[msg("You have already voted")]
    IsVoted,
    #[msg("You don't have a vote to claim")]
    NotVoted,
    #[msg("You must wait longer to claim")]
    TimeLocked,
    #[msg("You need at least 100 tokens to vote")]
    TooLow,
}

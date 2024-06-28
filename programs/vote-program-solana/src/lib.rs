use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount, Transfer, transfer },
};

use solana_program::clock::Clock;

declare_id!("DdtYyWsnynW3iNQA9wt1gVxQweisgAVWJUEYhfJTe95x");

pub mod constants {
    pub const TREASURY_SEED: &[u8] = b"vote_vaulttremp";
    pub const VOTE_INFO_SEED: &[u8] = b"votewiftremp_info";
    pub const TOKEN_WALLET_SEED: &[u8] = b"votewiftremptoken";
    pub const GLOBAL_VOTE_SEED: &[u8] = b"votewiftrempglobal";
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TimeLength {
    OneMinute,
    Medium,
    Long,
    VeryLong,
}

#[program]
pub mod vote_program_solana {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn vote(
        ctx: Context<Vote>,
        amount: u64,
        vote_for_tremp: bool,
        timelength: TimeLength
    ) -> Result<()> {
        match timelength {
            TimeLength::OneMinute => {
                // handle short time length logic
            }
            TimeLength::Medium => {
                // handle medium time length logic
            }
            TimeLength::Long => {
                // handle long time length logic
            }
            TimeLength::VeryLong => {
                // handle very long time length logic
            }
            //add error for other and say wrong time length
        }
        let vote_info_account = &mut ctx.accounts.vote_info_account;
        let global_vote_account = &mut ctx.accounts.global_vote_account;


        if vote_info_account.is_voted {
            return Err(ErrorCode::IsVoted.into());
        }
        
        if amount < 100 {
            return Err(ErrorCode::TooLow.into());
        }

        let clock = Clock::get()?;

        vote_info_account.voted_at_slot = clock.slot;
        vote_info_account.is_voted = true;
        vote_info_account.wif_tremp = vote_for_tremp;
        vote_info_account.vote_amount=amount;

        let vote_amount = amount
            .checked_mul((10u64).pow(ctx.accounts.mint.decimals as u32))
            .unwrap();
        //1.000000

        let rewards = ((vote_amount as f64) * 0.2) as u64;

        //do a check to see if rewards are less than amount in reward wallet and return error code if so

        let bump = ctx.bumps.treasury_account;
        let signer: &[&[&[u8]]] = &[&[constants::TREASURY_SEED, &[bump]]];

        //transfer from treasury to vote account
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.treasury_account.to_account_info(),
                    to: ctx.accounts.vote_account.to_account_info(),
                    authority: ctx.accounts.treasury_account.to_account_info(),
                },
                signer
            ),
            rewards
        )?;

        //transfer for user wallet to vote account
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                from: ctx.accounts.user_votewiftremp_account.to_account_info(),
                to: ctx.accounts.vote_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            }),
            vote_amount //+reward pool amount and transfer from reward pool to vote_account
        )?;

        if vote_for_tremp {
            global_vote_account.tremp += amount;
        } else {
            global_vote_account.boden += amount;
        }

        // ALCULATE REWARD BASED ON SECOND FUNCTION PARAMETER TIME LENGTH, MULTIPLY REWARD RATE BY AMOUNT AND SEND REWARDS FROM REWARD WALLET TO VOTE_ACCOUNT ALONG WITH tokens
        //FROM VOTEWIFTREMP USER Account

        Ok(())
    }

    pub fn collect_vote(ctx: Context<CollectVote>) -> Result<()> {
        let vote_info_account = &mut ctx.accounts.vote_info_account;
        let global_vote_account = &mut ctx.accounts.global_vote_account;

        if !vote_info_account.is_voted {
            return Err(ErrorCode::NotVoted.into());
        }

        let clock = Clock::get()?;
        let _slots_pass = clock.slot - vote_info_account.voted_at_slot;

        let vote_amount = ctx.accounts.vote_account.amount;

        let voter = ctx.accounts.signer.key();
        let bump = ctx.bumps.vote_account;
        let signer: &[&[&[u8]]] = &[&[constants::TOKEN_WALLET_SEED, voter.as_ref(), &[bump]]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vote_account.to_account_info(),
                    to: ctx.accounts.user_votewiftremp_account.to_account_info(),
                    authority: ctx.accounts.vote_account.to_account_info(),
                },
                signer
            ),
            vote_amount
        )?;

        vote_info_account.is_voted = false;
        vote_info_account.voted_at_slot = clock.slot;
  


        if vote_info_account.wif_tremp {
            global_vote_account.tremp -=  vote_info_account.vote_amount;
        } else {
            global_vote_account.boden -=  vote_info_account.vote_amount;
        }

        vote_info_account.vote_amount = 0;

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
        token::authority = treasury_account
    )]
    pub treasury_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [constants::GLOBAL_VOTE_SEED],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<GlobalVotes>()
    )]
    pub global_vote_account: Account<'info, GlobalVotes>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::GLOBAL_VOTE_SEED],
        bump,
    )]
    pub global_vote_account: Box<Account<'info, GlobalVotes>>,

    #[account(
        init_if_needed,
        seeds = [constants::VOTE_INFO_SEED, signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<VoteInfo>()
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
    pub vote_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_votewiftremp_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [constants::TREASURY_SEED],
        bump,
    )]
    pub treasury_account: Box<Account<'info, TokenAccount>>,

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
        seeds = [constants::GLOBAL_VOTE_SEED],
        bump,
    )]
    pub global_vote_account: Box<Account<'info, GlobalVotes>>,

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
    pub vote_info_account: Box<Account<'info, VoteInfo>>,

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
    pub wif_tremp: bool,
    pub vote_amount: u64,// Voted for Tremp or Boden
}

#[account]
pub struct GlobalVotes {
    pub tremp: u64,
    pub boden: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You have already voted")]
    IsVoted,
    #[msg("You don't have a vote to claim")]
    NotVoted,
    #[msg("You must wait longer to claim")]
    TimeLocked,
    #[msg("Not enough vote token")]
    TooLow,
    #[msg("Not enough tokens in reward pool to place this vote")]
    RewardPoolLow,
    #[msg("This program has already been deployed")]
    ProgramAlreadyInitialized,
}

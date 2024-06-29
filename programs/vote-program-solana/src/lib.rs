use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount, Transfer, transfer },
};

use solana_program::clock::Clock;

declare_id!("HTKTfrxwcr6F41Ajf3sbG9gnytuMdGnd4TiCfDumceMM");

pub mod constants {
    pub const TREASURY_SEED: &[u8] = b"vote_vaulttremp";
    pub const VOTE_INFO_SEED: &[u8] = b"votewiftremp_info";
    pub const TOKEN_WALLET_SEED: &[u8] = b"votewiftremptoken";
    pub const GLOBAL_VOTE_SEED: &[u8] = b"votewiftrempglobal";
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TimeLength {
    OneMinute,
    OneDay,
    OneMonth,
    FiveMonths,
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
        let length_in_seconds_locked: i64;
        let mut reward_rate_denominator: f64 = 0.2;
        match timelength {
            TimeLength::OneMinute => {
                length_in_seconds_locked = 60;
                reward_rate_denominator =
                    reward_rate_denominator / (5.0 * 30.0 * 24.0 * 60.0 * 1.05); //5 months 30 days 24 hours 60 second timeLengthBuffer
            }
            TimeLength::OneDay => {
                length_in_seconds_locked = 24 * 60 * 60; // 1 day in seconds
                reward_rate_denominator = reward_rate_denominator / (5.0 * 30.0 * 1.03);
            }
            TimeLength::OneMonth => {
                length_in_seconds_locked = 30 * 24 * 60 * 60;
                reward_rate_denominator = reward_rate_denominator / (5.0 * 1.02);
            }
            TimeLength::FiveMonths => {
                length_in_seconds_locked = 5 * 30 * 24 * 60 * 60;
            }
        }

        msg!("Selected time length in seconds: {}", length_in_seconds_locked);
        msg!("reward rate denom: {}", reward_rate_denominator);
        let vote_info_account = &mut ctx.accounts.vote_info_account;
        let global_vote_account = &mut ctx.accounts.global_vote_account;

        if vote_info_account.is_voted {
            return Err(ErrorCode::IsVoted.into());
        }

        if amount < 5000 {
            return Err(ErrorCode::TooLow.into());
        }

        let clock = Clock::get()?;
        msg!("seconds: {}", clock.unix_timestamp);

        vote_info_account.vote_locked_until = clock.unix_timestamp + length_in_seconds_locked;
        vote_info_account.is_voted = true;
        vote_info_account.wif_tremp = vote_for_tremp;
        vote_info_account.vote_amount = amount;

        let vote_amount = amount
            .checked_mul((10u64).pow(ctx.accounts.mint.decimals as u32))
            .unwrap();
        //1.000000

        let rewards = ((vote_amount as f64) * reward_rate_denominator) as u64;

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
        let clock_current_time = clock.unix_timestamp;

        if clock_current_time < vote_info_account.vote_locked_until {
            return Err(ErrorCode::TimeLocked.into());
        }

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

        if vote_info_account.wif_tremp {
            global_vote_account.tremp = global_vote_account.tremp
                .checked_sub(vote_info_account.vote_amount)
                .unwrap_or(0);
        } else {
            global_vote_account.boden = global_vote_account.boden
                .checked_sub(vote_info_account.vote_amount)
                .unwrap_or(0);
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
    pub vote_locked_until: i64, // Exact time slot vote is placed
    pub is_voted: bool,
    pub wif_tremp: bool,
    pub vote_amount: u64, // Voted for Tremp or Boden
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
    #[msg("Your votes are still locked. You must wait longer")]
    TimeLocked,
    #[msg("Not enough vote token")]
    TooLow,
    #[msg("Not enough tokens in reward pool to place this vote")]
    RewardPoolLow,
    #[msg("This program has already been deployed")]
    ProgramAlreadyInitialized,
    #[msg("Invalid Time Length")]
    InvalidTimeLength,
}

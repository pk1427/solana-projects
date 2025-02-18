use anchor_lang::prelude::*;

declare_id!("BcgVBWBcGagSLs1TigHrPnKDMWfqT8AJnK81Kiqppjjb");

#[program]
pub mod voting_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let voting_state = &mut ctx.accounts.voting_state;
        voting_state.option1_votes = 0;
        voting_state.option2_votes = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, option: u8) -> Result<()> {
        let voting_state = &mut ctx.accounts.voting_state;
        if option == 1 {
            voting_state.option1_votes += 1;
        } else if option == 2 {
            voting_state.option2_votes += 1;
        } else {
            return Err(ErrorCode::InvalidOption.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8)]
    pub voting_state: Account<'info, VotingState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voting_state: Account<'info, VotingState>,
}

#[account]
pub struct VotingState {
    pub option1_votes: u64,
    pub option2_votes: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid voting option.")]
    InvalidOption,
}

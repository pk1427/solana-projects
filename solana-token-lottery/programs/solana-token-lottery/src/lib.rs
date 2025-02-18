use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::system_program;
declare_id!("4q3SyytkXxnKbzY2CnDauNUuYcsdeY3633ConNN5eTRL");


#[program]
pub mod solana_token_lottery {
    use super::*;

    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        ctx.accounts.lottery_account.total_pool = 0;
        Ok(())
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        // Perform token transfer
        token::transfer(ctx.accounts.into_transfer_context(), amount)?;

        // Update lottery pool
        ctx.accounts.lottery_account.total_pool += amount;
        Ok(())
    }

    pub fn pick_winner(ctx: Context<PickWinner>) -> Result<()> {
        let amount = ctx.accounts.lottery_account.total_pool;

        // Transfer winnings to the winner
        token::transfer(ctx.accounts.into_transfer_context(), amount)?;

        // Reset lottery pool
        ctx.accounts.lottery_account.total_pool = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub lottery_account: Account<'info, Lottery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub lottery_account: Account<'info, Lottery>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lottery_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct PickWinner<'info> {
    #[account(mut)]
    pub lottery_account: Account<'info, Lottery>,
    #[account(mut)]
    pub winner: Signer<'info>,
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lottery_token_account: Account<'info, TokenAccount>,
    pub lottery_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Lottery {
    pub total_pool: u64,
}

impl<'info> DepositTokens<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.lottery_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> PickWinner<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.lottery_token_account.to_account_info(),
            to: self.winner_token_account.to_account_info(),
            authority: self.lottery_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

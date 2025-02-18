use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("AL4DFTE9jWm15PThQachf5mJkNu8gvhFtLcakadBEfPa");

#[program]
pub mod solana_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, token::TokenAccount>, // ✅ Corrected
    #[account(mut)]
    pub pool_token_account: Account<'info, token::TokenAccount>, // ✅ Corrected
    pub token_program: Program<'info, Token>,
}

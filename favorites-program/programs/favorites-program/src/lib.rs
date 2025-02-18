use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxqSW9dWqyxzp6VbfnnF2hZStZJ");

#[program]
pub mod favorites_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.favorite_account;
        account.items = vec![];
        Ok(())
    }

    pub fn add_favorite(ctx: Context<AddFavorite>, item: String) -> Result<()> {
        let account = &mut ctx.accounts.favorite_account;
        account.items.push(item);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 500)]
    pub favorite_account: Account<'info, FavoriteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddFavorite<'info> {
    #[account(mut)]
    pub favorite_account: Account<'info, FavoriteAccount>,
}

#[account]
pub struct FavoriteAccount {
    pub items: Vec<String>,
}

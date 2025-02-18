use anchor_lang::prelude::*;

declare_id!("4q3SyytkXxnKbzY2CnDauNUuYcsdeY3633ConNN5eTRL");

#[program]
pub mod solana_blinks_actions {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let action_state = &mut ctx.accounts.action_state;
        action_state.counter = 0;
        Ok(())
    }

    pub fn execute_action(ctx: Context<ExecuteAction>, amount: u64) -> Result<()> {
        let action_state = &mut ctx.accounts.action_state;
        action_state.counter += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub action_state: Account<'info, ActionState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteAction<'info> {
    #[account(mut)]
    pub action_state: Account<'info, ActionState>,
}

#[account]
pub struct ActionState {
    pub counter: u64,
}

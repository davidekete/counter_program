use anchor_lang::prelude::*;

declare_id!("5hcfuuJTzCW1ySkhECrm78AVBYZSp8udXnZhGrjx6cgy");

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>, data: u64) -> Result<()> {
        ctx.accounts.counter_account.counter = data;
        ctx.accounts.counter_account.update_count = ctx
            .accounts
            .counter_account
            .update_count
            .checked_add(1)
            .unwrap();

        ctx.accounts.counter_account.authority = ctx.accounts.signer.key();

        msg!(
            "Counter value initialized to {}
        Current update count: {}
        ",
            data,
            ctx.accounts.counter_account.update_count
        );

        Ok(())
    }

    pub fn increment_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        ctx.accounts.counter_account.counter =
            ctx.accounts.counter_account.counter.checked_add(1).unwrap();
        ctx.accounts.counter_account.update_count = ctx
            .accounts
            .counter_account
            .update_count
            .checked_add(1)
            .unwrap();

        Ok(())
    }

    pub fn decrement_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        ctx.accounts.counter_account.counter =
            ctx.accounts.counter_account.counter.checked_sub(1).unwrap();
        ctx.accounts.counter_account.update_count = ctx
            .accounts
            .counter_account
            .update_count
            .checked_add(1)
            .unwrap();
        Ok(())
    }

    pub fn set_counter(ctx: Context<SetCounter>, data: u64) -> Result<()> {
        ctx.accounts.counter_account.counter = data;
        ctx.accounts.counter_account.update_count = ctx
            .accounts
            .counter_account
            .update_count
            .checked_add(1)
            .unwrap();
        Ok(())
    }
}

//Interface (list of accounts required by a program)
#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = signer, space = 8 + CounterState::INIT_SPACE)]
    pub counter_account: Account<'info, CounterState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterState>,
}

#[derive(Accounts)]
pub struct SetCounter<'info> {
    #[account(mut, has_one = authority)]
    pub counter_account: Account<'info, CounterState>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

//STATE
#[account]
#[derive(InitSpace)]
pub struct CounterState {
    pub counter: u64,
    pub update_count: u64,
    pub authority: Pubkey,
}

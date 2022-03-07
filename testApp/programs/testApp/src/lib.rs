use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod test_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn do_pay(ctx: Context<Initialize>, lamports: u64) -> Result<()>
    {
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    data: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>

}

use anchor_lang::prelude::*;
//use anchor_spl::token::{self, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod test_app {
    use super::*;

    //const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn do_msg(ctx: Context<Empty>) -> Result<()>
    {
        msg!("do msg");
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()>
    {
        ctx.accounts.admin.authority = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn do_pay(ctx: Context<PayerList>, lamports: u64) -> Result<()>
    {
        anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &id(),
            lamports
        );
        ctx.accounts.payer_accounts.lst_accounts.push(ctx.accounts.user.key());
        Ok(())
    }

    pub fn get_admin_monay(ctx: Context<AdminMoney>) -> Result<()>
    {
        anchor_lang::solana_program::system_instruction::transfer(
            &id(),
            &ctx.accounts.admin.key(),
            2
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Empty {
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32
    )]
    pub admin: Account<'info, AdminAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AdminMoney<'info> {
    #[account(mut, has_one = authority)]
    pub admin: Account<'info, AdminAccount>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct PayerList<'info> {
    //pub admin: Account<'info, AdminAccount>,
    #[account(mut)]
    pub payer_accounts: Account<'info, PayersAccount>,
    #[account(mut, has_one = authority)]
    pub user: Account<'info, UserAccount>,
    pub authority: Signer<'info>
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey
}

#[account]
pub struct AdminAccount {
    pub authority: Pubkey
}

#[account]
pub struct PayersAccount {
    pub lst_accounts: Vec<Pubkey>
}
use anchor_lang::prelude::*;
use chainlink_solana as chainlink;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod te1 {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn execute(ctx: Context<Exectute>) -> ProgramResult {
        let round = chainlink::latest_round_data(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed.to_account_info())?;

        let result_account = &mut ctx.accounts.result_account;
        result_account.value = round.answer;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Exectute<'info> {
    #[account(init, payer=user, space=100)]
    let result_account = Account<'info, ResultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub chainlink_program: Account<'info>,
    pub chainlink_feed: Acount<'info>

}

#[account]
pub struct ResultAccount<'info> {
    pub value: i128
}
use anchor_lang::prelude::*;

declare_id!("FKBWhshzcQa29cCyaXc1vfkZ5U985gD5YsqfCzJYUBr");

#[program]
pub mod fake_metadata {
    use super::*;

    pub fn create_metadata(ctx: Context<CreateMetadata>) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        metadata.character = ctx.accounts.character.key();
        metadata.health = u8::MAX;
        metadata.power = u8::MAX;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    /// CHECK: character passed manually
    pub character: AccountInfo<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 1,
        seeds = [character.key().as_ref()],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Metadata {
    pub character: Pubkey,
    pub health: u8,
    pub power: u8,
}
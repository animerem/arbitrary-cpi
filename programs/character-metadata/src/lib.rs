use anchor_lang::prelude::*;

declare_id!("D4hPnYEsAx4u3EQMrKEXsY3MkfLndXbBKTEYTwwm25TE");

#[program]
pub mod character_metadata {
    use super::*;

    pub fn create_metadata(ctx: Context<CreateMetadata>) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;
        let clock = Clock::get()?;

        let random_health = pseudo_random(clock, 20);
        let random_power = pseudo_random(clock, 20);

        metadata.character = ctx.accounts.character.key();
        metadata.health = random_health;
        metadata.power = random_power;

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
        space = 8 + 32 + 1 + 1, // correct sizing: Pubkey + u8 + u8
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

fn pseudo_random(clock: Clock, limit: u8) -> u8 {
    let seed = clock.unix_timestamp.unsigned_abs(); // Prevent negative remainders
    (seed % limit as u64) as u8
}
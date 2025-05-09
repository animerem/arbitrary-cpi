use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use character_metadata::cpi::accounts::CreateMetadata;

declare_id!("AbFFYMjsZ2iaXn6wU9C8BDDJS8yP3bE9tEndB56cn3yE");

#[program]
pub mod gameplay {
    use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
    use super::*;

    pub fn create_character_insecure(ctx: Context<CreateCharacterInsecure>) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.metadata = ctx.accounts.metadata_account.key();
        character.auth = ctx.accounts.authority.key();
        character.wins = 0;

        let signer_seeds = &[ctx.accounts.authority.key().as_ref(), &[ctx.bumps["character"]]];

        let create_metadata_ix = Instruction {
            program_id: ctx.accounts.metadata_program.key(),
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.character.key(), false),
                AccountMeta::new(ctx.accounts.metadata_account.key(), false),
                AccountMeta::new(ctx.accounts.authority.key(), true),
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
            ],
            data: anchor_sighash("create_metadata").to_vec(),
        };

        invoke_signed(
            &create_metadata_ix,
            &[
                ctx.accounts.character.to_account_info(),
                ctx.accounts.metadata_account.clone(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[signer_seeds],
        )?;

        Ok(())
    }

    pub fn battle_insecure(ctx: Context<BattleInsecure>) -> Result<()> {
        let player_one_meta = Metadata::try_from_slice(&ctx.accounts.player_one_metadata.try_borrow_data()?)?;
        let player_two_meta = Metadata::try_from_slice(&ctx.accounts.player_two_metadata.try_borrow_data()?)?;

        let p1_health = player_one_meta.health.saturating_sub(player_two_meta.power);
        let p2_health = player_two_meta.health.saturating_sub(player_one_meta.power);

        if p1_health > p2_health {
            ctx.accounts.player_one.wins += 1;
        } else {
            ctx.accounts.player_two.wins += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCharacterInsecure<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8,
        seeds = [authority.key().as_ref()],
        bump
    )]
    pub character: Account<'info, Character>,
    #[account(
        mut,
        seeds = [character.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump
    )]
    /// CHECK: unchecked for demonstration purposes
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: intentionally unchecked
    pub metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BattleInsecure<'info> {
    pub player_one: Account<'info, Character>,
    pub player_two: Account<'info, Character>,
    /// CHECK: unchecked metadata
    pub player_one_metadata: UncheckedAccount<'info>,
    /// CHECK: unchecked metadata
    pub player_two_metadata: UncheckedAccount<'info>,
    /// CHECK: unchecked
    pub metadata_program: UncheckedAccount<'info>,
}

#[account]
pub struct Character {
    pub auth: Pubkey,
    pub metadata: Pubkey,
    pub wins: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Metadata {
    pub character: Pubkey,
    pub health: u8,
    pub power: u8,
}

fn anchor_sighash(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}

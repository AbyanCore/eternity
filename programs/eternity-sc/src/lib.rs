use anchor_lang::prelude::*;

mod events;
mod state;
mod instructions;
mod utils;

use instructions::{
    personality::*,
    relic::*,
    vault::*, 
    fragments::*
};

declare_id!("4zEC44NxzuFsrQ3ZVXqnrP1UghjL29ARnY7a6xaz2WZb");

const TOKEN_LAMPORT: u64 = 10;

#[program]
pub mod eternity_sc {
    use super::*;

    // PERSONALITY INSTRUCTION
    pub fn create_personality(ctx: Context<CreatePersonality>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
        create_personality_handler(ctx, name, age, hobbie, message)
    }
    pub fn update_personality(ctx: Context<ManagePersonality>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
        update_personality_handler(ctx, name, age, hobbie, message)
    }

    // PERSONALITY MICRO INSTRUCTION
    pub fn m_set_personality_message(ctx: Context<ManagePersonality>,message: String) -> Result<()> {
        m_set_personality_message_handler(ctx, message)
    }
    pub fn m_set_personality_hobbie(ctx: Context<ManagePersonality>,hobbie: Vec<String>) -> Result<()> {
        m_set_personality_hobbie_handler(ctx, hobbie)
    }

    // RELIC INSTRUCTION
    pub fn create_relic(ctx: Context<CreateRelic>,relic_id: u32,name: String, description: String) -> Result<()> {
        create_relic_handler(ctx, relic_id, name, description)
    }
    pub fn update_relic(ctx: Context<ManageRelic>,relic_id: u32,name: String, description: String, visibillity: bool) -> Result<()> {
        update_relic_handler(ctx, relic_id, name, description, visibillity)
    }
    
    // RELIC MICRO INSTRUCTION
    pub fn m_set_relic_description(ctx: Context<ManageRelic>, relic_id: u32, description: String) -> Result<()> {
        m_set_relic_description_handler(ctx, relic_id, description)
    }
    pub fn m_set_relic_heir(ctx: Context<ManageRelic>, relic_id: u32, heir: Pubkey) -> Result<()> {
        m_set_relic_heir_handler(ctx, relic_id, heir)
    }

    // FRAGMENTS INSTRUCTION
    pub fn create_fragments(ctx: Context<CreateFragments>,relic_id: u32,fragment_id: u32) -> Result<()> {
        create_fragments_handler(ctx, relic_id, fragment_id)
    }

    // FRAGMENTS MICRO INSTRUCTION
    pub fn m_add_fragment(ctx: Context<ManageFragments>,relic_id: u32,fragment_id: u32, key: [u8; 32]) -> Result<()> {
        m_add_fragment_handler(ctx, relic_id, fragment_id, key)
    }
    pub fn m_update_fragment(ctx: Context<ManageFragments>,relic_id: u32,fragment_id: u32, id: u16, key: [u8; 32]) -> Result<()> {
        m_update_fragment_handler(ctx, relic_id, fragment_id, id, key)
    }
    pub fn m_delete_fragment(ctx: Context<ManageFragments>,relic_id: u32,fragment_id: u32, id: u16) -> Result<()> {
        m_delete_fragment_handler(ctx, relic_id, fragment_id, id)
    }

    // VAULT INSTRUCTION
    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        create_vault_handler(ctx)
    }

    // VAULT MICRO INSTRUCTION
    pub fn m_buy_token(ctx: Context<ManageVault>,amount: u64) -> Result<()> {
        m_buy_token_handler(ctx, amount)
    }
    pub fn m_take_token(ctx: Context<ManageVault>, amount: u64) -> Result<()> {
        m_take_token_handler(ctx, amount)
    }

}
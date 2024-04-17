pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7VRhge12SRGVNCsJNuS2d6YgHhDQuD8Kt84Z5KtxQYP");

#[program]
pub mod thobaby_token {
    use super::*;

    pub fn initialize(ctx: Context<InitToken>, metadata: Metadata) -> Result<()> {
        init_token::handler(ctx, metadata)
    }
}

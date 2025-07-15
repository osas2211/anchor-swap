pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FJMGnEKVQqcZbVHcXLJaFCPw6KZpv4N7kRh3u4GhkZ26");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&ctx, 67)?;
        instructions::make_offer::save_offer(ctx, 0, 50)?;
        Ok(())
    }
}

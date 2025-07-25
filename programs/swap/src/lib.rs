pub mod constants;
pub mod error;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use state::*;
pub use instructions::*;

declare_id!("72pLLY8evuK84qXQWWxJj7EgGfPSBHznY9NEBwXH1LnJ");

#[program]
pub mod swap{
    use super::*;

    pub fn make_offer(
        context : Context<MakeOffer>,
        id : u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount : u64
    ) -> Result<()> {
        instructions::make_offer::send_offered_token_to_vault(&context, token_a_offered_amount)?;
        instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(&context)?;
        instructions::take_offer::withdraw_and_close_vault(context)
    }
}
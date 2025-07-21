pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Eo9M2qhnJuRkZZsqGKhY9bxue7kSzowu5ppzgo5c2GWC");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}

// escrow is a neutral third party that holds assets until certain conditions are met. 
// ex:- bank, lawyer; here we replace it with code.
// three instructions - make, take, refund
// make - creates an escrow account, deposits their tokens into the vault, and specify what they want in return.
// take - lets the other party accept those terms, deposit their tokens and recieve whats in the vault.
// refund - original creator cancel and reclain their tokens.
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub use instructions::*;
pub mod state;

declare_id!("D5mwBZPWdVHiJ2hCg4QPfxfRpA1wtSfi9T24pcpg9wWD");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        // if i'm the maker all i'm doing is creating a new escrow, 
        // defining the info that needs to be in that escrow, 
        // and depositing the tokens into the vault.
        instructions::make::handler(ctx, seed, receive, amount)
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::handler(ctx)
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}
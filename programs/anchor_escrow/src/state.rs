// going to define the custom state of every account we will create.
use anchor_lang::prelude::*;
 
#[derive(InitSpace)]
#[account(discriminator = 1)]
  pub struct Escrow {
  pub seed: u64,
  // the person who makes the escrow contract.
  pub maker: Pubkey,
  // mint account for two different tokens.
  pub  mint_a: Pubkey,
  pub mint_b: Pubkey,
  // the person who will recieve the tokens in the vault.
  pub receive: u64,
  pub bump: u8,
}
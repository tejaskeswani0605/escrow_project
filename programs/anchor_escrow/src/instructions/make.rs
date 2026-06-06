use crate::errors::EscrowError;
use crate::state::Escrow;
use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
#[derive(Accounts)]
#[instruction(seed: u64)] // Hey i'm using this variable seeds but dont have access to this, 
// so this will come as an input parameter of this function.
pub struct Make<'info> {
    #[account(mut)] // maker is mutable because they are paying for account creation, so their account info will change.
    pub maker: Signer<'info>,
    #[account(
        // init says this account doesn't exist and we have to create it (in anchor).
        init,
        // reason we need a payer is because we are creating a new account.
        payer=maker,
        // calculating how much space it is using.
        space=Escrow::INIT_SPACE +Escrow::DISCRIMINATOR.len(),
        // seeds and bump define a pda, which is a public key which falls off the Ed25519 curve.
        // deterministically genrated from a set of seeds and a bump.
        // hashed using SHA-256 and if it lies on the Ed25519 curve then it regenrates the bump until it lands on the Ed25519 curve.
        // Now we have a public key without a private key.
        // all trust lies within this code.
        seeds=[b"escrow",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    //Token accounts
    // mint_a is for the token being deposited into the vault.
    // mint_b is for the token the maker wants to receive in return.
    // mint accounts are the global information about the token, like total supply, decimals, etc.
    #[account(
      mint::token_program=token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program=token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program,

    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    // token accounts are the accounts which hold the tokens
    // associated token account of the maker.
    // need a token account to transfer tokens from the maker to the vault.
    #[account(
        init,
        payer=maker,
        associated_token::mint = mint_a,
        // authority is the escrow, which makes everything trustless.
        associated_token::authority=escrow,
        associated_token::token_program=token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>, // need to creat ata accounts.
    pub token_program: Interface<'info, TokenInterface>, // since we are interacting with tokens.
    pub system_program: Program<'info, System>, // since we will initialize new accounts. 
}

impl<'info> Make<'info> {
    fn populate_escrow(&mut self, seed: u64, amount: u64, bump: u8) -> Result<()> {
        // defining all of the information being stored in the escrow.
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: amount,
            bump,
        });
        Ok(())
    }
    // CPI - Cross Program Invocation
    // a mechanism that allows one program to call an instruction in another program. 
    // This enables programs to interact and compose functionality on-chain.
    fn deposit_tokens(&mut self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.key(),
                TransferChecked { // transfers a token from one account to another.
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.maker.to_account_info(), // the maker will have this authority.
                },
            ),
            amount,
            self.mint_a.decimals,
        )?;
        Ok(())
    }
}

pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    require_gt!(receive, 0, EscrowError::InvalidAmount);
    require_gt!(amount, 0, EscrowError::InvalidAmount);

    ctx.accounts
        .populate_escrow(seed, receive, ctx.bumps.escrow)?;
    ctx.accounts.deposit_tokens(amount)?;
    Ok(())
}
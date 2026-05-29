//! Helpers for associated token account checks and initialization flows.

use {
    pinocchio::{
        address::Address,
        cpi::{Seed as CpiSeed, Signer},
        error::ProgramError,
        AccountView, ProgramResult,
    },
    pinocchio_associated_token_account::instructions::{Create, CreateIdempotent},
};

use crate::token_interface;

/// Validates an associated token account's owner, token layout, and PDA
/// derivation.
#[inline]
pub fn check(
    account: &AccountView,
    wallet: &AccountView,
    mint: &AccountView,
    token_program: &AccountView,
) -> Result<(), ProgramError> {
    token_interface::check_token_account(account)?;

    let expected_address = Address::derive_program_address(
        &[
            wallet.address().as_ref(),
            token_program.address().as_ref(),
            mint.address().as_ref(),
        ],
        &pinocchio_associated_token_account::ID,
    )
    .ok_or(ProgramError::InvalidSeeds)?
    .0;

    if expected_address != *account.address() {
        return Err(ProgramError::InvalidSeeds);
    }

    Ok(())
}

/// Creates and initializes an associated token account.
///
/// Returns an error if the target account already exists.
#[inline]
pub fn init(
    account: &AccountView,
    mint: &AccountView,
    payer: &AccountView,
    wallet: &AccountView,
    system_program: &AccountView,
    token_program: &AccountView,
    signer_seeds: &[CpiSeed],
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];

    Create {
        funding_account: payer,
        account,
        wallet,
        mint,
        system_program,
        token_program,
    }
    .invoke_signed(&signers)
}

/// Creates and initializes an associated token account only when missing.
///
/// Returns an error if the account exists with a different owner.
#[inline]
pub fn init_if_needed(
    account: &AccountView,
    mint: &AccountView,
    payer: &AccountView,
    wallet: &AccountView,
    system_program: &AccountView,
    token_program: &AccountView,
    signer_seeds: &[CpiSeed],
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];

    CreateIdempotent {
        funding_account: payer,
        account,
        wallet,
        mint,
        system_program,
        token_program,
    }
    .invoke_signed(&signers)
}

//! Unified helpers that dispatch to SPL Token or Token-2022 implementations.

use {
    pinocchio::{
        address::Address, cpi::Seed as CpiSeed, error::ProgramError, sysvars::rent::Rent,
        AccountView, ProgramResult,
    },
    pinocchio_token::state::{Account as TokenAccount, Mint},
    pinocchio_token_2022::state::{Account as TokenAccount2022, Mint as Mint2022},
};

use crate::{token, token_2022, DataLen};

#[inline]
fn is_token_program(program: &Address) -> bool {
    program == &pinocchio_token::ID
}

#[inline]
fn is_token_2022_program(program: &Address) -> bool {
    program == &pinocchio_token_2022::ID
}

/// Validates a token account for either SPL Token or Token-2022, based on
/// account owner.
#[inline]
pub fn check_token_account(account: &AccountView) -> Result<(), ProgramError> {
    if is_token_program(account.owner()) {
        TokenAccount::check_data_len(account)
    } else if is_token_2022_program(account.owner()) {
        TokenAccount2022::check_data_len(account)
    } else {
        Err(ProgramError::InvalidAccountOwner)
    }
}

/// Initializes a token account by dispatching to the selected token program.
#[inline]
pub fn init_token_account(
    account: &AccountView,
    mint: &AccountView,
    payer: &AccountView,
    rent: Option<&Rent>,
    signer_seeds: &[CpiSeed],
    owner: &Address,
    token_program: &Address,
) -> ProgramResult {
    if is_token_program(token_program) {
        token::init_token_account(account, mint, payer, rent, signer_seeds, owner)
    } else if is_token_2022_program(token_program) {
        token_2022::init_token_account(account, mint, payer, rent, signer_seeds, owner)
    } else {
        Err(ProgramError::InvalidAccountOwner)
    }
}

/// Initializes a token account only when the current account is uninitialized.
#[inline]
pub fn init_token_account_if_needed(
    account: &AccountView,
    mint: &AccountView,
    payer: &AccountView,
    rent: Option<&Rent>,
    signer_seeds: &[CpiSeed],
    owner: &Address,
    token_program: &Address,
) -> ProgramResult {
    match check_token_account(account) {
        Ok(()) => Ok(()),
        Err(_) => init_token_account(
            account,
            mint,
            payer,
            rent,
            signer_seeds,
            owner,
            token_program,
        ),
    }
}

/// Validates a mint account for either SPL Token or Token-2022, based on
/// account owner.
#[inline]
pub fn check_mint(account: &AccountView) -> Result<(), ProgramError> {
    if is_token_program(account.owner()) {
        Mint::check_data_len(account)
    } else if is_token_2022_program(account.owner()) {
        Mint2022::check_data_len(account)
    } else {
        Err(ProgramError::InvalidAccountOwner)
    }
}

/// Initializes a mint account by dispatching to the selected token program.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn init_mint(
    account: &AccountView,
    payer: &AccountView,
    rent: Option<&Rent>,
    signer_seeds: &[CpiSeed],
    decimals: u8,
    mint_authority: &Address,
    freeze_authority: Option<&Address>,
    token_program: &Address,
) -> ProgramResult {
    if is_token_program(token_program) {
        token::init_mint(
            account,
            payer,
            rent,
            signer_seeds,
            decimals,
            mint_authority,
            freeze_authority,
        )
    } else if is_token_2022_program(token_program) {
        token_2022::init_mint(
            account,
            payer,
            rent,
            signer_seeds,
            decimals,
            mint_authority,
            freeze_authority,
        )
    } else {
        Err(ProgramError::InvalidAccountOwner)
    }
}

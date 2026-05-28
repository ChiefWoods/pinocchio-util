//! Helpers for SPL Token (legacy program) mint and token-account flows.

use pinocchio::{
    address::Address, cpi::Seed as CpiSeed, error::ProgramError, sysvars::rent::Rent, AccountView,
    ProgramResult,
};
use pinocchio_token::{
    instructions::{InitializeAccount3, InitializeMint2},
    state::{Account as TokenAccount, Mint},
};

use crate::{create_account, DataLen};

impl DataLen for TokenAccount {
    const LEN: usize = TokenAccount::LEN;
}

impl DataLen for Mint {
    const LEN: usize = Mint::LEN;
}

/// Ensures the account is owned by the SPL Token program.
#[inline]
pub fn check_owner(account: &AccountView) -> Result<(), ProgramError> {
    if account.owner() != &pinocchio_token::ID {
        return Err(ProgramError::InvalidAccountOwner);
    }

    Ok(())
}

/// Validates a token account's owner and canonical data length.
#[inline]
pub fn check_token_account(account: &AccountView) -> Result<(), ProgramError> {
    check_owner(account)?;
    TokenAccount::check_data_len(account)
}

/// Creates and initializes an SPL Token account.
#[inline]
pub fn init_token_account(
    account: &AccountView,
    mint: &AccountView,
    payer: &AccountView,
    rent: Option<&Rent>,
    signer_seeds: &[CpiSeed],
    owner: &Address,
) -> ProgramResult {
    create_account::<TokenAccount>(payer, account, rent, signer_seeds, &pinocchio_token::ID)?;

    InitializeAccount3 {
        account,
        mint,
        owner,
    }
    // TODO: replace with invoke_signed
    .invoke()
}

/// Creates and initializes an SPL Token account only when uninitialized.
#[inline]
pub fn init_token_account_if_needed(
    account: &AccountView,
    mint: &AccountView,
    payer: &AccountView,
    rent: Option<&Rent>,
    signer_seeds: &[CpiSeed],
    owner: &Address,
) -> ProgramResult {
    match check_token_account(account) {
        Ok(()) => Ok(()),
        Err(_) => init_token_account(account, mint, payer, rent, signer_seeds, owner),
    }
}

/// Validates a mint account's owner and canonical data length.
#[inline]
pub fn check_mint(account: &AccountView) -> Result<(), ProgramError> {
    check_owner(account)?;
    Mint::check_data_len(account)
}

/// Creates and initializes an SPL Token mint account.
#[inline]
pub fn init_mint(
    account: &AccountView,
    payer: &AccountView,
    rent: Option<&Rent>,
    signer_seeds: &[CpiSeed],
    decimals: u8,
    mint_authority: &Address,
    freeze_authority: Option<&Address>,
) -> ProgramResult {
    create_account::<Mint>(payer, account, rent, signer_seeds, &pinocchio_token::ID)?;

    InitializeMint2 {
        mint: account,
        decimals,
        mint_authority,
        freeze_authority,
    }
    // TODO: replace with invoke_signed
    .invoke()
}

//! Helpers for SPL Token (legacy program) mint and token-account flows.

use pinocchio::cpi::Signer;

use {
    pinocchio::{
        address::Address, cpi::Seed as CpiSeed, error::ProgramError, sysvars::rent::Rent,
        AccountView, ProgramResult,
    },
    pinocchio_token::{
        instructions::{
            Approve, ApproveChecked, AuthorityType, Burn, BurnChecked, CloseAccount, FreezeAccount,
            InitializeAccount3, InitializeImmutableOwner, InitializeMint2, MintTo, MintToChecked,
            Revoke, SetAuthority, SyncNative, ThawAccount, Transfer, TransferChecked,
        },
        state::{Account as TokenAccount, Mint},
    },
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
    .invoke()
}

#[inline]
pub fn transfer(
    from: &AccountView,
    to: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    Transfer {
        from,
        to,
        authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn transfer_checked(
    from: &AccountView,
    mint: &AccountView,
    to: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    TransferChecked {
        from,
        mint,
        to,
        authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
        decimals,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn mint_to(
    mint: &AccountView,
    account: &AccountView,
    mint_authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    MintTo {
        mint,
        account,
        mint_authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn mint_to_checked(
    mint: &AccountView,
    account: &AccountView,
    mint_authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    MintToChecked {
        mint,
        account,
        mint_authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
        decimals,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn burn(
    account: &AccountView,
    mint: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    Burn {
        account,
        mint,
        authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn burn_checked(
    account: &AccountView,
    mint: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    BurnChecked {
        account,
        mint,
        authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
        decimals,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn approve(
    source: &AccountView,
    delegate: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    Approve {
        source,
        delegate,
        authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn approve_checked(
    source: &AccountView,
    mint: &AccountView,
    delegate: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    ApproveChecked {
        source,
        mint,
        delegate,
        authority,
        multisig_signers: &[] as &[&AccountView],
        amount,
        decimals,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn revoke(
    source: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    Revoke {
        source,
        authority,
        multisig_signers: &[] as &[&AccountView],
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn set_authority(
    account: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
    authority_type: AuthorityType,
    new_authority: Option<&Address>,
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    SetAuthority {
        account,
        authority,
        multisig_signers: &[] as &[&AccountView],
        authority_type,
        new_authority,
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn freeze_account(
    account: &AccountView,
    mint: &AccountView,
    freeze_authority: &AccountView,
    signer_seeds: &[CpiSeed],
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    FreezeAccount {
        account,
        mint,
        freeze_authority,
        multisig_signers: &[] as &[&AccountView],
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn thaw_account(
    account: &AccountView,
    mint: &AccountView,
    freeze_authority: &AccountView,
    signer_seeds: &[CpiSeed],
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    ThawAccount {
        account,
        mint,
        freeze_authority,
        multisig_signers: &[] as &[&AccountView],
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn close_account(
    account: &AccountView,
    destination: &AccountView,
    authority: &AccountView,
    signer_seeds: &[CpiSeed],
) -> ProgramResult {
    let signers = [Signer::from(signer_seeds)];
    CloseAccount {
        account,
        destination,
        authority,
        multisig_signers: &[] as &[&AccountView],
    }
    .invoke_signed(&signers)
}

#[inline]
pub fn sync_native(
    native_token: &AccountView,
    rent_sysvar: Option<&AccountView>,
    _signer_seeds: &[CpiSeed],
) -> ProgramResult {
    SyncNative::new(native_token, rent_sysvar).invoke()
}

#[inline]
pub fn init_immutable_owner(account: &AccountView, _signer_seeds: &[CpiSeed]) -> ProgramResult {
    InitializeImmutableOwner { account }.invoke()
}

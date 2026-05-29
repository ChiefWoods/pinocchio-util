use {
    pinocchio::{
        address::Address,
        cpi::{Seed as CpiSeed, Signer},
        error::ProgramError,
        sysvars::rent::Rent,
        AccountView, ProgramResult, Resize,
    },
    pinocchio_system::instructions::CreateAccount,
};

use crate::sysvar::get_sysvar;

pub mod sysvar;
pub mod token;
pub mod token_2022;
pub mod token_interface;
pub mod option;

/// Get the length of an account's data.
pub trait DataLen {
    const LEN: usize;

    fn check_data_len(account: &AccountView) -> Result<(), ProgramError> {
        if account.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

/// Get the PDA seed namespace for a program account type.
pub trait Seed {
    const SEED: &'static str;
}

/// Generate an enum and associated function for updating fields
/// on an account struct.
pub trait AccountUpdates {
    type Update;
    fn updates(&mut self, updates: Self::Update) -> Result<(), ProgramError>;
}

/// Validate surface level account attributes like keys, data length, and more.
pub trait Validate<'info> {
    fn validate(&self) -> Result<(), ProgramError>;
}

/// Build an instruction context with both accounts and instruction data
pub trait Context<'info>: Sized {
    const ACCOUNTS_LEN: usize;
    fn build(accounts: &'info mut [AccountView]) -> Result<Self, ProgramError>;
}

/// Create a new program-owned account using signer seeds.
#[inline]
pub fn create_account<T>(
    payer: &AccountView,
    account: &AccountView,
    rent: Option<&Rent>,
    signer_seeds: &[CpiSeed],
    owner: &Address,
) -> Result<(), ProgramError>
where
    T: DataLen,
{
    let signers = [Signer::from(signer_seeds)];

    let rent = get_sysvar::<Rent>(rent)?;

    CreateAccount {
        from: payer,
        to: account,
        space: T::LEN as u64,
        owner,
        lamports: rent.minimum_balance_unchecked(T::LEN),
    }
    .invoke_signed(&signers)?;

    Ok(())
}

/// Close an account by transferring its lamports to `destination`,
/// shrinking the source account, and marking its data as closed.
#[inline]
pub fn close_account(account: &mut AccountView, destination: &mut AccountView) -> ProgramResult {
    {
        let mut data = account.try_borrow_mut()?;
        data[0] = 0xff;
    }

    destination.set_lamports(destination.lamports() + account.lamports());
    account.set_lamports(0);

    account.resize(1)?;
    account.close()
}
/// Load an immutable reference to an account's data as an arbitrary type. This
/// requires that the provided type implements the `DataLen` trait so there's
/// assurance that no out of bounds access will occur.
///
/// # Example
///
/// ```rust,ignore
/// let account = AccountInfo::new(
///     &account,
///     false,
///     false,
///     false,
///     &mut accounts,
///     &mut ctx,
/// );
///
/// let account_data = load::<UserData>(&account)?;
/// ```
#[inline]
pub fn load<T: DataLen>(account: &AccountView) -> Result<&T, ProgramError> {
    if account.data_len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(unsafe {
        &*core::mem::transmute::<*const u8, *const T>(account.borrow_unchecked().as_ptr())
    })
}

/// Load a mutable reference to an account's data as an arbitrary type. This
/// requires that the provided type implements the `DataLen` trait so there's
/// assurance that no out of bounds access will occur.
///
/// # Example
///
/// ```rust,ignore
/// let mut account = AccountInfo::new(
///     &account,
///     false,
///     false,
///     false,
///     &mut accounts,
///     &mut ctx,
/// );
///
/// let mut account_data = load_mut::<UserData>(&account)?;
/// ```
#[inline]
pub fn load_mut<T: DataLen>(account: &mut AccountView) -> Result<&mut T, ProgramError> {
    if account.data_len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(unsafe {
        &mut *core::mem::transmute::<*mut u8, *mut T>(account.borrow_unchecked_mut().as_mut_ptr())
    })
}

/// Extract an account's discriminator. This is useful if working with Anchor
/// programs, and you need to validate that a provided account is of a specific
/// type.
///
/// You can optionally provide a custom length for the discriminator, and if not
/// provided the length will be defaulted to 8 bytes.
///
/// # Example
///
/// ```rust,ignore
/// let discriminator = load_discriminator(&account, None).unwrap();
/// assert_eq!(discriminator, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
///
/// let discriminator = load_discriminator(&account, Some(4)).unwrap();
/// assert_eq!(discriminator, &[0x00, 0x00, 0x00, 0x00]);
/// ```
#[inline]
pub fn load_discriminator(
    account: &AccountView,
    len: Option<usize>,
) -> Result<&[u8; 8], ProgramError> {
    let discriminator_len = len.unwrap_or(8);
    unsafe {
        account.borrow_unchecked()[0..discriminator_len]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)
    }
}

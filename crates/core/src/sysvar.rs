//! Sysvar helpers for loading optional runtime sysvars in a uniform way.

use pinocchio::{error::ProgramError, sysvars::Sysvar};

/// Returns a sysvar from an optional borrowed value, or loads it with
/// [`Sysvar::get`] when not provided.
///
/// This returns an owned `T`: it copies from a provided `&T` when present,
/// or loads via `Sysvar::get()` when absent.
///
/// # Errors
///
/// Returns [`ProgramError`] if loading the sysvar via [`Sysvar::get`] fails.
///
/// # Example
///
/// ```rust,ignore
/// let rent = get_sysvar::<Rent>(maybe_rent)?;
/// ```
pub fn get_sysvar<T: Sysvar + Copy>(sysvar: Option<&T>) -> Result<T, ProgramError> {
    match sysvar {
        Some(sysvar) => Ok(*sysvar),
        None => T::get(),
    }
}

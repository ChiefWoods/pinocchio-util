//! Parsing helpers for wire-encoded `Option<T>` integer values.
//!
//! Expected layout:
//! - `0x00` => `None`
//! - `0x01` => `Some(T)` followed by little-endian bytes of `T`

use pinocchio::error::ProgramError;

#[doc(hidden)]
pub trait OptionInteger: Sized {
    const LEN: usize;
    fn from_le_slice(data: &[u8]) -> Result<Self, ProgramError>;
}

/// Parse wire-encoded `Option<T>` from a byte slice using a 1-byte tag
/// followed by an optional little-endian payload.
///
/// Expected layout:
/// - `0x00` => `None`
/// - `0x01` => `Some(T)` followed by little-endian bytes of `T`
///
/// # Example
///
/// ```rust
/// use pinocchio_util::option::parse_option;
///
/// let bytes = [1, 42, 0, 0, 0, 0, 0, 0, 0];
/// let value = parse_option::<u64>(&bytes).unwrap();
/// assert_eq!(value, Some(42));
///
/// let bytes = [0];
/// let value = parse_option::<u64>(&bytes).unwrap();
/// assert_eq!(value, None);
/// ```
#[inline]
pub fn parse_option<T: OptionInteger>(data: &[u8]) -> Result<Option<T>, ProgramError> {
    let Some((&tag, rest)) = data.split_first() else {
        return Err(ProgramError::InvalidInstructionData);
    };

    match tag {
        0 => Ok(None),
        1 => {
            if rest.len() < T::LEN {
                return Err(ProgramError::InvalidInstructionData);
            }
            Ok(Some(T::from_le_slice(&rest[..T::LEN])?))
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

macro_rules! parse_option_impl {
    ($t:ty) => {
        impl OptionInteger for $t {
            const LEN: usize = core::mem::size_of::<$t>();

            #[inline]
            fn from_le_slice(data: &[u8]) -> Result<Self, ProgramError> {
                let bytes: [u8; core::mem::size_of::<$t>()] = data
                    .try_into()
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(<$t>::from_le_bytes(bytes))
            }
        }
    };
}

parse_option_impl!(u8);
parse_option_impl!(u16);
parse_option_impl!(u32);
parse_option_impl!(u64);
parse_option_impl!(u128);
parse_option_impl!(i8);
parse_option_impl!(i16);
parse_option_impl!(i32);
parse_option_impl!(i64);
parse_option_impl!(i128);

#[cfg(test)]
mod tests {
    use {super::parse_option, pinocchio::error::ProgramError};

    #[test]
    fn parses_some_unsigned() {
        let bytes = [1, 42, 0, 0, 0];
        let value = parse_option::<u32>(&bytes).unwrap();
        assert_eq!(value, Some(42));
    }

    #[test]
    fn parses_some_signed() {
        let bytes = [1, 0xfe, 0xff];
        let value = parse_option::<i16>(&bytes).unwrap();
        assert_eq!(value, Some(-2));
    }

    #[test]
    fn parses_none() {
        let value = parse_option::<u64>(&[0]).unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn rejects_invalid_tag() {
        let err = parse_option::<u8>(&[2]).unwrap_err();
        assert_eq!(err, ProgramError::InvalidInstructionData);
    }

    #[test]
    fn rejects_missing_tag() {
        let err = parse_option::<u8>(&[]).unwrap_err();
        assert_eq!(err, ProgramError::InvalidInstructionData);
    }

    #[test]
    fn rejects_truncated_payload() {
        let err = parse_option::<u64>(&[1, 1, 2, 3]).unwrap_err();
        assert_eq!(err, ProgramError::InvalidInstructionData);
    }
}

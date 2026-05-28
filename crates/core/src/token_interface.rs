//! Unified helpers that dispatch to SPL Token or Token-2022 implementations.

use {
    bytemuck::{try_pod_read_unaligned, Pod},
    core::mem::size_of,
    pinocchio::{
        address::Address, cpi::Seed as CpiSeed, error::ProgramError, sysvars::rent::Rent,
        AccountView, ProgramResult,
    },
    pinocchio_token::state::{Account as TokenAccount, Mint},
    pinocchio_token_2022::state::{Account as TokenAccount2022, Mint as Mint2022, Multisig},
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

pub type ExtensionsVec = Vec<ExtensionType>;

const ACCOUNT_TYPE_OFFSET: usize = TokenAccount2022::BASE_LEN;
const MINT_ACCOUNT_TYPE: u8 = 1;
const TOKEN_ACCOUNT_TYPE: u8 = 2;
const TYPE_SIZE: usize = size_of::<u16>();
const LENGTH_SIZE: usize = size_of::<u16>();
const TLV_HEADER_SIZE: usize = TYPE_SIZE + LENGTH_SIZE;
const MINT_TLV_START: usize = ACCOUNT_TYPE_OFFSET + 1;

/// Token-2022 extension type identifiers.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtensionType {
    Uninitialized = 0,
    TransferFeeConfig = 1,
    TransferFeeAmount = 2,
    MintCloseAuthority = 3,
    ConfidentialTransferMint = 4,
    ConfidentialTransferAccount = 5,
    DefaultAccountState = 6,
    ImmutableOwner = 7,
    MemoTransfer = 8,
    NonTransferable = 9,
    InterestBearingConfig = 10,
    CpiGuard = 11,
    PermanentDelegate = 12,
    NonTransferableAccount = 13,
    TransferHook = 14,
    TransferHookAccount = 15,
    ConfidentialTransferFeeConfig = 16,
    ConfidentialTransferFeeAmount = 17,
    MetadataPointer = 18,
    TokenMetadata = 19,
    GroupPointer = 20,
    TokenGroup = 21,
    GroupMemberPointer = 22,
    TokenGroupMember = 23,
    ConfidentialMintBurn = 24,
    ScaledUiAmount = 25,
    Pausable = 26,
    PausableAccount = 27,
    PermissionedBurn = 28,
}

impl TryFrom<u16> for ExtensionType {
    type Error = ProgramError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let extension_type = match value {
            0 => Self::Uninitialized,
            1 => Self::TransferFeeConfig,
            2 => Self::TransferFeeAmount,
            3 => Self::MintCloseAuthority,
            4 => Self::ConfidentialTransferMint,
            5 => Self::ConfidentialTransferAccount,
            6 => Self::DefaultAccountState,
            7 => Self::ImmutableOwner,
            8 => Self::MemoTransfer,
            9 => Self::NonTransferable,
            10 => Self::InterestBearingConfig,
            11 => Self::CpiGuard,
            12 => Self::PermanentDelegate,
            13 => Self::NonTransferableAccount,
            14 => Self::TransferHook,
            15 => Self::TransferHookAccount,
            16 => Self::ConfidentialTransferFeeConfig,
            17 => Self::ConfidentialTransferFeeAmount,
            18 => Self::MetadataPointer,
            19 => Self::TokenMetadata,
            20 => Self::GroupPointer,
            21 => Self::TokenGroup,
            22 => Self::GroupMemberPointer,
            23 => Self::TokenGroupMember,
            24 => Self::ConfidentialMintBurn,
            25 => Self::ScaledUiAmount,
            26 => Self::Pausable,
            27 => Self::PausableAccount,
            28 => Self::PermissionedBurn,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(extension_type)
    }
}

impl ExtensionType {
    #[inline]
    fn is_mint_extension(self) -> bool {
        matches!(
            self,
            Self::TransferFeeConfig
                | Self::MintCloseAuthority
                | Self::ConfidentialTransferMint
                | Self::DefaultAccountState
                | Self::NonTransferable
                | Self::InterestBearingConfig
                | Self::PermanentDelegate
                | Self::TransferHook
                | Self::ConfidentialTransferFeeConfig
                | Self::MetadataPointer
                | Self::TokenMetadata
                | Self::GroupPointer
                | Self::TokenGroup
                | Self::GroupMemberPointer
                | Self::ConfidentialMintBurn
                | Self::TokenGroupMember
                | Self::ScaledUiAmount
                | Self::Pausable
                | Self::PermissionedBurn
        )
    }

    #[inline]
    fn try_get_type_len(self) -> Result<usize, ProgramError> {
        match self {
            Self::Uninitialized => Ok(0),
            Self::TransferFeeConfig => Ok(108),
            Self::TransferFeeAmount => Ok(8),
            Self::MintCloseAuthority => Ok(32),
            Self::DefaultAccountState => Ok(1),
            Self::ImmutableOwner => Ok(0),
            Self::MemoTransfer => Ok(1),
            Self::NonTransferable => Ok(0),
            Self::InterestBearingConfig => Ok(52),
            Self::CpiGuard => Ok(1),
            Self::PermanentDelegate => Ok(32),
            Self::NonTransferableAccount => Ok(0),
            Self::TransferHook => Ok(64),
            Self::TransferHookAccount => Ok(1),
            Self::MetadataPointer => Ok(64),
            Self::GroupPointer => Ok(64),
            Self::TokenGroup => Ok(80),
            Self::GroupMemberPointer => Ok(64),
            Self::TokenGroupMember => Ok(72),
            Self::ScaledUiAmount => Ok(56),
            Self::Pausable => Ok(33),
            Self::PausableAccount => Ok(0),
            Self::PermissionedBurn => Ok(32),
            // Unsupported or variable-sized extensions in this local impl.
            Self::TokenMetadata
            | Self::ConfidentialTransferMint
            | Self::ConfidentialTransferAccount
            | Self::ConfidentialTransferFeeConfig
            | Self::ConfidentialTransferFeeAmount
            | Self::ConfidentialMintBurn => Err(ProgramError::InvalidArgument),
        }
    }

    #[inline]
    fn try_get_tlv_len(self) -> Result<usize, ProgramError> {
        Ok(self
            .try_get_type_len()?
            .saturating_add(TYPE_SIZE)
            .saturating_add(LENGTH_SIZE))
    }
}

/// Local extension trait equivalent to Anchor's token extension bound.
pub trait Extension {
    const TYPE: ExtensionType;
}

/// Iterate over TLV extension data and return all extension types present.
/// Works for both Token-2022 mint and token-account layouts.
pub fn get_all_extensions(data: &[u8]) -> Result<Vec<ExtensionType>, ProgramError> {
    let mut extension_types = Vec::new();

    if data.len() <= MINT_TLV_START {
        return Ok(extension_types);
    }

    let account_type_byte = data[ACCOUNT_TYPE_OFFSET];
    if account_type_byte != MINT_ACCOUNT_TYPE && account_type_byte != TOKEN_ACCOUNT_TYPE {
        return Err(ProgramError::InvalidAccountData);
    }

    let ext_bytes = &data[MINT_TLV_START..];
    let mut start = 0usize;

    while start.saturating_add(TLV_HEADER_SIZE) <= ext_bytes.len() {
        let type_bytes: [u8; 2] = ext_bytes[start..start + TYPE_SIZE]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let ext_type = ExtensionType::try_from(u16::from_le_bytes(type_bytes))
            .map_err(|_| ProgramError::InvalidAccountData)?;

        extension_types.push(ext_type);

        let len_bytes: [u8; 2] = ext_bytes[start + TYPE_SIZE..start + TLV_HEADER_SIZE]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let ext_len = u16::from_le_bytes(len_bytes) as usize;

        let next_start = start
            .saturating_add(TLV_HEADER_SIZE)
            .saturating_add(ext_len);
        if next_start > ext_bytes.len() {
            return Err(ProgramError::InvalidAccountData);
        }
        start = next_start;
    }

    Ok(extension_types)
}

/// Returns the required account size for a mint with optional Token-2022
/// extensions.
#[inline]
pub fn find_mint_account_size(extensions: Option<&ExtensionsVec>) -> Result<usize, ProgramError> {
    if let Some(extensions) = extensions {
        if extensions.is_empty() {
            return Ok(Mint2022::BASE_LEN);
        }

        let mut deduped = Vec::with_capacity(extensions.len());
        for extension_type in extensions {
            if !deduped.contains(extension_type) {
                deduped.push(*extension_type);
            }
        }

        let mut extension_size = 0usize;
        for extension_type in deduped {
            if !extension_type.is_mint_extension() {
                return Err(ProgramError::InvalidArgument);
            }
            extension_size = extension_size.saturating_add(extension_type.try_get_tlv_len()?);
        }

        let total_len = extension_size.saturating_add(MINT_TLV_START);
        if total_len == Multisig::LEN {
            Ok(total_len.saturating_add(TYPE_SIZE))
        } else {
            Ok(total_len)
        }
    } else {
        Ok(Mint2022::BASE_LEN)
    }
}

/// Loads and returns a Token-2022 mint extension value from account data.
#[inline]
pub fn get_mint_extension_data<T: Extension + Pod + Copy>(
    account: &AccountView,
) -> Result<T, ProgramError> {
    let mint_data = account.try_borrow()?;

    if mint_data.len() <= MINT_TLV_START || mint_data[ACCOUNT_TYPE_OFFSET] != MINT_ACCOUNT_TYPE {
        return Err(ProgramError::InvalidAccountData);
    }

    let target_type = T::TYPE as u16;
    let mut offset = MINT_TLV_START;
    while offset.saturating_add(TLV_HEADER_SIZE) <= mint_data.len() {
        let extension_type = u16::from_le_bytes([mint_data[offset], mint_data[offset + 1]]);
        let extension_len = u16::from_le_bytes([mint_data[offset + 2], mint_data[offset + 3]]);
        offset = offset.saturating_add(TLV_HEADER_SIZE);

        let extension_len = extension_len as usize;
        let extension_end = offset.saturating_add(extension_len);
        if extension_end > mint_data.len() {
            return Err(ProgramError::InvalidAccountData);
        }

        if extension_type == target_type {
            let extension_bytes = &mint_data[offset..extension_end];
            return try_pod_read_unaligned::<T>(extension_bytes)
                .map_err(|_| ProgramError::InvalidAccountData);
        }

        offset = extension_end;
    }

    Err(ProgramError::InvalidAccountData)
}

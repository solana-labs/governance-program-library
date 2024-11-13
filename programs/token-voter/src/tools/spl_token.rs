//! General purpose SPL token utility functions

use {
    crate::error::TokenVoterError,
    anchor_lang::prelude::*,
    arrayref::array_ref,
    solana_program::{
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_pack::Pack,
        sysvar::Sysvar,
    },
    spl_token::state::Multisig,
    spl_token_2022::{
        extension::{
            transfer_fee::TransferFeeConfig, transfer_hook, AccountType, BaseStateWithExtensions,
            PodStateWithExtensions, StateWithExtensions,
        },
        generic_token_account::GenericTokenAccount,
        pod::PodMint,
        state::{Account, Mint},
    },
    spl_transfer_hook_interface::onchain::add_extra_accounts_for_execute_cpi,
};

/// Computationally cheap method to get amount from a token account
/// It reads amount without deserializing full account data
pub fn get_spl_token_amount(token_account_info: &AccountInfo) -> Result<u64> {
    assert_is_valid_spl_token_account(token_account_info)?;

    // TokeAccount layout:   mint(32), owner(32), amount(8), ...
    let data = token_account_info.try_borrow_data()?;
    let amount_bytes = array_ref![data, 64, 8];

    Ok(u64::from_le_bytes(*amount_bytes))
}

/// Transfers SPL Tokens
pub fn transfer_checked_spl_tokens<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    additional_accounts: &[AccountInfo<'a>],
) -> ProgramResult {
    let spl_token_program_id = spl_token_info.key;

    let mut transfer_instruction = spl_token_2022::instruction::transfer_checked(
        spl_token_program_id,
        source_info.key,
        mint_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
        get_mint_decimals(mint_info)?,
    )
    .unwrap();

    let mut cpi_account_infos = vec![
        source_info.clone(),
        mint_info.clone(),
        destination_info.clone(),
        authority_info.clone(),
    ];

    // if it's a signer, it might be a multisig signer, throw it in!
    additional_accounts
        .iter()
        .filter(|ai| ai.is_signer)
        .for_each(|ai| {
            cpi_account_infos.push(ai.clone());
            transfer_instruction
                .accounts
                .push(AccountMeta::new_readonly(*ai.key, ai.is_signer));
        });
    // used for transfer_hooks
    // scope the borrowing to avoid a double-borrow during CPI
    {
        let mint_data = mint_info.try_borrow_data()?;
        let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
        if let Some(program_id) = transfer_hook::get_program_id(&mint) {
            add_extra_accounts_for_execute_cpi(
                &mut transfer_instruction,
                &mut cpi_account_infos,
                &program_id,
                source_info.clone(),
                mint_info.clone(),
                destination_info.clone(),
                authority_info.clone(),
                amount,
                additional_accounts,
            )?;
        }
    }

    invoke(&transfer_instruction, &cpi_account_infos)?;

    Ok(())
}

/// Transfers SPL Tokens checked from a token account owned by the provided PDA
/// authority with seeds
#[allow(clippy::too_many_arguments)]
pub fn transfer_spl_tokens_signed_checked<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    authority_seeds: &[&[u8]],
    program_id: &Pubkey,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    additional_accounts: &[AccountInfo<'a>],
) -> ProgramResult {
    let (authority_address, bump_seed) = Pubkey::find_program_address(authority_seeds, program_id);

    if authority_address != *authority_info.key {
        msg!(
                "Transfer SPL Token with Authority PDA: {:?} was requested while PDA: {:?} was expected",
                authority_info.key,
                authority_address
            );
        return Err(ProgramError::InvalidSeeds);
    }

    let spl_token_program_id = spl_token_info.key;

    let mut transfer_instruction = spl_token_2022::instruction::transfer_checked(
        spl_token_program_id,
        source_info.key,
        mint_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
        get_mint_decimals(mint_info)?,
    )
    .unwrap();

    let mut signers_seeds = authority_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    let mut cpi_account_infos = vec![
        source_info.clone(),
        mint_info.clone(),
        destination_info.clone(),
        authority_info.clone(),
    ];

    // if it's a signer, it might be a multisig signer, throw it in!
    additional_accounts
        .iter()
        .filter(|ai| ai.is_signer)
        .for_each(|ai| {
            cpi_account_infos.push(ai.clone());
            transfer_instruction
                .accounts
                .push(AccountMeta::new_readonly(*ai.key, ai.is_signer));
        });

    // used for transfer_hooks
    // scope the borrowing to avoid a double-borrow during CPI
    {
        let mint_data = mint_info.try_borrow_data()?;
        let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
        if let Some(program_id) = transfer_hook::get_program_id(&mint) {
            add_extra_accounts_for_execute_cpi(
                &mut transfer_instruction,
                &mut cpi_account_infos,
                &program_id,
                source_info.clone(),
                mint_info.clone(),
                destination_info.clone(),
                authority_info.clone(),
                amount,
                additional_accounts,
            )?;
        }
    }

    invoke_signed(
        &transfer_instruction,
        &cpi_account_infos,
        &[&signers_seeds[..]],
    )?;

    Ok(())
}

/// Asserts the given account_info represents a valid SPL Token account which is
/// initialized and belongs to spl_token program
pub fn assert_is_valid_spl_token_account(account_info: &AccountInfo) -> Result<()> {
    if account_info.data_is_empty() {
        return Err(TokenVoterError::SplTokenAccountDoesNotExist.into());
    }

    if account_info.owner != &spl_token_2022::id() && account_info.owner != &spl_token::id() {
        return Err(TokenVoterError::SplTokenAccountWithInvalidOwner.into());
    }

    // Check if the account data is a valid token account
    // also checks if the account is initialized or not.
    if !Account::valid_account_data(&account_info.try_borrow_data()?) {
        return Err(TokenVoterError::SplTokenInvalidTokenAccountData.into());
    }

    Ok(())
}

/// Asserts the given mint_info represents a valid SPL Token Mint account  which
/// is initialized and belongs to spl_token program
pub fn assert_is_valid_spl_token_mint(mint_info: &AccountInfo) -> Result<()> {
    if mint_info.data_is_empty() {
        return Err(TokenVoterError::SplTokenMintDoesNotExist.into());
    }

    if mint_info.owner != &spl_token_2022::id() && mint_info.owner != &spl_token::id() {
        return Err(TokenVoterError::SplTokenMintWithInvalidOwner.into());
    }

    // assert that length is mint
    if !valid_mint_length(&mint_info.try_borrow_data()?) {
        return Err(TokenVoterError::SplTokenInvalidMintAccountData.into());
    }

    // In token program [36, 8, 1, is_initialized(1), 36] is the layout
    let data = mint_info.try_borrow_data()?;
    let is_initialized = array_ref![data, 45, 1];

    if is_initialized == &[0] {
        return Err(TokenVoterError::SplTokenMintNotInitialized.into());
    }

    Ok(())
}

/// Computationally cheap method to just get supply from a mint without
/// unpacking the whole object
pub fn get_spl_token_mint_supply(mint_info: &AccountInfo) -> Result<u64> {
    assert_is_valid_spl_token_mint(mint_info)?;
    // In token program, 36, 8, 1, 1 is the layout, where the first 8 is supply u64.
    // so we start at 36.
    let data = mint_info.try_borrow_data().unwrap();
    let bytes = array_ref![data, 36, 8];

    Ok(u64::from_le_bytes(*bytes))
}

/// Computationally cheap method to just get supply off a mint without unpacking whole object
pub fn get_mint_decimals(account_info: &AccountInfo) -> Result<u8> {
    // In token program, 36, 8, 1, 1, is the layout, where the first 1 is decimals u8.
    // so we start at 36.
    let data = account_info.try_borrow_data()?;

    // If we don't check this and an empty account is passed in, we get a panic when
    // we try to index into the data.
    if data.is_empty() {
        return Err(TokenVoterError::InvalidAccountData.into());
    }

    Ok(data[44])
}

const ACCOUNTTYPE_MINT: u8 = AccountType::Mint as u8;
fn valid_mint_length(mint_data: &[u8]) -> bool {
    mint_data.len() == Mint::LEN
        || (mint_data.len() > Mint::LEN
            && mint_data.len() != Multisig::LEN
            && ACCOUNTTYPE_MINT == mint_data[Mint::LEN])
}

/// Computationally cheap method to get owner from a token account
/// It reads owner without deserializing full account data
pub fn get_spl_token_owner(token_account_info: &AccountInfo) -> Result<Pubkey> {
    assert_is_valid_spl_token_account(token_account_info)?;

    // TokeAccount layout:   mint(32), owner(32), amount(8)
    let data = token_account_info.try_borrow_data()?;
    let owner_data = array_ref![data, 32, 32];
    Ok(Pubkey::new_from_array(*owner_data))
}

/// Get current TransferFee, returns 0 if no TransferFeeConfig exist.
pub fn get_current_mint_fee(mint_info: &AccountInfo, amount: u64) -> Result<u64> {
    let mint_data = mint_info.try_borrow_data()?;
    let mint = PodStateWithExtensions::<PodMint>::unpack(&mint_data)?;

    if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        Ok(transfer_fee_config
            .calculate_epoch_fee(Clock::get()?.epoch, amount)
            .ok_or(TokenVoterError::Overflow)?)
    } else {
        Ok(0)
    }
}

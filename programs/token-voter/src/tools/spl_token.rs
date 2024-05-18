//! General purpose SPL token utility functions

use {
    crate::error::TokenVoterError,
    anchor_lang::prelude::*,
    arrayref::array_ref,
    solana_program::{
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_option::COption,
        program_pack::Pack,
        system_instruction,
        sysvar::Sysvar,
    },
    spl_governance::tools::pack::unpack_coption_pubkey,
    spl_token::state::Multisig,
    spl_token_2022::{
        cmp_pubkeys,
        extension::{
            transfer_fee::TransferFeeConfig, transfer_hook, AccountType, BaseStateWithExtensions,
            ExtensionType, PodStateWithExtensions, StateWithExtensions,
        },
        generic_token_account::GenericTokenAccount,
        instruction::AuthorityType,
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

/// Checks if the provided spl_token_program is spl token 2022
pub fn is_spl_token_2022(spl_token_program_id: &Pubkey) -> bool {
    if cmp_pubkeys(spl_token_program_id, &spl_token::id()) {
        return false;
    }
    return true;
}

/// Creates and initializes SPL token account with PDA using the provided PDA
/// seeds
#[allow(clippy::too_many_arguments)]
pub fn create_spl_token_account_signed<'a>(
    payer_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    token_account_address_seeds: &[&[u8]],
    token_mint_info: &AccountInfo<'a>,
    token_account_owner_info: &AccountInfo<'a>,
    program_id: &Pubkey,
    system_info: &AccountInfo<'a>,
    spl_token_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    rent: &Rent,
) -> Result<()> {
    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);

    // Get the token space for if the token has extensions.
    let space = if is_spl_token_2022(spl_token_program_id) {
        let mint_data = token_mint_info.data.borrow();

        let state = PodStateWithExtensions::<PodMint>::unpack(&mint_data).map_err(|_| {
            Into::<TokenVoterError>::into(TokenVoterError::InvalidGoverningTokenMint)
        })?;
        let mint_extensions = state.get_extension_types()?;
        let required_extensions =
            ExtensionType::get_required_init_account_extensions(&mint_extensions);
        ExtensionType::try_calculate_account_len::<Account>(&required_extensions)?
    } else {
        spl_token_2022::state::Account::get_packed_len()
    };

    let create_account_instruction = system_instruction::create_account(
        payer_info.key,
        token_account_info.key,
        1.max(rent.minimum_balance(space)),
        space as u64,
        spl_token_program_id,
    );

    let (account_address, bump_seed) =
        Pubkey::find_program_address(token_account_address_seeds, program_id);

    if account_address != *token_account_info.key {
        msg!(
            "Create SPL Token Account with PDA: {:?} was requested while PDA: {:?} was expected",
            token_account_info.key,
            account_address
        );
        return Err(ProgramError::InvalidSeeds.into());
    }

    let mut signers_seeds = token_account_address_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &create_account_instruction,
        &[
            payer_info.clone(),
            token_account_info.clone(),
            system_info.clone(),
        ],
        &[&signers_seeds[..]],
    )?;

    let initialize_account_instruction = spl_token_2022::instruction::initialize_account(
        spl_token_program_id,
        token_account_info.key,
        token_mint_info.key,
        token_account_owner_info.key,
    )?;

    invoke(
        &initialize_account_instruction,
        &[
            payer_info.clone(),
            token_account_info.clone(),
            token_account_owner_info.clone(),
            token_mint_info.clone(),
            spl_token_info.clone(),
            rent_sysvar_info.clone(),
        ],
    )?;

    Ok(())
}

/// Transfers SPL Tokens
pub fn transfer_spl_tokens<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);
    // for previous instruction compatibility we do not use transfer_checked() here.
    #[allow(deprecated)]
    let transfer_instruction = spl_token_2022::instruction::transfer(
        spl_token_program_id,
        source_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    invoke(
        &transfer_instruction,
        &[
            spl_token_info.clone(),
            authority_info.clone(),
            source_info.clone(),
            destination_info.clone(),
        ],
    )?;

    Ok(())
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
    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);

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

/// Mint SPL Tokens
pub fn mint_spl_tokens_to<'a>(
    mint_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    mint_authority_info: &AccountInfo<'a>,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);

    let mint_to_ix = spl_token_2022::instruction::mint_to(
        spl_token_program_id,
        mint_info.key,
        destination_info.key,
        mint_authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    invoke(
        &mint_to_ix,
        &[
            spl_token_info.clone(),
            mint_authority_info.clone(),
            mint_info.clone(),
            destination_info.clone(),
        ],
    )?;

    Ok(())
}

/// Transfers SPL Tokens from a token account owned by the provided PDA
/// authority with seeds
pub fn transfer_spl_tokens_signed<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    authority_seeds: &[&[u8]],
    program_id: &Pubkey,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
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

    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);
    // for previous instruction compatibility we do not use transfer_checked() here.
    #[allow(deprecated)]
    let transfer_instruction = spl_token_2022::instruction::transfer(
        spl_token_program_id,
        source_info.key,
        destination_info.key,
        authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    let mut signers_seeds = authority_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &transfer_instruction,
        &[
            spl_token_info.clone(),
            authority_info.clone(),
            source_info.clone(),
            destination_info.clone(),
        ],
        &[&signers_seeds[..]],
    )?;

    Ok(())
}

/// Transfers SPL Tokens checked from a token account owned by the provided PDA
/// authority with seeds
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

    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);

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

/// Burns SPL Tokens from a token account owned by the provided PDA authority
/// with seeds
pub fn burn_spl_tokens_signed<'a>(
    token_account_info: &AccountInfo<'a>,
    token_mint_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    authority_seeds: &[&[u8]],
    program_id: &Pubkey,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let (authority_address, bump_seed) = Pubkey::find_program_address(authority_seeds, program_id);

    if authority_address != *authority_info.key {
        msg!(
            "Burn SPL Token with Authority PDA: {:?} was requested while PDA: {:?} was expected",
            authority_info.key,
            authority_address
        );
        return Err(ProgramError::InvalidSeeds);
    }

    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);
    let burn_ix = spl_token_2022::instruction::burn(
        spl_token_program_id,
        token_account_info.key,
        token_mint_info.key,
        authority_info.key,
        &[],
        amount,
    )
    .unwrap();

    let mut signers_seeds = authority_seeds.to_vec();
    let bump = &[bump_seed];
    signers_seeds.push(bump);

    invoke_signed(
        &burn_ix,
        &[
            spl_token_info.clone(),
            token_account_info.clone(),
            token_mint_info.clone(),
            authority_info.clone(),
        ],
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

/// Checks if the given account_info  is spl-token token account
pub fn is_spl_token_account(account_info: &AccountInfo) -> bool {
    assert_is_valid_spl_token_account(account_info).is_ok()
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

/// Checks if the given account_info is be spl-token mint account
pub fn is_spl_token_mint(mint_info: &AccountInfo) -> bool {
    assert_is_valid_spl_token_mint(mint_info).is_ok()
}

/// Computationally cheap method to get mint from a token account
/// It reads mint without deserializing full account data
pub fn get_spl_token_mint(token_account_info: &AccountInfo) -> Result<Pubkey> {
    assert_is_valid_spl_token_account(token_account_info)?;

    // TokeAccount layout:   mint(32), owner(32), amount(8), ...
    let data = token_account_info.try_borrow_data()?;
    let mint_data = array_ref![data, 0, 32];
    Ok(Pubkey::new_from_array(*mint_data))
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

/// Computationally cheap method to just get authority from a mint without
/// unpacking the whole object
pub fn get_spl_token_mint_authority(mint_info: &AccountInfo) -> Result<COption<Pubkey>> {
    assert_is_valid_spl_token_mint(mint_info)?;
    // In token program, 36, 8, 1, 1 is the layout, where the first 36 is authority.
    let data = mint_info.try_borrow_data().unwrap();
    let bytes = array_ref![data, 0, 36];

    Ok(unpack_coption_pubkey(bytes)?)
}

/// Asserts current mint authority matches the given authority and it's signer
/// of the transaction
pub fn assert_spl_token_mint_authority_is_signer(
    mint_info: &AccountInfo,
    mint_authority_info: &AccountInfo,
) -> Result<()> {
    let mint_authority = get_spl_token_mint_authority(mint_info)?;

    if mint_authority.is_none() {
        return Err(TokenVoterError::MintHasNoAuthority.into());
    }

    if !mint_authority.contains(mint_authority_info.key) {
        return Err(TokenVoterError::InvalidMintAuthority.into());
    }

    if !mint_authority_info.is_signer {
        return Err(TokenVoterError::MintAuthorityMustSign.into());
    }

    Ok(())
}

/// Asserts current token owner matches the given owner and it's signer of the
/// transaction
pub fn assert_spl_token_owner_is_signer(
    token_info: &AccountInfo,
    token_owner_info: &AccountInfo,
) -> Result<()> {
    let token_owner = get_spl_token_owner(token_info)?;

    if token_owner != *token_owner_info.key {
        return Err(TokenVoterError::InvalidTokenOwner.into());
    }

    if !token_owner_info.is_signer {
        return Err(TokenVoterError::TokenOwnerMustSign.into());
    }

    Ok(())
}

/// Sets spl-token account (Mint or TokenAccount) authority
pub fn set_spl_token_account_authority<'a>(
    account_info: &AccountInfo<'a>,
    account_authority: &AccountInfo<'a>,
    new_account_authority: &Pubkey,
    authority_type: AuthorityType,
    spl_token_info: &AccountInfo<'a>,
) -> Result<()> {
    let spl_token_program_id = &get_spl_token_program_id(spl_token_info);
    let set_authority_ix = spl_token_2022::instruction::set_authority(
        spl_token_program_id,
        account_info.key,
        Some(new_account_authority),
        authority_type,
        account_authority.key,
        &[],
    )?;

    invoke(
        &set_authority_ix,
        &[
            account_info.clone(),
            account_authority.clone(),
            spl_token_info.clone(),
        ],
    )?;

    Ok(())
}

fn get_spl_token_program_id(spl_token_info: &AccountInfo) -> Pubkey {
    if is_spl_token_2022(spl_token_info.key) {
        spl_token_2022::id()
    } else {
        spl_token::id()
    }
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

use {
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
    },
    spl_associated_token_account_interface::instruction as associated_token_account_instruction,
    spl_token_interface::instruction as token_instruction,
};

pub fn mint_nft(accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let edition_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let associated_token_account: &AccountInfo<'_> = next_account_info(account_iter)?;
    let payer: &AccountInfo<'_> = next_account_info(account_iter)?;
    let _rent = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let associated_token_program = next_account_info(account_iter)?;
    let token_metadata_program = next_account_info(account_iter)?;

    if associated_token_account.lamports() == 0 {
        msg!("Creating associated token account ...");
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                payer.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                mint_account.clone(),
                associated_token_account.clone(),
                payer.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("Associated token account exists.");
    }

    msg!("Associated Token Address: {}", associated_token_account.key);

    // Mint the NFT to the user's wallet
    //
    msg!("Minting NFT to associated token account...");
    invoke(
        &crate::mpl_util::create_master_edition_v3(
            edition_account.key,
            mint_account.key,
            mint_authority.key,
            mint_authority.key,
            payer.key,
            metadata_account.key,
            token_program.key,
            system_program.key,
            1,
        ),
        &[
            edition_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            mint_authority.clone(),
            payer.clone(),
            metadata_account.clone(),
            token_program.clone(),
            system_program.clone(),
            token_metadata_program.clone(),
        ],
    )?;
    Ok(())
}

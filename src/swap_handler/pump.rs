use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};

const PUMP_BUY_FLAGS: &[u8] = &[
    1, // pool - readonly
    2, // user - signer + writable
    0, // global_config - readonly
    0, // base_mint - readonly
    0, // quote_mint - readonly
    1, // base_ata - writable
    1, // quote_ata - writable
    1, // base_vault - writable
    1, // quote_vault - writable
    0, // protocol fee recipient - readonly
    1, // protocol fee recipient ata - writable
    0, // base token program - readonly
    0, // quote token program - readonly
    0, // system program - readonly
    0, // associated token program - readonly
    0, // event authority - readonly
    0, // program id - readonly
    1, // coin creator vault ata - writable
    0, // coin creator vault auth - readonly
    1, //global volume acc - writable
    1, //user volume acc - writable
    0, //fee config - readonly
    0, //fee program - readonly
]; //23
const PUMP_SELL_FLAGS: &[u8] = &[
    1, // pool - readonly
    2, // user - signer + writable
    0, // global_config - readonly
    0, // base_mint - readonly
    0, // quote_mint - readonly
    1, // base_ata - writable
    1, // quote_ata - writable
    1, // base_vault - writable
    1, // quote_vault - writable
    0, // protocol fee recipient - readonly
    1, // protocol fee recipient ata - writable
    0, // base token program - readonly
    0, // quote token program - readonly
    0, // system program - readonly
    0, // associated token program - readonly
    0, // event authority - readonly
    0, // program id - readonly
    1, // coin creator vault ata - writable
    0, // coin creator vault auth - readonly
    0, //fee config - readonly
    0, //fee program - readonly
]; //21
pub fn process_pump_buy(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount_in: [u8; 8],
    amount_out: [u8; 8],
) -> ProgramResult {
    let cpi_accounts = [
        &accounts[offset + 4], // pool
        bh.base.payer,         // user
        &accounts[offset + 1], // global_config
        bh.base.token_a_mint,
        bh.base.token_b_mint,
        bh.base.token_a_ata,
        bh.base.token_b_ata,
        &accounts[offset + 5], // vault_a
        &accounts[offset + 6], // vault_b
        &accounts[offset + 3], // pump_fee_wallet
        &accounts[offset + 3], // pump_fee_wallet ata
        bh.base.token_a_program,
        bh.base.token_b_program,
        bh.base.system_program,
        bh.base.ata_program,
        &accounts[offset + 2],  // pump_auth
        &accounts[offset],      // program_id
        &accounts[offset + 7],  // coin_creator_vault_ata
        &accounts[offset + 8],  // coin_creator_vault_authority
        &accounts[offset + 9],  // global_volume_accumulator
        &accounts[offset + 10], // user_volume_accumulator
        &accounts[offset + 11], // pump_fee_config
        &accounts[offset + 12], // pump_fee_program_id
    ];

    let mut instr_data = [0u8; 24];
    instr_data[0..8].copy_from_slice(PUMPFUN_BUY_SELECTOR);
    instr_data[8..16].copy_from_slice(&amount_in);
    instr_data[16..24].copy_from_slice(&amount_out);

    execute_cpi::<23>(
        &PUMP_AMM_PROGRAM_ID,
        &cpi_accounts,
        PUMP_BUY_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

pub fn process_pump_sell(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
) -> ProgramResult {
    let cpi_accounts = [
        &accounts[offset + 4], // pool
        bh.base.payer,         // user
        &accounts[offset + 1], // global_config
        bh.base.token_a_mint,
        bh.base.token_b_mint,
        bh.base.token_a_ata,
        bh.base.token_b_ata,
        &accounts[offset + 5], // vault_a
        &accounts[offset + 6], // vault_b
        &accounts[offset + 3], // pump_fee_wallet
        &accounts[offset + 3], // pump_fee_wallet ata
        bh.base.token_a_program,
        bh.base.token_b_program,
        bh.base.system_program,
        bh.base.ata_program,
        &accounts[offset + 2],  // pump_auth
        &accounts[offset],      // program_id
        &accounts[offset + 7],  // coin_creator_vault_ata
        &accounts[offset + 8],  // coin_creator_vault_authority
        &accounts[offset + 11], // pump_fee_config
        &accounts[offset + 12], // pump_fee_program_id
    ];

    let mut instr_data = [0u8; 24];
    instr_data[0..8].copy_from_slice(PUMPFUN_SELL_SELECTOR);
    instr_data[8..16].copy_from_slice(&amount);
    instr_data[16..24].copy_from_slice(&0u64.to_le_bytes());

    execute_cpi::<21>(
        &PUMP_AMM_PROGRAM_ID,
        &cpi_accounts,
        PUMP_SELL_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

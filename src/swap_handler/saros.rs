use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const SAROS_SWAP_FLAGS: &[u8] = &[0, 0, 2, 1, 1, 1, 1, 1, 1, 0]; //10
const SAROS_DLMM_SWAP_FLAGS: &[u8] = &[1, 0, 0, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0]; //15
/*
    pub program_id: AccountInfo,
    pub pool: AccountInfo,
    pub pool_mint: AccountInfo,
    pub pool_fee: AccountInfo,
    pub pool_auth: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
*/
pub fn process_saros_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let cpi_accounts: [&AccountInfo; 10] = [
        &accounts[offset + 1],                          // pool
        &accounts[offset + 4],                          // pool_auth
        &bh.base.payer,                                 // user transfer auth
        token_in_ata,                                   // source
        &accounts[offset + if a_to_b { 5 } else { 6 }], // swap source
        &accounts[offset + if a_to_b { 6 } else { 5 }], // swap dest
        token_out_ata,                                  // destination
        &accounts[offset + 2],                          // pool_mint
        &accounts[offset + 3],                          // pool_fee
        &bh.base.token_a_program,                       // token program
    ];

    let mut instr_data = [0u8; 17];
    instr_data[0] = 1u8; // discriminator
    instr_data[1..9].copy_from_slice(&amount);
    instr_data[9..17].copy_from_slice(&1u64.to_le_bytes());

    execute_cpi::<10>(
        &SAROS_PROGRAM_ID,
        &cpi_accounts,
        &SAROS_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

/*
    pub program_id: AccountInfo,
    pub memo_program_v2: AccountInfo,
    pub pool: AccountInfo,
    pub event_auth: AccountInfo,
    pub bin_arrray_lower: AccountInfo,
    pub bin_arrray_upper: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
*/
pub fn process_saros_dlmm_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
    is_base_input: Option<bool>,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);
    let direction = if a_to_b { 0u8 } else { 1u8 };

    let cpi_accounts: [&AccountInfo; 15] = [
        &accounts[offset + 2],    // pool
        &bh.base.token_a_mint,    // token mint x
        &bh.base.token_b_mint,    // token mint y
        &accounts[offset + 4],    // bin array lower
        &accounts[offset + 5],    // bin array upper
        &accounts[offset + 6],    // vault_a
        &accounts[offset + 7],    // vault_b
        token_in_ata,             // user vault x (source)
        token_out_ata,            // user vault y (dest)
        &bh.base.payer,           // user
        &bh.base.token_a_program, // token program x
        &bh.base.token_b_program, // token program y
        &accounts[offset + 1],    // memo_program_v2
        &accounts[offset + 3],    // event_auth
        &accounts[offset + 0],    // program_id
    ];

    let mut instr_data = [0u8; 26];
    instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
    instr_data[9..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); // min out
    instr_data[24] = direction as u8; // direction
    instr_data[25] = if is_base_input.unwrap_or(true) {
        0u8
    } else {
        1u8
    };

    execute_cpi::<15>(
        &SAROS_PROGRAM_ID,
        &cpi_accounts,
        &SAROS_DLMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

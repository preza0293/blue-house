use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const OBRIC_V2_SWAP_FLAGS: &[u8] = &[1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 2, 0]; //12
/*
    pub program_id: AccountInfo,
    pub pool: AccountInfo,
    pub protocol fee: AccountInfo,
    pub second_ref_oracle_key: AccountInfo,
    pub third_ref_oracle_key: AccountInfo,
    pub x_price_feed: AccountInfo,
    pub y_price_feed: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
*/
pub fn process_obric_v2_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let cpi_accounts = [
        &accounts[offset + 1],   // pool
        &accounts[offset + 3],   // second_ref_oracle_key
        &accounts[offset + 4],   // third_ref_oracle_key
        &accounts[offset + 7],   // vault_a
        &accounts[offset + 8],   // vault_b
        token_in_ata,            // input ata
        token_out_ata,           // output ata
        &accounts[offset + 2],   // protocol fee
        &accounts[offset + 5],   // x_price_feed
        &accounts[offset + 6],   // y_price_feed
        bh.base.payer,           // payer
        bh.base.token_a_program, // token program
    ];
    let mut instr_data = [0u8; 25];
    instr_data[0..8].copy_from_slice(SWAP2_SELECTOR); // discriminator
    instr_data[8] = a_to_b as u8; // direction flag
    instr_data[9..17].copy_from_slice(&amount);
    instr_data[17..25].copy_from_slice(&1u64.to_le_bytes()); // flag

    execute_cpi::<12>(
        &OBRIC_V2_PROGRAM_ID,
        &cpi_accounts,
        OBRIC_V2_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

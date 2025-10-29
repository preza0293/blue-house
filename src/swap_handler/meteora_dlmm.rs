use crate::Bluehouse;
use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
//swap ix
const DLMM_SWAP_FLAGS: &[u8] = &[1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 2, 0, 0, 0, 0, 1, 1, 1]; //16-18
/*
    pub program_id: AccountInfo,
    pub event_auth: AccountInfo,
    pub lb_pair: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub oracle: AccountInfo,
    pub bin_array_0: AccountInfo,
    pub bin_array_1: AccountInfo,
    pub bin_array_2: AccountInfo,
*/
pub fn process_meteora_dlmm_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let mut cpi_accounts: ArrayVec<&AccountInfo, 18> = ArrayVec::new();
    cpi_accounts.extend([
        &accounts[offset + 2], // lb_pair
        &accounts[offset],     // program_id
        &accounts[offset + 3], // vault_a
        &accounts[offset + 4], // vault_b
        token_in_ata,          // user token in ATA
        token_out_ata,         // user token out ATA
        bh.base.token_a_mint,  // mint A
        bh.base.token_b_mint,  // mint B
        &accounts[offset + 5], // oracle
        &accounts[offset],     // program_id(host fee in)
        bh.base.payer,         // payer
        bh.base.token_a_program,
        bh.base.token_b_program,
        &accounts[offset + 1], // event_auth
        &accounts[offset],     // program_id
        &accounts[offset + 6], // bin_array_0
    ]);

    // Only push bin_array_1 & bin_array_2 if lamports != 0
    if accounts[offset + 7].lamports() != 0 {
        cpi_accounts.push(&accounts[offset + 8]);
    }
    if accounts[offset + 8].lamports() != 0 {
        cpi_accounts.push(&accounts[offset + 9]);
    }

    let mut instr_data = [0u8; 24];
    instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); // flag

    execute_cpi::<18>(
        &DLMM_PROGRAM_ID,
        &cpi_accounts,
        DLMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

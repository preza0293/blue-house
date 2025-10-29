use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const RAY_CPMM_SWAP_FLAGS: &[u8] = &[2, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1]; //13
//swap base input
/*
    pub program_id: AccountInfo,
    pub auth: AccountInfo,
    pub pool: AccountInfo,
    pub amm_config: AccountInfo,
    pub observation: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
*/
pub fn process_ray_cpmm_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let cpi_accounts: [&AccountInfo; 13] = [
        bh.base.payer,                                  // payer
        &accounts[offset + 1],                          // auth
        &accounts[offset + 3],                          // amm_config
        &accounts[offset + 2],                          // pool
        token_in_ata,                                   // token in ATA
        token_out_ata,                                  // token out ATA
        &accounts[offset + if a_to_b { 5 } else { 6 }], // input vault
        &accounts[offset + if a_to_b { 6 } else { 5 }], // output vault
        if a_to_b {
            bh.base.token_a_program
        } else {
            bh.base.token_b_program
        }, // input program
        if a_to_b {
            bh.base.token_b_program
        } else {
            bh.base.token_a_program
        }, // output program
        if a_to_b {
            bh.base.token_a_mint
        } else {
            bh.base.token_b_mint
        }, // input mint
        if a_to_b {
            bh.base.token_b_mint
        } else {
            bh.base.token_a_mint
        }, // output mint
        &accounts[offset + 4],                          // observation
    ];

    let mut instr_data = [0u8; 24];
    instr_data[0..8].copy_from_slice(CPSWAP_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); // other amount / flag

    execute_cpi::<13>(
        &RAY_CPMM_PROGRAM_ID,
        &cpi_accounts,
        RAY_CPMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

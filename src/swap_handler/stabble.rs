use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const STABBLE_SWAP_FLAGS: &[u8] = &[2, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]; //12
/*
   pub program_id: AccountInfo,
   pub withdraw_auth: AccountInfo,
   pub vault_auth: AccountInfo,
   pub vault: AccountInfo,
   pub vault_program: AccountInfo,
   pub benificary_token_account: AccountInfo,
   pub pool: AccountInfo,
   pub vault_a: AccountInfo,
   pub vault_b: AccountInfo,
*/
pub fn process_stabble_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = if a_to_b {
        (&bh.base.token_a_ata, &bh.base.token_b_ata)
    } else {
        (&bh.base.token_b_ata, &bh.base.token_a_ata)
    };

    let cpi_accounts: [&AccountInfo; 12] = [
        bh.base.payer,                                  // payer
        token_in_ata,                                   // token in
        token_out_ata,                                  // token out
        &accounts[offset + if a_to_b { 7 } else { 8 }], // vault token in
        &accounts[offset + if a_to_b { 8 } else { 7 }], // vault token out
        &accounts[offset + 5],                          // beneficiary token account
        &accounts[offset + 6],                          // pool
        &accounts[offset + 1],                          // withdraw_auth
        &accounts[offset + 3],                          // vault
        &accounts[offset + 2],                          // vault_auth
        &accounts[offset + 4],                          // vault_program
        bh.base.token_a_program,                        // token program
    ];

    let mut instr_data = [0u8; 17];
    instr_data[0] = 1; // discriminator
    instr_data[1..9].copy_from_slice(&amount);
    instr_data[9..17].copy_from_slice(&1u64.to_le_bytes());

    execute_cpi::<12>(
        &STABBLE_PROGRAM_ID,
        &cpi_accounts,
        STABBLE_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

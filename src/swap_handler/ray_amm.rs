use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const RAY_SWAP_FLAGS: &[u8] = &[0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]; //17
/*
  pub program_id: AccountInfo,
   pub auth: AccountInfo,
   pub pool: AccountInfo,
   pub vault_a: AccountInfo,
   pub vault_b: AccountInfo,
*/
pub fn process_ray_amm_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let cpi_accounts: [&AccountInfo; 17] = [
        bh.base.token_a_program, // token program
        &accounts[offset + 2],   // pool
        &accounts[offset + 1],   // auth
        &accounts[offset + 2],   // open orders
        &accounts[offset + 3],   // vault_a / coin ATA
        &accounts[offset + 4],   // vault_b / pc ATA
        &accounts[offset + 2],   // serum id
        &accounts[offset + 2],   // serum market
        &accounts[offset + 2],   // serum bids
        &accounts[offset + 2],   // serum asks
        &accounts[offset + 2],   // event queue
        &accounts[offset + 2],   // coin vault
        &accounts[offset + 2],   // pc vault
        &accounts[offset + 2],   // vault signer
        token_in_ata,            // source ATA
        token_out_ata,           // destination ATA
        bh.base.payer,           // owner
    ];

    let mut instr_data = [0u8; 17];
    instr_data[0] = 9u8; // discriminator
    instr_data[1..9].copy_from_slice(&amount); // amount
    instr_data[9..17].copy_from_slice(&1u64.to_le_bytes()); // flag

    execute_cpi::<17>(
        &RAY_AMM_PROGRAM_ID,
        &cpi_accounts,
        RAY_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

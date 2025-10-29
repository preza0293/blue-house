use crate::Bluehouse;
use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const RAY_CL_SWAP_FLAGS: &[u8] = &[2, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]; //13
/*
accounts passed to raydium
program_id, offset+0
pool, offset+1
amm_config, offset+2
observation_state, offset+3
b_extension, offset+4
vault_x, offset+5
vault_y, offset+6
tick 0, offset+7
tick 1, offset+8
tick 2, offset+9
 */

pub fn process_ray_cl_swap(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
    is_base_input: Option<bool>,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);

    let (input_vault, output_vault) = if a_to_b {
        (&accounts[offset + 5], &accounts[offset + 6])
    } else {
        (&accounts[offset + 6], &accounts[offset + 5])
    };

    let mut cpi_accounts: ArrayVec<&AccountInfo, 13> = ArrayVec::new();
    cpi_accounts.extend([
        bh.base.payer,           // payer
        &accounts[offset + 2],   // amm_config
        &accounts[offset + 1],   // pool
        token_in_ata,            // input ATA
        token_out_ata,           // output ATA
        input_vault,             // input vault
        output_vault,            // output vault
        &accounts[offset + 3],   // observation_state
        bh.base.token_a_program, // token program
        &accounts[offset + 7],   // tick_array_0
        &accounts[offset + 4],   // bitmap_extension
    ]);

    // Optionally push tick_array_1 and tick_array_2 if lamports > 0
    if accounts[offset + 8].lamports() != 0 {
        cpi_accounts.push(&accounts[offset + 8]);
    }
    if accounts[offset + 9].lamports() != 0 {
        cpi_accounts.push(&accounts[offset + 9]);
    }

    let mut instr_data = [0u8; 41];
    instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); // other amount threshold
    instr_data[24..40].copy_from_slice(&0u128.to_le_bytes()); // sqrt_price_limit_x64
    instr_data[40] = is_base_input.unwrap_or(true) as u8; // is_base_input

    execute_cpi::<13>(
        &RAY_CL_PROGRAM_ID,
        &cpi_accounts,
        RAY_CL_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

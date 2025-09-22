use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const RAY_CL_SWAP_FLAGS: &[u8] = &[2, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]; //13
/*
accounts passed to raydium
program_id, offset+0
first_mint, offset+1
pool, offset+2
amm_config, offset+3
observation_state, offset+4
b_extension, offset+5
vault_x, offset+6
vault_y, offset+7
tick 0, offset+8
tick 1, offset+9
tick 2, offset+10
 */

pub fn process_ray_cl_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let mut instr_data: ArrayVec<u8, 42> = ArrayVec::new();
    instr_data.try_extend_from_slice(SWAP_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in); // amount
    instr_data.try_extend_from_slice(&1u64.to_le_bytes()); // amount_other
    instr_data.try_extend_from_slice(&0u128.to_le_bytes()); // sqrt_price_limit
    instr_data.try_extend_from_slice(&(true as u8).to_le_bytes()); // is_base_input

    let (token_in_ata, token_out_ata) = token_atas(a_to_b, accounts);

    let (input_vault, output_vault) = if a_to_b {
        (
            accounts[offset + 6].clone(), // input_vault
            accounts[offset + 7].clone(), // output_vault
        )
    } else {
        (
            accounts[offset + 7].clone(), // input_vault
            accounts[offset + 6].clone(), // output_vault
        )
    };
    let mut cpi_accounts: ArrayVec<AccountInfo, 13> = ArrayVec::new();
    cpi_accounts.push(accounts[0].clone()); // payer
    cpi_accounts.push(accounts[offset + 3].clone()); // amm_config
    cpi_accounts.push(accounts[offset + 2].clone()); // pool
    cpi_accounts.push(token_in_ata.clone()); // input ATA
    cpi_accounts.push(token_out_ata.clone()); // output ATA
    cpi_accounts.push(input_vault); // input vault 
    cpi_accounts.push(output_vault); // output vault
    cpi_accounts.push(accounts[offset + 4].clone()); // observation_state
    cpi_accounts.push(accounts[3].clone()); // token program
    cpi_accounts.push(accounts[offset + 8].clone()); // tick_array_0
    cpi_accounts.push(accounts[offset + 5].clone()); // tick_array_bitmap_extension

    // Optional tick arrays (only if lamports > 0)
    if accounts.len() > offset + 9 && accounts[offset + 9].lamports() != 0 {
        cpi_accounts.push(accounts[offset + 9].clone());
    }
    if accounts.len() > offset + 10 && accounts[offset + 10].lamports() != 0 {
        cpi_accounts.push(accounts[offset + 10].clone());
    }
    execute_cpi::<13>(
        &RAY_CL_PROGRAM_ID,
        &cpi_accounts,
        &RAY_CL_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

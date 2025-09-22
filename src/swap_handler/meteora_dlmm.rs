use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const DLMM_SWAP_FLAGS: &[u8] = &[1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1]; //18-20
/*
accounts passed to dlmm
program_id,offset+0
first_mint,offset+1
event_auth,offset+2
pair,offset+3
vault_x,offset+4
vault_y,offset+5
oracle,offset+6
bin arrays
*/
pub fn process_meteora_dlmm_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new();
    instr_data.try_extend_from_slice(SWAP_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in);
    instr_data.try_extend_from_slice(&1u64.to_le_bytes()); // other_amount_threshold
    let (token_x_mint, token_y_mint, token_x_prog, token_y_prog, x_vault, y_vault) = (
        &accounts[1],
        &accounts[6],
        &accounts[3],
        &accounts[7],
        &accounts[offset + 4],
        &accounts[offset + 5],
    );

    let (token_in_ata, token_out_ata) = token_atas(a_to_b, accounts);
    // Define a fixed upper bound, assuming DLMM can use max 18 accounts
    let mut dlmm_cpi_accounts: ArrayVec<AccountInfo, 18> = ArrayVec::new();

    dlmm_cpi_accounts.push(accounts[offset + 3].clone()); // pair
    dlmm_cpi_accounts.push(accounts[offset + 0].clone()); // bitmap extension
    dlmm_cpi_accounts.push(x_vault.clone()); // reserve x
    dlmm_cpi_accounts.push(y_vault.clone()); // reserve y
    dlmm_cpi_accounts.push(token_in_ata.clone()); // token_in_ata
    dlmm_cpi_accounts.push(token_out_ata.clone()); // token_out_ata
    dlmm_cpi_accounts.push(token_x_mint.clone()); // token_x_mint
    dlmm_cpi_accounts.push(token_y_mint.clone()); // token_y_mint
    dlmm_cpi_accounts.push(accounts[offset + 6].clone()); // oracle
    dlmm_cpi_accounts.push(accounts[offset + 0].clone()); // host fee in
    dlmm_cpi_accounts.push(accounts[0].clone()); // user
    dlmm_cpi_accounts.push(token_x_prog.clone()); // token_x_prog
    dlmm_cpi_accounts.push(token_y_prog.clone()); // token_y_prog
    dlmm_cpi_accounts.push(accounts[offset + 2].clone()); // event authority
    dlmm_cpi_accounts.push(accounts[offset + 0].clone()); // program

    // Optional accounts (only if they exist/lamports > 0)
    if accounts[offset + 7].lamports() != 0 {
        dlmm_cpi_accounts.push(accounts[offset + 7].clone());
    }
    if accounts[offset + 8].lamports() != 0 {
        dlmm_cpi_accounts.push(accounts[offset + 8].clone());
    }
    if accounts[offset + 9].lamports() != 0 {
        dlmm_cpi_accounts.push(accounts[offset + 9].clone());
    }

    execute_cpi::<18>(
        &DLMM_PROGRAM_ID,
        &dlmm_cpi_accounts,
        &DLMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

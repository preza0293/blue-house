use crate::Bluehouse;
use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const DLMM_SWAP_FLAGS: &[u8] = &[1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1]; //16-20
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
#[derive(Clone)]
pub struct DlmmSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub event_auth: AccountInfo,
    pub lb_pair: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub oracle: AccountInfo,
    pub bin_array_0: AccountInfo,
    pub bin_array_1: Option<AccountInfo>,
    pub bin_array_2: Option<AccountInfo>,
    pub bin_array_3: Option<AccountInfo>,
    pub bin_array_4: Option<AccountInfo>,
}

pub fn process_meteora_dlmm_swap(
    bh: &Bluehouse,
    market: &DlmmSwapAccounts,
    amount: u64,
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);
    let mut cpi_accounts: ArrayVec<&AccountInfo, 20> = ArrayVec::new();
    cpi_accounts.extend([
        &market.lb_pair,
        &market.program_id,
        &market.vault_a,
        &market.vault_b,
        token_in_ata,
        token_out_ata,
        &bh.base.token_a_mint,
        &bh.base.token_b_mint,
        &market.oracle,
        &market.program_id,
        &bh.base.payer,
        &bh.base.token_a_program,
        &bh.base.token_b_program,
        &market.event_auth,
        &market.program_id,
        &market.bin_array_0,
    ]);

    if let Some(ref b1) = market.bin_array_1 {
        cpi_accounts.push(b1);
    }
    if let Some(ref b2) = market.bin_array_2 {
        cpi_accounts.push(b2);
    }
    if let Some(ref b3) = market.bin_array_3 {
        cpi_accounts.push(b3);
    }

    if let Some(ref b4) = market.bin_array_4 {
        cpi_accounts.push(b4);
    }
    let mut instr_data = [0u8; 24];
    //8+8+8
    instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes());

    execute_cpi::<20>(
        &DLMM_PROGRAM_ID,
        &cpi_accounts,
        &DLMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

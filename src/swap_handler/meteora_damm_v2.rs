use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const DAMM_V2_SWAP_FLAGS: &[u8] = &[0, 1, 1, 1, 1, 1, 0, 0, 2, 0, 0, 0, 0, 0]; //14
/*
accounts passed to damm v2
program_id,offset+0
first_token ,offset+1
event_auth,offset+2
pool_auth,offset+3z
pool,offset+4
vault a ,offset+5
vault b,offset+6
*/
#[derive(Clone)]
pub struct DammV2SwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub event_auth: AccountInfo,
    pub pool_auth: AccountInfo,
    pub pool: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}

pub fn process_meteora_damm_v2_swap(
    bh: &Bluehouse,
    market: &DammV2SwapAccounts,
    amount: u64,
    a_to_b: bool,
) -> ProgramResult {
    let mut instr_data = [0u8; 24];
    //8+8+8
    instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes());
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);
    let cpi_accounts = [
        &market.pool_auth,        // pool_auth
        &market.pool,             // pool
        token_in_ata,             // user token in ATA
        token_out_ata,            // user token out ATA
        &market.vault_a,          // vault A
        &market.vault_b,          // vault B
        &bh.base.token_a_mint,    // mint A
        &bh.base.token_b_mint,    // mint B
        &bh.base.payer,           // payer / user
        &bh.base.token_a_program, // token A program
        &bh.base.token_b_program, // token B program
        &market.program_id,       // referrer token account
        &market.event_auth,       // event auth
        &market.program_id,       // program ID
    ];
    execute_cpi::<14>(
        &DAMM_PROGRAM_ID,
        &cpi_accounts,
        &DAMM_V2_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

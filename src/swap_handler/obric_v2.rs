use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const OBRIC_V2_SWAP_FLAGS: &[u8] = &[1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0]; //12
#[derive(Clone)]
pub struct ObricV2SwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub pool: AccountInfo,
    pub ref_oracle_key: AccountInfo,
    pub second_ref_oracle_key: AccountInfo,
    pub third_ref_oracle_key: AccountInfo,
    pub x_price_feed: AccountInfo,
    pub y_price_feed: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}

pub fn process_obric_v2_swap(
    bh: &Bluehouse,
    market: &ObricV2SwapAccounts,
    amount: u64,
    a_to_b: bool,
) -> ProgramResult {
    let mut instr_data = [0u8; 25];
    //8+8+8
    instr_data[0..8].copy_from_slice(SWAP2_SELECTOR); // discriminator
    instr_data[8] = a_to_b as u8;
    instr_data[9..17].copy_from_slice(&amount.to_le_bytes());
    instr_data[17..25].copy_from_slice(&1u64.to_le_bytes());
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);
    let cpi_accounts = [
        &market.pool,                  // pool_
        &market.second_ref_oracle_key, // 2nd oracle
        &market.third_ref_oracle_key,  // 3rd oracle
        &market.vault_a,               // vault a
        &market.vault_b,               // vault b
        token_in_ata,                  // input ata
        token_out_ata,                 // output ata
        &market.ref_oracle_key,        // oracle
        &market.x_price_feed,          // x feed
        &market.y_price_feed,          // y feed
        &bh.base.payer,                // payer
        &bh.base.token_a_program,      // token program
    ];
    execute_cpi::<12>(
        &OBRIC_V2_PROGRAM_ID,
        &cpi_accounts,
        &OBRIC_V2_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

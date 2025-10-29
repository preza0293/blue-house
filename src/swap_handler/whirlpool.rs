use crate::Bluehouse;
use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const WHIRLPOOL_SWAP_FLAGS: &[u8] = &[0, 0, 0, 2, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]; //15
/*
    pub program_id: AccountInfo,
    pub meme_program_v2: AccountInfo,
    pub pool: AccountInfo,
    pub oracle: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub tick_array_0: AccountInfo,
    pub tick_array_1: AccountInfo,
    pub tick_array_2: AccountInfo,
*/
pub fn process_orca_swap_v2(
    bh: &Bluehouse,
    accounts: &[AccountInfo],
    offset: usize,
    amount: [u8; 8],
    a_to_b: bool,
    is_base_input: Option<bool>,
) -> ProgramResult {
    let (mint_a, mint_b, token_program_a, token_program_b, ata_a, ata_b, vault_a, vault_b) =
        if a_to_b {
            (
                &bh.base.token_a_mint,
                &bh.base.token_b_mint,
                &bh.base.token_a_program,
                &bh.base.token_b_program,
                &bh.base.token_a_ata,
                &bh.base.token_b_ata,
                &accounts[offset + 4], // vault_a
                &accounts[offset + 5], // vault_b
            )
        } else {
            (
                &bh.base.token_b_mint,
                &bh.base.token_a_mint,
                &bh.base.token_b_program,
                &bh.base.token_a_program,
                &bh.base.token_b_ata,
                &bh.base.token_a_ata,
                &accounts[offset + 5], // vault_a
                &accounts[offset + 4], // vault_b
            )
        };

    let mut cpi_accounts: ArrayVec<&AccountInfo, 15> = ArrayVec::new();
    cpi_accounts.extend([
        token_program_a,       // token program a
        token_program_b,       // token program b
        &accounts[offset + 1], // meme_program_v2 (memo)
        bh.base.payer,         // payer
        &accounts[offset + 2], // pool
        mint_a,
        mint_b,
        ata_a,
        vault_a,
        ata_b,
        vault_b,
        &accounts[offset + 6], // tick_array_0
        &accounts[offset + 7], // tick_array_1
        &accounts[offset + 8], // tick_array_2
        &accounts[offset + 3], // oracle
    ]);

    let sqrt_price_limit = if a_to_b {
        4295048016i128
    } else {
        79226673515401279992447579055i128
    };

    let mut instr_data = [0u8; 43];
    instr_data[0..8].copy_from_slice(SWAP_V2_SELECTOR);
    instr_data[8..16].copy_from_slice(&amount);
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); // other_amount_threshold
    instr_data[24..40].copy_from_slice(&sqrt_price_limit.to_le_bytes());
    instr_data[40] = is_base_input.unwrap_or(true) as u8;
    instr_data[41] = a_to_b as u8;
    instr_data[42] = 0u8; // optional limit

    execute_cpi::<15>(
        &WHIRLPOOLS_PROGRAM_ID,
        &cpi_accounts,
        WHIRLPOOL_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

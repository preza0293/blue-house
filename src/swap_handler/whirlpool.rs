use crate::{Bluehouse, common::*};
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const WHIRLPOOL_SWAP_FLAGS: &[u8] = &[0, 0, 0, 2, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]; //15
/*
accounts passed to orca
program_id,offset+0
first mint,offset+1
memo v2,offset+2
pool,offset+3
oracle,offset+4
vault x,offset+5
vault y,offset+6
ticks(3)
*/

pub struct WhirlpoolSwapV2Accounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub meme_program_v2: AccountInfo,
    pub pool: AccountInfo,
    pub oracle: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub tick_array_0: AccountInfo,
    pub tick_array_1: AccountInfo,
    pub tick_array_2: AccountInfo,
}

pub fn process_orca_swap_v2(
    bh: &Bluehouse,
    market: &WhirlpoolSwapV2Accounts,
    amount: u64,
    a_to_b: bool,
    is_base_input: Option<bool>,
) -> ProgramResult {
    let (mint_a, mint_b, token_program_a, token_program_b, ata_a, ata_b, vault_a, vault_b) =
        if a_to_b {
            (
                &bh.base.token_a_mint,    // mint_a
                &bh.base.token_b_mint,    // mint_b
                &bh.base.token_a_program, // token_program_a
                &bh.base.token_b_program, // token_program_b
                &bh.base.token_a_ata,     // ata_a
                &bh.base.token_b_ata,     // ata_b
                &market.vault_a,          // vault_a
                &market.vault_b,          // vault_b
            )
        } else {
            (
                &bh.base.token_b_mint,    // mint_a
                &bh.base.token_a_mint,    // mint_b
                &bh.base.token_b_program, // token_program_a
                &bh.base.token_a_program, // token_program_b
                &bh.base.token_b_ata,     // ata_a
                &bh.base.token_a_ata,     // ata_b
                &market.vault_b,          // vault_a
                &market.vault_a,          // vault_b
            )
        };

    let mut cpi_accounts: ArrayVec<&AccountInfo, 15> = ArrayVec::new();
    cpi_accounts.extend([
        token_program_a, //token a program
        token_program_b,
        &market.meme_program_v2, //memo
        &bh.base.payer,          //payer
        &market.pool,            //pool
        mint_a,
        mint_b,
        ata_a,
        vault_a,
        ata_b,
        vault_b,
        &market.tick_array_0, //tick array 0
        &market.tick_array_1, //tick array 1
        &market.tick_array_2, //tick array 2
        &market.oracle,       //oracle
    ]);

    let sqrt_price_limit = if a_to_b == true {
        4295048016i128
    } else {
        79226673515401279992447579055i128
    };
    let mut instr_data = [0u8; 43];
    //8+8+8+16+1+1+1
    instr_data[0..8].copy_from_slice(SWAP_V2_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
    instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); //other amount threhsold
    instr_data[24..40].copy_from_slice(&sqrt_price_limit.to_le_bytes()); //sqrt price limit
    instr_data[40] = is_base_input.unwrap_or(true) as u8; // is_base_input
    instr_data[41] = a_to_b as u8;
    instr_data[42] = 0u8; //optional limit
    execute_cpi::<15>(
        &WHIRLPOOLS_PROGRAM_ID,
        &cpi_accounts,
        &WHIRLPOOL_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

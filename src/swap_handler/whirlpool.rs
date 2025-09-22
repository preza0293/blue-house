use crate::common::*;
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
pub fn process_orca_swap_v2(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let (mint_a, mint_b, token_program_a, token_program_b, ata_a, ata_b, vault_a, vault_b) = (
        accounts[1].clone(),          // mint_a
        accounts[6].clone(),          // mint_b
        accounts[3].clone(),          // token_program_a
        accounts[7].clone(),          // token_program_b
        accounts[2].clone(),          // ata_a
        accounts[8].clone(),          // ata_b
        accounts[offset + 5].clone(), // vault_a
        accounts[offset + 6].clone(), // vault_b
    );

    let sqrt_price_limit = if a_to_b == true {
        4295048016i128
    } else {
        79226673515401279992447579055i128
    };

    let mut instr_data: ArrayVec<u8, 43> = ArrayVec::new();
    instr_data.try_extend_from_slice(SWAP_V2_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in);
    instr_data.try_extend_from_slice(&1u64.to_le_bytes()); // other_amount_threshold
    instr_data.try_extend_from_slice(&sqrt_price_limit.to_le_bytes());
    instr_data.try_extend_from_slice(&[true as u8]); // amount_specified_is_input//TODO:generalize
    instr_data.try_extend_from_slice(&[a_to_b as u8]);
    instr_data.try_extend_from_slice(&[0u8]); // optional limit
    let mut cpi_accounts: ArrayVec<AccountInfo, 15> = ArrayVec::new();
    cpi_accounts.push(token_program_a);
    cpi_accounts.push(token_program_b);
    cpi_accounts.push(accounts[offset + 2].clone());
    cpi_accounts.push(accounts[0].clone());
    cpi_accounts.push(accounts[offset + 3].clone());
    cpi_accounts.push(mint_a);
    cpi_accounts.push(mint_b);
    cpi_accounts.push(ata_a);
    cpi_accounts.push(vault_a);
    cpi_accounts.push(ata_b);
    cpi_accounts.push(vault_b);
    // Optional: bin arrays (only include if account exists)
    if accounts[offset + 7].lamports() != 0 {
        cpi_accounts.push(accounts[offset + 7].clone());
    }
    if accounts[offset + 8].lamports() != 0 {
        cpi_accounts.push(accounts[offset + 8].clone());
    }
    if accounts[offset + 9].lamports() != 0 {
        cpi_accounts.push(accounts[offset + 9].clone());
    }
    cpi_accounts.push(accounts[offset + 4].clone());
    execute_cpi::<15>(
        &WHIRLPOOLS_PROGRAM_ID,
        &cpi_accounts,
        &WHIRLPOOL_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

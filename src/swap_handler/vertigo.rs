use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const VERTIGO_SWAP_FLAGS: &[u8] = &[1, 2, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]; //13
/*
accounts passed to vertigo
program_id,offset+0
first_mint,offser+1
pool,offset+2
pool_owner,offset+3
vault a,offset+4
vault b,offset+5
 */
pub fn process_vertigo_buy(
    accounts: &[AccountInfo],
    amount: [u8; 8],
    offset: usize,
) -> ProgramResult {
    let mut instr_data = [0u8; 24];
    //8+8+8
    instr_data[0..8].copy_from_slice(VERTIGO_BUY_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&0u64.to_le_bytes());

    let cpi_accounts = [
        accounts[offset + 2].clone(), // pool
        accounts[0].clone(),          // user
        accounts[offset + 3].clone(), // owner
        accounts[1].clone(),          // mint a
        accounts[6].clone(),          // mint b
        accounts[2].clone(),          // user ata a
        accounts[8].clone(),          // user ata b
        accounts[offset + 4].clone(), // vault a
        accounts[offset + 5].clone(), // vault b
        accounts[offset + 3].clone(), // token program a
        accounts[offset + 7].clone(), // token program b
        accounts[3].clone(),          // sys program
        accounts[offset + 0].clone(), // program id
    ];

    execute_cpi::<13>(
        &VERTIGO_PROGRAM_ID,
        &cpi_accounts,
        &VERTIGO_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
pub fn process_vertigo_sell(
    accounts: &[AccountInfo],
    amount: [u8; 8],
    offset: usize,
) -> ProgramResult {
    let mut instr_data = [0u8; 24];
    //8+8+8
    instr_data[0..8].copy_from_slice(VERTIGO_SELL_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount); // amount
    instr_data[16..24].copy_from_slice(&0u64.to_le_bytes());

    let cpi_accounts = [
        accounts[offset + 2].clone(), // pool
        accounts[0].clone(),          // user
        accounts[offset + 3].clone(), // owner
        accounts[1].clone(),          // mint a
        accounts[6].clone(),          // mint b
        accounts[2].clone(),          // user ata a
        accounts[8].clone(),          // user ata b
        accounts[offset + 4].clone(), // vault a
        accounts[offset + 5].clone(), // vault b
        accounts[offset + 3].clone(), // token program a
        accounts[offset + 7].clone(), // token program b
        accounts[3].clone(),          // sys program
        accounts[offset + 0].clone(), // program id
    ];

    execute_cpi::<13>(
        &VERTIGO_PROGRAM_ID,
        &cpi_accounts,
        &VERTIGO_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

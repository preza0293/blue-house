use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const DAMM_SWAP_FLAGS: &[u8] = &[0, 1, 1, 1, 1, 1, 0, 0, 2, 0, 0, 0, 0, 0]; //14
/*
accounts passed to damm
program_id,offset+0
first_token ,offset+1
event_auth,offset+2
pool_auth,offset+3z
pool,offset+4
vault x ,offset+5
vault y,offset+6
*/
pub fn process_meteora_damm_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new();
    instr_data.try_extend_from_slice(SWAP_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in);
    instr_data.try_extend_from_slice(&1u64.to_le_bytes());

    let (token_in_ata, token_out_ata) = token_atas(a_to_b, accounts);
    let cpi_accounts = [
        accounts[offset + 3].clone(), // pool_auth
        accounts[offset + 4].clone(), // pool
        token_in_ata.clone(),         // user token in ATA
        token_out_ata.clone(),        // user token out ATA
        accounts[offset + 5].clone(), // vault A
        accounts[offset + 6].clone(), // vault B
        accounts[6].clone(),          // mint A
        accounts[1].clone(),          // mint B
        accounts[0].clone(),          // payer / user
        accounts[7].clone(),          // token A program
        accounts[3].clone(),          // token B program
        accounts[offset + 0].clone(), // referrer token account
        accounts[offset + 2].clone(), // event auth
        accounts[offset].clone(),     // program ID
    ];
    execute_cpi::<14>(
        &DAMM_PROGRAM_ID,
        &cpi_accounts,
        &DAMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const RAY_SWAP_FLAGS: &[u8] = &[0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]; //17
/*
accounts passed to raydium
program_id,offset+0
first_mint,offser+1
auth,offset+2
pool,offset+3
vault x,offset+4
vault y,offset+5
 */
pub fn process_ray_amm_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new();
    instr_data.try_extend_from_slice(&9u8.to_le_bytes()); // instruction id
    instr_data.try_extend_from_slice(&amount_in); // amount
    instr_data.try_extend_from_slice(&1u64.to_le_bytes()); // other threshold

    let (token_in_ata, token_out_ata) = token_atas(a_to_b, accounts);

    let cpi_accounts = [
        accounts[3].clone(),          // token_program
        accounts[offset + 3].clone(), // amm id (pool)
        accounts[offset + 2].clone(), // amm authority
        accounts[offset + 3].clone(), // open orders
        accounts[offset + 4].clone(), // coin ATA
        accounts[offset + 5].clone(), // pc ATA
        accounts[offset + 3].clone(), // serum id
        accounts[offset + 3].clone(), // serum market
        accounts[offset + 3].clone(), // serum bids
        accounts[offset + 3].clone(), // serum asks
        accounts[offset + 3].clone(), // event queue
        accounts[offset + 3].clone(), // coin vault
        accounts[offset + 3].clone(), // pc vault
        accounts[offset + 3].clone(), // vault signer
        token_in_ata.clone(),         // source ATA
        token_out_ata.clone(),        // dest ATA
        accounts[0].clone(),          // owner
    ];
    execute_cpi::<17>(
        &RAY_AMM_PROGRAM_ID,
        &cpi_accounts,
        &RAY_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

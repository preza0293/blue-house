use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const STABBLE_SWAP_FLAGS: &[u8] = &[2, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]; //12
/*
accounts passed to stabble
program_id,offset+0
first_mint,offset+1
pool ,offset+2
withdraw auth,offset+3
vault,offset+4
vault auth,offset+5
vault program,offset+6
vault a,offset+7
vault b,offset+8
beinifcary_token_out,offset+9
 */
pub fn process_stabble_swap(
    accounts: &[AccountInfo],
    amount: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let mut instr_data = [0u8; 17];
    instr_data[0] = 1;
    instr_data[1..9].copy_from_slice(&amount); // amount
    instr_data[9..17].copy_from_slice(&1u64.to_le_bytes());
    let (token_in_ata, token_out_ata) = token_atas(a_to_b, accounts);
    let (vault_token_in, vault_token_out) = if a_to_b {
        (accounts[offset + 7].clone(), accounts[offset + 8].clone())
    } else {
        (accounts[offset + 8].clone(), accounts[offset + 7].clone())
    };
    let cpi_accounts = [
        accounts[0].clone(),          // user
        token_in_ata.clone(),         // token in
        token_out_ata.clone(),        // token out
        vault_token_in,               // vault token in
        vault_token_out,              //vault token out
        accounts[offset + 9].clone(), // benificcary token out
        accounts[offset + 2].clone(), // pool
        accounts[offset + 3].clone(), // withdraw auth
        accounts[offset + 4].clone(), // vault
        accounts[offset + 5].clone(), // vault auth
        accounts[offset + 6].clone(), // vault program
        accounts[3].clone(),          // token program
    ];

    execute_cpi::<12>(
        &STABBLE_PROGRAM_ID,
        &cpi_accounts,
        &STABBLE_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

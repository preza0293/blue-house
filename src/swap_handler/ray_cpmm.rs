use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const RAY_CPMM_SWAP_FLAGS: &[u8] = &[2, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1]; //13
/*
accounts provided
id,offset+0
first_mint,offset+1
auth,offset+2
pool,offset+3
amm_config,offset+4
vault x,offset+5
vault y,offset+6
observation,offset+7
*/
pub fn process_ray_cpmm_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new();
    instr_data.try_extend_from_slice(CPSWAP_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in); // amount
    instr_data.try_extend_from_slice(&1u64.to_le_bytes()); // min_out
    let (token_in_ata, token_out_ata) = token_atas(a_to_b, accounts);
    let (input_mint, output_mint, input_prog, output_prog, input_vault, output_vault) = if a_to_b {
        (
            accounts[1].clone(),          // input_mint
            accounts[6].clone(),          // output_mint
            accounts[3].clone(),          // input_prog (token program)
            accounts[7].clone(),          // output_prog
            accounts[offset + 5].clone(), // input_vault
            accounts[offset + 6].clone(), // output_vault
        )
    } else {
        (
            accounts[6].clone(),          // input_mint
            accounts[1].clone(),          // output_mint
            accounts[7].clone(),          // input_prog
            accounts[3].clone(),          // output_prog
            accounts[offset + 6].clone(), // input_vault
            accounts[offset + 5].clone(), // output_vault
        )
    };

    let cpi_accounts = [
        accounts[0].clone(),          // payer
        accounts[offset + 2].clone(), // auth
        accounts[offset + 4].clone(), // amm config
        accounts[offset + 3].clone(), // pool state
        token_in_ata.clone(),         // token in ATA
        token_out_ata.clone(),        // token out ATA
        input_vault,                  //
        output_vault,                 //
        input_prog,                   //
        output_prog,                  //
        input_mint,                   //
        output_mint,                  //
        accounts[offset + 7].clone(), // observation
    ];

    execute_cpi::<13>(
        &RAY_CPMM_PROGRAM_ID,
        &cpi_accounts,
        &RAY_CPMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

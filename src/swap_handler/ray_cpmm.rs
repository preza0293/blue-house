use crate::Bluehouse;
use crate::common::*;
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
#[derive(Clone)]
pub struct RayCpmmSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub auth: AccountInfo,
    pub pool: AccountInfo,
    pub amm_config: AccountInfo,
    pub observation: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}

pub fn process_ray_cpmm_swap(
    bh: &Bluehouse,
    market: &RayCpmmSwapAccounts,
    amount: u64,
    a_to_b: bool,
) -> ProgramResult {
    let (token_in_ata, token_out_ata) = bh.token_atas(a_to_b);
    let (input_mint, output_mint, input_prog, output_prog, input_vault, output_vault) = if a_to_b {
        (
            &bh.base.token_a_mint,    // input_mint
            &bh.base.token_b_mint,    // output_mint
            &bh.base.token_a_program, // input_prog (token program)
            &bh.base.token_b_program, // output_prog
            &market.vault_a,          // input_vault
            &market.vault_b,          // output_vault
        )
    } else {
        (
            &bh.base.token_b_mint,    // input_mint
            &bh.base.token_a_mint,    // output_mint
            &bh.base.token_b_program, // input_prog (token program)
            &bh.base.token_a_program, // output_prog
            &market.vault_b,          // input_vault
            &market.vault_a,          // output_vault
        )
    };

    let cpi_accounts = [
        &bh.base.payer,      // payer
        &market.auth,        //auth
        &market.amm_config,  //amm_config
        &market.pool,        //pool state
        token_in_ata,        // token in ATA
        token_out_ata,       // token out ATA
        input_vault,         //
        output_vault,        //
        input_prog,          //
        output_prog,         //
        input_mint,          //
        output_mint,         //
        &market.observation, // observation
    ];
    let mut instr_data = [0u8; 24];
    //8+8+8
    instr_data[0..8].copy_from_slice(CPSWAP_SELECTOR); // discriminator
    instr_data[1..9].copy_from_slice(&amount.to_le_bytes()); // amount
    instr_data[9..17].copy_from_slice(&1u64.to_le_bytes());
    execute_cpi::<13>(
        &RAY_CPMM_PROGRAM_ID,
        &cpi_accounts,
        &RAY_CPMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

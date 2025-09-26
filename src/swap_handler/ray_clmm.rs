use crate::Bluehouse;
use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const RAY_CL_SWAP_FLAGS: &[u8] = &[2, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]; //13
/*
accounts passed to raydium
program_id, offset+0
first_mint, offset+1
pool, offset+2
amm_config, offset+3
observation_state, offset+4
b_extension, offset+5
vault_x, offset+6
vault_y, offset+7
tick 0, offset+8
tick 1, offset+9
tick 2, offset+10
 */
#[derive(Clone)]
pub struct RayClmmSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub pool: AccountInfo,
    pub amm_config: AccountInfo,
    pub onservation: AccountInfo,
    pub bitmap_extension: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub tick_array_0: AccountInfo,
    pub tick_array_1: Option<AccountInfo>,
    pub tick_array_2: Option<AccountInfo>,
}
impl<'a> Bluehouse {
    pub fn process_ray_cl_swap(
        &self,
        market: &RayClmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
        is_base_input: Option<bool>,
    ) -> ProgramResult {
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);

        let (input_vault, output_vault) = if a_to_b {
            (
                &market.vault_a, // input_vault
                &market.vault_b, // output_vault
            )
        } else {
            (
                &market.vault_b, // input_vault
                &market.vault_a, // output_vault
            )
        };
        let mut cpi_accounts: ArrayVec<&AccountInfo, 13> = ArrayVec::new();
        cpi_accounts.extend([
            &self.base.payer,           //payer
            &market.amm_config,         // amm_config
            &market.pool,               // pool
            token_in_ata,               // input ATA
            token_out_ata,              // output ATA
            input_vault,                // input vault
            output_vault,               // output vault
            &market.onservation,        // observation_state
            &self.base.token_a_program, // token program
            &market.tick_array_0,       // tick_array_0
            &market.bitmap_extension,   // tick_array_bitmap_extension
        ]);
        if let Some(ref b1) = market.tick_array_1 {
            cpi_accounts.push(b1);
        }
        if let Some(ref b2) = market.tick_array_2 {
            cpi_accounts.push(b2);
        }
        let mut instr_data = [0u8; 41];
        //8+8+8+16+1
        instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
        instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); //other amount threshold
        instr_data[24..40].copy_from_slice(&0u64.to_le_bytes()); //sqrt price limit x64
        instr_data[40..41].copy_from_slice(&(is_base_input.unwrap_or(true) as u8).to_le_bytes()); // is_base_input
        execute_cpi::<13>(
            &RAY_CL_PROGRAM_ID,
            &cpi_accounts,
            &RAY_CL_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}

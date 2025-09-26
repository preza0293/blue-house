use crate::Bluehouse;
use crate::common::*;
use arrayref::array_ref;
use arrayvec::ArrayVec;
use pinocchio::program_error::ProgramError;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const PANCAKE_SWAP_FLAGS: &[u8] = &[2, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1]; //11-3
#[derive(Clone)]
pub struct PancakeSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub pool: AccountInfo,
    pub amm_config: AccountInfo,
    pub observation: AccountInfo,
    pub bitmap_extension: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub tick_array_0: AccountInfo,
    pub tick_array_1: Option<AccountInfo>,
    pub tick_array_2: Option<AccountInfo>,
}
impl<'a> Bluehouse {
    pub fn process_pancake_v3_swap(
        &self,
        market: &PancakeSwapAccounts,
        amount: u64,
        a_to_b: bool,
        is_base_input: Option<bool>,
    ) -> ProgramResult {
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);
        let (input_vault, output_vault) = if a_to_b {
            (&market.vault_a, &market.vault_b)
        } else {
            (&market.vault_b, &market.vault_a)
        };
        let mut cpi_accounts: ArrayVec<&AccountInfo, 13> = ArrayVec::new();
        cpi_accounts.extend([
            &market.program_id,         // payer
            &market.amm_config,         //amm config
            &market.pool,               //pool
            token_in_ata,               //input ata
            token_out_ata,              // output ata
            input_vault,                // input vault
            output_vault,               //output vault
            &market.observation,        // observation state
            &self.base.token_a_program, //token program
            &market.tick_array_0,
            &market.bitmap_extension,
        ]);
        if let Some(ref b1) = market.tick_array_1 {
            cpi_accounts.push(b1);
        }
        if let Some(ref b2) = market.tick_array_2 {
            cpi_accounts.push(b2);
        }

        let sqrt_price_limit_x64 = 0u128;
        let other_amount_threshold = 1u64;
        let mut instr_data = [0u8; 41];
        instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
        instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[16..24].copy_from_slice(&other_amount_threshold.to_le_bytes());
        instr_data[24..40].copy_from_slice(&sqrt_price_limit_x64.to_le_bytes());
        instr_data[40] = is_base_input.unwrap_or(true) as u8;
        execute_cpi::<13>(
            &PANCAKE_SWAP_V3_PROGRAM_ID,
            &cpi_accounts,
            &PANCAKE_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}

use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const LIFINITY_SWAP_FLAGS: &[u8] = &[0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]; //13
#[derive(Clone)]
pub struct LifinitySwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub pool: AccountInfo,
    pub auth: AccountInfo,
    pub pool_mint: AccountInfo,
    pub fee_account: AccountInfo,
    pub oracle_main_account: AccountInfo,
    pub oracle_sub_account: AccountInfo,
    pub oracle_pc_account: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
impl<'a> Bluehouse {
    pub fn process_lifinity_swap(
        &self,
        market: &LifinitySwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        let mut instr_data = [0u8; 24];
        //8+8+8
        instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
        instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[16..24].copy_from_slice(&1u64.to_le_bytes());
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);
        let (swap_source, swap_destination) = if a_to_b {
            (&market.vault_a, &market.vault_b)
        } else {
            (&market.vault_b, &market.vault_a)
        };
        let cpi_accounts = [
            &market.auth,                // auth
            &market.pool,                // amm
            &self.base.payer,            // user transer auth
            token_in_ata,                // source
            token_in_ata,                // destination
            swap_source,                 // swap source
            swap_destination,            // swap destination
            &market.pool_mint,           // pool mint
            &market.fee_account,         // fee account
            &self.base.token_a_program,  // token  program
            &market.oracle_main_account, // oracle main account
            &market.oracle_sub_account,  //  oracle sub account
            &market.oracle_pc_account,   //  oracle pc account
        ];
        execute_cpi::<13>(
            &LIFINITY_PROGRAM_ID,
            &cpi_accounts,
            &LIFINITY_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}

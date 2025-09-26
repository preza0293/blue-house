use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const SAROS_SWAP_FLAGS: &[u8] = &[0, 0, 2, 1, 1, 1, 1, 1, 1, 0]; //10
const SAROS__DLMM_SWAP_FLAGS: &[u8] = &[1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]; //15

pub struct SarosSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub pool: AccountInfo,
    pub pool_mint: AccountInfo,
    pub pool_fee: AccountInfo,
    pub pool_auth: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
pub struct SarosDlmmSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub memo_program_v2: AccountInfo,
    pub pool: AccountInfo,
    pub event_auth: AccountInfo,
    pub bin_arrray_lower: AccountInfo,
    pub bin_arrray_upper: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
impl<'a> Bluehouse {
    pub fn process_saros_swap(
        &self,
        market: &SarosSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);
        let (swap_source, swap_destination) = if a_to_b {
            (&market.vault_a, &market.vault_b)
        } else {
            (&market.vault_b, &market.vault_a)
        };
        let cpi_accounts: [&AccountInfo; 10] = [
            &market.pool,               // swap
            &market.pool_auth,          // authority
            &self.base.payer,           // user transfer auth
            token_in_ata,               // source
            swap_source,                // swap source
            swap_destination,           // swap dest
            token_out_ata,              // destination
            &market.pool_mint,          // pool mint
            &market.pool_fee,           // pool fee
            &self.base.token_a_program, // token program id
        ];

        let mut instr_data = [0u8; 17];
        //1+8+8
        instr_data[0] = 1u8; // discriminator
        instr_data[1..9].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[9..17].copy_from_slice(&1u64.to_le_bytes());
        execute_cpi::<10>(
            &SAROS_PROGRAM_ID,
            &cpi_accounts,
            &SAROS_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
    pub fn process_saros_slmm_swap(
        &self,
        market: &SarosDlmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
        is_base_input: bool,
    ) -> ProgramResult {
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);

        //naming convention as per saros dlmm idl
        let cpi_accounts: [&AccountInfo; 15] = [
            &market.pool,               //pair
            &self.base.token_a_mint,    // token mint x
            &self.base.token_b_mint,    // token mint y
            &market.bin_arrray_lower,   // bin array lower
            &market.bin_arrray_upper,   // bin array upper
            &market.vault_a,            // token vault x
            &market.vault_b,            // token vault y
            token_in_ata,               // user vault x(source)
            token_out_ata,              // user vault y(dest)
            &self.base.payer,           // user
            &self.base.token_a_program, // token program x
            &self.base.token_b_program, // token program y
            &market.memo_program_v2,    // memo
            &market.event_auth,         // event auth
            &market.program_id,         // program
        ];
        let direction = if a_to_b { 0u8 } else { 1u8 };
        let mut instr_data = [0u8; 26];
        //8+8+8+1+1
        instr_data[0..8].copy_from_slice(SWAP_SELECTOR); // discriminator
        instr_data[9..16].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[16..24].copy_from_slice(&1u64.to_le_bytes()); //min out
        instr_data[24] = direction as u8;
        instr_data[25] = if is_base_input { 0u8 } else { 1u8 };
        execute_cpi::<15>(
            &SAROS_PROGRAM_ID,
            &cpi_accounts,
            &SAROS_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}

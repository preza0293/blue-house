use crate::Bluehouse;
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
#[derive(Clone)]
pub struct VertigoSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub pool: AccountInfo,
    pub pool_owner: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
impl<'a> Bluehouse {
    pub fn process_vertigo_buy(&self, market: &VertigoSwapAccounts, amount: u64) -> ProgramResult {
        let cpi_accounts = [
            &market.pool,               // pool
            &self.base.payer,           // user
            &market.pool_owner,         // owner
            &self.base.token_a_mint,    // mint a
            &self.base.token_b_mint,    // mint b
            &self.base.token_a_ata,     // user ata a
            &self.base.token_b_ata,     // user ata b
            &market.vault_a,            // vault a
            &market.vault_b,            // vault b
            &self.base.token_a_program, // token program a
            &self.base.token_b_program, // token program b
            &self.base.system_program,  // sys program
            &market.program_id,         // program id
        ];
        let mut instr_data = [0u8; 24];
        //8+8+8
        instr_data[0..8].copy_from_slice(VERTIGO_BUY_SELECTOR); // discriminator
        instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[16..24].copy_from_slice(&0u64.to_le_bytes());
        execute_cpi::<13>(
            &VERTIGO_PROGRAM_ID,
            &cpi_accounts,
            &VERTIGO_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
    pub fn process_vertigo_sell(&self, market: &VertigoSwapAccounts, amount: u64) -> ProgramResult {
        let cpi_accounts = [
            &market.pool,               // pool
            &self.base.payer,           // user
            &market.pool_owner,         // owner
            &self.base.token_a_mint,    // mint a
            &self.base.token_b_mint,    // mint b
            &self.base.token_a_ata,     // user ata a
            &self.base.token_b_ata,     // user ata b
            &market.vault_a,            // vault a
            &market.vault_b,            // vault b
            &self.base.token_a_program, // token program a
            &self.base.token_b_program, // token program b
            &self.base.system_program,  // sys program
            &market.program_id,         // program id
        ];
        let mut instr_data = [0u8; 24];
        //8+8+8
        instr_data[0..8].copy_from_slice(VERTIGO_SELL_SELECTOR); // discriminator
        instr_data[8..16].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[16..24].copy_from_slice(&0u64.to_le_bytes());

        execute_cpi::<13>(
            &VERTIGO_PROGRAM_ID,
            &cpi_accounts,
            &VERTIGO_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}

use crate::Bluehouse;
use crate::common::*;
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
pub struct RaySwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub auth: AccountInfo,
    pub pool: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
impl<'a> Bluehouse {
    pub fn process_ray_amm_swap(
        &self,
        market: &RaySwapAccounts,
        amount_in: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);

        let cpi_accounts: [&AccountInfo; 17] = [
            &self.base.token_a_program, // token program
            &market.pool,               // amm id / pool
            &market.auth,               // amm authority
            &market.pool,               // open orders
            &market.vault_a,            // coin ATA
            &market.vault_b,            // pc ATA
            &market.pool,               // serum id
            &market.pool,               // serum market
            &market.pool,               // serum bids
            &market.pool,               // serum asks
            &market.pool,               // event queue
            &market.pool,               // coin vault
            &market.pool,               // pc vault
            &market.pool,               // vault signer
            token_in_ata,               // source ATA
            token_out_ata,              // dest ATA
            &self.base.payer,           // owner
        ];

        let mut instr_data = [0u8; 17];
        //8+8+8
        instr_data[0] = 9u8; // discriminator
        instr_data[1..9].copy_from_slice(&amount_in.to_le_bytes()); // amount
        instr_data[9..17].copy_from_slice(&1u64.to_le_bytes());
        execute_cpi::<17>(
            &RAY_AMM_PROGRAM_ID,
            &cpi_accounts,
            &RAY_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}

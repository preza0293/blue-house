use crate::Bluehouse;
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
#[derive(Clone)]
pub struct StabbleSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub withdraw_auth: AccountInfo,
    pub vault_auth: AccountInfo,
    pub vault: AccountInfo,
    pub vault_program: AccountInfo,
    pub benificary_token_account: AccountInfo,
    pub pool: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
impl<'a> Bluehouse {
    pub fn process_stabble_swap(
        &self,
        market: &StabbleSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);
        let (vault_token_in, vault_token_out) = if a_to_b {
            (&self.base.token_a_ata, &self.base.token_b_ata)
        } else {
            (&self.base.token_b_ata, &self.base.token_a_ata)
        };
        let cpi_accounts = [
            &self.base.payer,                 //payer
            token_in_ata,                     //token in
            token_out_ata,                    //token out
            vault_token_in,                   // vault token in
            vault_token_out,                  //vault token out
            &market.benificary_token_account, //benificary token account
            &market.pool,                     //pool
            &market.withdraw_auth,            //withdraw_auth
            &market.vault,                    //pool
            &market.vault_auth,               //vault_auth
            &market.vault_program,            //vautt_program
            &self.base.token_a_program,       //token program
        ];
        let mut instr_data = [0u8; 17];
        instr_data[0] = 1;
        instr_data[1..9].copy_from_slice(&amount.to_le_bytes()); // amount
        instr_data[9..17].copy_from_slice(&1u64.to_le_bytes());
        execute_cpi::<12>(
            &STABBLE_PROGRAM_ID,
            &cpi_accounts,
            &STABBLE_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}

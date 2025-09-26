use crate::Bluehouse;
use crate::common::*;
use arrayref::array_ref;
use pinocchio::program_error::ProgramError;
use pinocchio::{ProgramResult, account_info::AccountInfo};
const PHONIEX_SWAP_FLAGS: &[u8] = &[0, 0, 1, 2, 1, 1, 1, 1, 0]; //9
//https://github.com/Ellipsis-Labs/phoenix-v1/blob/master/idl/phoenix_v1.json
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
pub struct PhoniexSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub log_auth: AccountInfo,
    pub pool: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
}
impl<'a> Bluehouse {
    pub fn process_phoniex_swap(
        &self,
        market: &PhoniexSwapAccounts,
        amount: u64,
        a_to_b: bool, //base_to_quote
    ) -> ProgramResult {
        let (token_in_ata, token_out_ata) = self.token_atas(a_to_b);

        let cpi_accounts = [
            &market.program_id,         // program
            &market.log_auth,           //log auth
            &market.pool,               //market
            &self.base.payer,           //trader
            token_in_ata,               // base account
            token_out_ata,              // quote account
            &market.vault_a,            //base vault
            &market.vault_b,            // quote vault
            &self.base.token_a_program, //token program
        ];
        let order_type = 2u8; // 'immediateOrCancel'
        let self_trade_behavior = 1u8; // 'cancelProvide'
        let (base_lot_size, quote_lot_size) = get_lot_size(&market.pool)?;
        let (side, num_base_lots, num_quote_lots) = if a_to_b {
            (
                1u8,
                amount
                    .checked_div(base_lot_size)
                    .ok_or(ProgramError::InvalidInstructionData)?,
                0u64,
            ) // 'ask' side
        } else {
            (
                0u8,
                0u64,
                amount
                    .checked_div(quote_lot_size)
                    .ok_or(ProgramError::InvalidInstructionData)?,
            ) // 'bid' side
        };
        let mut instr_data = [0u8; 55];
        instr_data[0] = 0u8; // disc
        instr_data[1] = order_type;
        instr_data[2] = side;
        instr_data[3] = 0; // absence of price_in_ticks

        instr_data[4..12].copy_from_slice(&num_base_lots.to_le_bytes());
        instr_data[12..20].copy_from_slice(&num_quote_lots.to_le_bytes());
        instr_data[20..28].copy_from_slice(&0u64.to_le_bytes()); // min_base_lots_to_fill
        instr_data[28..36].copy_from_slice(&0u64.to_le_bytes()); // min_quote_lots_to_fill

        instr_data[36] = self_trade_behavior;
        instr_data[37] = 0; // absence of match_limit

        instr_data[38..54].copy_from_slice(&0u128.to_le_bytes()); // client_order_id

        instr_data[54] = 0u8; // use_only_deposited_funds = false

        execute_cpi::<9>(
            &PHONIEX_PROGRAM_ID,
            &cpi_accounts,
            &PHONIEX_SWAP_FLAGS,
            &instr_data,
        )?;

        Ok(())
    }
}
fn get_lot_size(market: &AccountInfo) -> Result<(u64, u64), ProgramError> {
    let data = &market.try_borrow_data()?;
    let base_lots_size = u64::from_le_bytes(*array_ref![data, 112, 8]);
    let quote_lots_size = u64::from_le_bytes(*array_ref![data, 192, 8]);
    Ok((base_lots_size, quote_lots_size))
}

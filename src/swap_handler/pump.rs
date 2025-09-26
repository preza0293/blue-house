use crate::Bluehouse;
use crate::common::*;
use pinocchio::{ProgramResult, account_info::AccountInfo};

const PUMP_BUY_FLAGS: &[u8] = &[
    1, // pool - readonly
    2, // user - signer + writable
    0, // global_config - readonly
    0, // base_mint - readonly
    0, // quote_mint - readonly
    1, // base_ata - writable
    1, // quote_ata - writable
    1, // base_vault - writable
    1, // quote_vault - writable
    1, // protocol fee recipient - readonly
    1, // protocol fee recipient ata - writable
    0, // base token program - readonly
    0, // quote token program - readonly
    0, // system program - readonly
    0, // associated token program - readonly
    0, // event authority - readonly
    0, // program id - readonly
    1, // coin creator vault ata - writable
    0, // coin creator vault auth - readonly
    1, //global volume acc - writable
    1, //user volume acc - writable
    0, //fee config - readonly
    0, //fee program - readonly
]; //23
const PUMP_SELL_FLAGS: &[u8] = &[
    1, // pool - readonly
    2, // user - signer + writable
    0, // global_config - readonly
    0, // base_mint - readonly
    0, // quote_mint - readonly
    1, // base_ata - writable
    1, // quote_ata - writable
    1, // base_vault - writable
    1, // quote_vault - writable
    1, // protocol fee recipient - readonly
    1, // protocol fee recipient ata - writable
    0, // base token program - readonly
    0, // quote token program - readonly
    0, // system program - readonly
    0, // associated token program - readonly
    0, // event authority - readonly
    0, // program id - readonly
    1, // coin creator vault ata - writable
    0, // coin creator vault auth - readonly
    0, //fee config - readonly
    0, //fee program - readonly
]; //21
#[derive(Clone)]
pub struct PumpSwapAccounts {
    pub program_id: AccountInfo,
    pub first_mint: AccountInfo,
    pub global_config: AccountInfo,
    pub pump_auth: AccountInfo,
    pub pump_fee_wallet: AccountInfo,
    pub pool: AccountInfo,
    pub vault_a: AccountInfo,
    pub vault_b: AccountInfo,
    pub coin_creator_vault_ata: AccountInfo,
    pub coin_creator_vault_authority: AccountInfo,
    pub global_volume_accumulator: AccountInfo,
    pub user_voulme_accumulator: AccountInfo,
    pub pump_fee_config: AccountInfo,
    pub pump_fee_program_id: AccountInfo,
}
/*
accounts provided
program id,offset+0
global config,offset+1
auth,offset+2
fee_vault,offset+3
pool,offset+4
token_vault,offset+5
sol_vault,offset+6
fee_token_wallet,offset+7
coin_creator_vault_ata,offset+8
coin_creator_vault_auth,offset+9
global_volume_acc,offset+10
user_volume_acc,offset+11
fee_config,offset+12
fee_program,offset+13
*/

pub fn process_pump_buy(
    bh: &Bluehouse,
    market: &PumpSwapAccounts,
    amount_in: u64,
    amount_out: u64,
) -> ProgramResult {
    let cpi_accounts = [
        &market.pool,                         // pool
        &bh.base.payer,                       // user
        &market.global_config,                // global config
        &bh.base.token_a_mint,                // base mint
        &bh.base.token_b_mint,                // quote mint
        &bh.base.token_a_ata,                 // base ata
        &bh.base.token_b_ata,                 // quote ata
        &market.vault_a,                      // base vault
        &market.vault_b,                      // quote vault
        &market.pump_fee_wallet,              // protocol fee recipient
        &market.pump_fee_wallet,              // protocol fee recipient ata
        &bh.base.token_a_program,             // base token program
        &bh.base.token_b_program,             // quote token program
        &bh.base.system_program,              // sys program
        &bh.base.ata_program,                 // ata program
        &market.pump_auth,                    // event_auth
        &market.program_id,                   // program id
        &market.coin_creator_vault_ata,       // coin creator vault ata
        &market.coin_creator_vault_authority, // coin creator vault auth
        &market.global_volume_accumulator,    // global volume accumulator
        &market.user_voulme_accumulator,      // user volume accumulator
        &market.pump_fee_config,              // fee config
        &market.pump_fee_program_id,          // fee program
    ];
    let mut instr_data = [0u8; 24];
    //8+8+8
    instr_data[0..8].copy_from_slice(PUMPFUN_BUY_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount_in.to_le_bytes()); // amount
    instr_data[16..24].copy_from_slice(&amount_out.to_le_bytes());

    execute_cpi::<23>(
        &PUMP_AMM_PROGRAM_ID,
        &cpi_accounts,
        &PUMP_BUY_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
pub fn process_pump_sell(
    bh: &Bluehouse,
    market: &PumpSwapAccounts,
    amount_in: u64,
) -> ProgramResult {
    let cpi_accounts = [
        &market.pool,                         // pool
        &bh.base.payer,                       // user
        &market.global_config,                // global config
        &bh.base.token_a_mint,                // base mint
        &bh.base.token_b_mint,                // quote mint
        &bh.base.token_a_ata,                 // base ata
        &bh.base.token_b_ata,                 // quote ata
        &market.vault_a,                      // base vault
        &market.vault_b,                      // quote vault
        &market.pump_fee_wallet,              // protocol fee recipient
        &market.pump_fee_wallet,              // protocol fee recipient ata
        &bh.base.token_a_program,             // base token program
        &bh.base.token_b_program,             // quote token program
        &bh.base.system_program,              // sys program
        &bh.base.ata_program,                 // ata program
        &market.pump_auth,                    // event_auth
        &market.program_id,                   // program id
        &market.coin_creator_vault_ata,       // coin creator vault ata
        &market.coin_creator_vault_authority, // coin creator vault auth
        &market.pump_fee_config,              // fee config
        &market.pump_fee_program_id,          // fee program
    ];
    let mut instr_data = [0u8; 24];
    //8+8+8
    instr_data[0..8].copy_from_slice(PUMPFUN_SELL_SELECTOR); // discriminator
    instr_data[8..16].copy_from_slice(&amount_in.to_le_bytes()); // amount
    instr_data[16..24].copy_from_slice(&0u64.to_le_bytes());
    execute_cpi::<21>(
        &PUMP_AMM_PROGRAM_ID,
        &cpi_accounts,
        &PUMP_SELL_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

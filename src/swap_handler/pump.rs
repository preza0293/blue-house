use crate::common::*;
use arrayvec::ArrayVec;
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
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    amount_out: [u8; 8],
    offset: usize,
) -> ProgramResult {
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new();
    instr_data.try_extend_from_slice(PUMPFUN_BUY_SELECTOR);
    instr_data.try_extend_from_slice(&amount_out);
    instr_data.try_extend_from_slice(&(amount_in));

    let cpi_accounts = [
        accounts[offset + 4].clone(),  // pool
        accounts[0].clone(),           // user
        accounts[offset + 1].clone(),  // global config
        accounts[6].clone(),           // base mint
        accounts[1].clone(),           // quote mint
        accounts[8].clone(),           // base ata
        accounts[2].clone(),           // quote ata
        accounts[offset + 5].clone(),  // base vault
        accounts[offset + 6].clone(),  // quote vault
        accounts[offset + 3].clone(),  // protocol fee recipient
        accounts[offset + 7].clone(),  // protocol fee recipient ata
        accounts[7].clone(),           // base token program
        accounts[3].clone(),           // quote token program
        accounts[4].clone(),           // sys program
        accounts[5].clone(),           // at program
        accounts[offset + 2].clone(),  // event_auth
        accounts[offset + 0].clone(),  // program id
        accounts[offset + 8].clone(),  // coin creator vault ata
        accounts[offset + 9].clone(),  // coin creator vault auth
        accounts[offset + 10].clone(), // global_vol_acc
        accounts[offset + 11].clone(), // local_volume_acc
        accounts[offset + 12].clone(), // fee config
        accounts[offset + 13].clone(), // fee program
    ];
    execute_cpi::<23>(
        &PUMP_AMM_PROGRAM_ID,
        &cpi_accounts,
        &PUMP_BUY_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
pub fn process_pump_sell(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    offset: usize,
) -> ProgramResult {
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new();
    instr_data.try_extend_from_slice(PUMPFUN_SELL_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in);
    instr_data.try_extend_from_slice(&0u64.to_le_bytes());

    let cpi_accounts = [
        accounts[offset + 4].clone(),  // pool
        accounts[0].clone(),           // user
        accounts[offset + 1].clone(),  // global config
        accounts[6].clone(),           // base mint (token x)
        accounts[1].clone(),           // quote mint
        accounts[8].clone(),           // base ata
        accounts[2].clone(),           // quote ata
        accounts[offset + 5].clone(),  // base vault
        accounts[offset + 6].clone(),  // quote vault
        accounts[offset + 3].clone(),  // protocol fee recipient
        accounts[offset + 7].clone(),  // protocol fee recipient ata
        accounts[7].clone(),           // base token program (token x program)
        accounts[3].clone(),           // quote token program
        accounts[4].clone(),           // sys program
        accounts[5].clone(),           // ata program
        accounts[offset + 2].clone(),  // event_auth
        accounts[offset + 0].clone(),  // program id
        accounts[offset + 8].clone(),  // coin creator vault ata
        accounts[offset + 9].clone(),  // coin creator vault auth
        accounts[offset + 12].clone(), // fee config
        accounts[offset + 13].clone(), // fee program
    ];
    execute_cpi::<21>(
        &PUMP_AMM_PROGRAM_ID,
        &cpi_accounts,
        &PUMP_SELL_FLAGS,
        &instr_data,
    )?;

    Ok(())
}

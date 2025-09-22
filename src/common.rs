use arrayvec::ArrayVec;
use core::{mem::MaybeUninit, slice::from_raw_parts};
use pinocchio::cpi::invoke_unchecked;
use pinocchio::instruction::Account;
use pinocchio::pubkey::Pubkey;
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};
pub const PUMPFUN_BUY_SELECTOR: &[u8; 8] = &[102, 6, 61, 18, 1, 218, 235, 234];
pub const PUMPFUN_SELL_SELECTOR: &[u8; 8] = &[51, 230, 133, 164, 1, 127, 131, 173];

pub const SWAP_V2_SELECTOR: &[u8; 8] = &[43, 4, 237, 11, 26, 201, 30, 98];
pub const SWAP_SELECTOR: &[u8; 8] = &[248, 198, 158, 145, 225, 117, 135, 200];
pub const CPSWAP_SELECTOR: &[u8; 8] = &[143, 190, 90, 218, 196, 30, 51, 222];
pub const HUMIDIFI_SWAP_SELECTOR: u8 = 0x4;
const HUMIDIFI_IX_DATA_KEY_SEED: [u8; 32] = [
    58, 255, 47, 255, 226, 186, 235, 195, 123, 131, 245, 8, 11, 233, 132, 219, 225, 40, 79, 119,
    169, 121, 169, 58, 197, 1, 122, 9, 216, 164, 149, 97,
];
pub const HUMIDIFI_IX_DATA_KEY: u64 = u64::from_le_bytes([
    HUMIDIFI_IX_DATA_KEY_SEED[0],
    HUMIDIFI_IX_DATA_KEY_SEED[1],
    HUMIDIFI_IX_DATA_KEY_SEED[2],
    HUMIDIFI_IX_DATA_KEY_SEED[3],
    HUMIDIFI_IX_DATA_KEY_SEED[4],
    HUMIDIFI_IX_DATA_KEY_SEED[5],
    HUMIDIFI_IX_DATA_KEY_SEED[6],
    HUMIDIFI_IX_DATA_KEY_SEED[7],
]);
pub const WSOL_MINT: [u8; 32] = [
    6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196, 57, 220, 26,
    235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
];
pub const ZERO_ADDRESS: [u8; 32] = [0; 32];
pub const DLMM_PROGRAM_ID: [u8; 32] = [
    4, 233, 225, 47, 188, 132, 232, 38, 201, 50, 204, 233, 226, 100, 12, 206, 21, 89, 12, 28, 98,
    115, 176, 146, 87, 8, 186, 59, 133, 32, 176, 188,
];
pub const WHIRLPOOLS_PROGRAM_ID: [u8; 32] = [
    14, 3, 104, 95, 142, 144, 144, 83, 228, 88, 18, 28, 102, 245, 167, 106, 237, 199, 112, 106,
    161, 28, 130, 248, 170, 149, 42, 143, 43, 120, 121, 169,
];
pub const DAMM_PROGRAM_ID: [u8; 32] = [
    9, 45, 33, 53, 101, 122, 21, 156, 43, 135, 212, 182, 106, 112, 219, 142, 151, 82, 56, 159, 247,
    106, 175, 32, 108, 237, 6, 58, 56, 249, 90, 237,
];
pub const RAY_AMM_PROGRAM_ID: [u8; 32] = [
    75, 217, 73, 196, 54, 2, 195, 63, 32, 119, 144, 237, 22, 163, 82, 76, 161, 185, 151, 92, 241,
    33, 162, 169, 12, 255, 236, 125, 248, 182, 138, 205,
];
pub const RAY_CPMM_PROGRAM_ID: [u8; 32] = [
    169, 42, 90, 139, 79, 41, 89, 82, 132, 37, 80, 170, 147, 253, 91, 149, 181, 172, 230, 168, 235,
    146, 12, 147, 148, 46, 67, 105, 12, 32, 236, 115,
];
pub const RAY_CL_PROGRAM_ID: [u8; 32] = [
    165, 213, 202, 158, 4, 207, 93, 181, 144, 183, 20, 186, 47, 227, 44, 177, 89, 19, 63, 193, 193,
    146, 183, 34, 87, 253, 7, 211, 156, 176, 64, 30,
];
pub const PUMP_AMM_PROGRAM_ID: [u8; 32] = [
    12, 20, 222, 252, 130, 94, 198, 118, 148, 37, 8, 24, 187, 101, 64, 101, 244, 41, 141, 49, 86,
    213, 113, 180, 212, 248, 9, 12, 24, 233, 168, 99,
];
pub const HUMIDIFI_PROGRAM_ID: [u8; 32] = [
    122, 253, 116, 43, 39, 247, 89, 233, 198, 112, 112, 60, 211, 157, 129, 122, 160, 147, 10, 206,
    59, 82, 210, 109, 84, 160, 84, 221, 35, 135, 187, 211,
];

pub const KNOWN_PROGRAMS: &[Pubkey] = &[
    WHIRLPOOLS_PROGRAM_ID,
    DLMM_PROGRAM_ID,
    DAMM_PROGRAM_ID,
    RAY_AMM_PROGRAM_ID,
    RAY_CPMM_PROGRAM_ID,
    RAY_CL_PROGRAM_ID,
    PUMP_AMM_PROGRAM_ID,
    HUMIDIFI_PROGRAM_ID,
];
pub fn execute_cpi<'a, const N: usize>(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    flags: &[u8],
    instr_data: &[u8],
) -> ProgramResult {
    let mut metas = ArrayVec::<AccountMeta<'a>, N>::new();
    const UNINIT: MaybeUninit<Account> = MaybeUninit::<Account>::uninit();
    let mut accounts: [MaybeUninit<Account>; N] = [UNINIT; N];
    for i in 0..account_infos.len() {
        let acc = &account_infos[i];

        let meta = match flags[i] {
            0 => AccountMeta::readonly(acc.key()),
            1 => AccountMeta::writable(acc.key()),
            2 => AccountMeta::new(acc.key(), true, true),
            _ => return Err(ProgramError::InvalidInstructionData),
        };
        metas.push(meta);

        accounts[i].write(Account::from(acc));
    }

    let ix = Instruction {
        program_id,
        accounts: &metas,
        data: instr_data,
    };

    unsafe {
        invoke_unchecked(
            &ix,
            from_raw_parts(accounts.as_ptr() as _, ix.accounts.len()),
        );
    }
    Ok(())
}
pub fn token_atas<'a>(
    a_to_b: bool,
    accounts: &'a [AccountInfo],
) -> (&'a AccountInfo, &'a AccountInfo) {
    if a_to_b == true {
        (&accounts[2], &accounts[8])
    } else {
        (&accounts[8], &accounts[2])
    }
}

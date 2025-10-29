use crate::Bluehouse;
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
pub const SWAP2_SELECTOR: &[u8; 8] = &[65, 75, 63, 76, 235, 91, 91, 136];
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
pub const LIFINITY_PROGRAM_ID: [u8; 32] = [
    28, 206, 152, 152, 53, 109, 235, 63, 44, 52, 141, 202, 162, 64, 79, 85, 142, 144, 236, 53, 202,
    227, 57, 218, 198, 85, 4, 45, 100, 3, 87, 175,
];
pub const OBRIC_V2_PROGRAM_ID: [u8; 32] = [
    11, 240, 33, 91, 170, 159, 148, 24, 247, 111, 15, 60, 221, 210, 13, 72, 178, 63, 199, 181, 10,
    107, 134, 153, 153, 75, 211, 55, 106, 94, 15, 12,
];
pub const WSOL_MINT: [u8; 32] = [
    6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196, 57, 220, 26,
    235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
];
pub const ZERO_ADDRESS: [u8; 32] = [0; 32];
pub const STABBLE_PROGRAM_ID: [u8; 32] = [
    13, 12, 193, 248, 239, 141, 238, 175, 144, 68, 158, 35, 127, 56, 60, 151, 110, 54, 189, 83,
    101, 45, 54, 121, 224, 17, 141, 21, 139, 115, 30, 66,
];
pub const DLMM_PROGRAM_ID: [u8; 32] = [
    4, 233, 225, 47, 188, 132, 232, 38, 201, 50, 204, 233, 226, 100, 12, 206, 21, 89, 12, 28, 98,
    115, 176, 146, 87, 8, 186, 59, 133, 32, 176, 188,
];
pub const VERTIGO_PROGRAM_ID: [u8; 32] = [
    13, 203, 179, 20, 172, 60, 167, 140, 165, 103, 213, 134, 29, 33, 40, 214, 192, 178, 98, 232,
    190, 131, 38, 221, 171, 228, 122, 172, 65, 242, 162, 135,
];
pub const SOLFI_PROGRAM_ID: [u8; 32] = [
    6, 155, 232, 110, 201, 175, 101, 235, 74, 97, 79, 217, 155, 142, 146, 84, 125, 160, 20, 95,
    171, 94, 128, 74, 219, 89, 77, 179, 231, 58, 39, 27,
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
pub const PANCAKE_SWAP_V3_PROGRAM_ID: [u8; 32] = [
    249, 221, 203, 31, 226, 248, 142, 187, 172, 95, 91, 151, 180, 185, 10, 212, 149, 81, 150, 43,
    137, 65, 157, 118, 82, 132, 98, 40, 69, 228, 28, 248,
];
pub const PHONIEX_PROGRAM_ID: [u8; 32] = [
    5, 208, 234, 79, 51, 115, 112, 19, 165, 99, 224, 147, 72, 237, 182, 244, 89, 61, 145, 252, 118,
    65, 249, 36, 124, 36, 65, 168, 66, 161, 187, 235,
];
pub const HUMIDIFI_PROGRAM_ID: [u8; 32] = [
    122, 253, 116, 43, 39, 247, 89, 233, 198, 112, 112, 60, 211, 157, 129, 122, 160, 147, 10, 206,
    59, 82, 210, 109, 84, 160, 84, 221, 35, 135, 187, 211,
];
pub const SAROS_PROGRAM_ID: [u8; 32] = [
    6, 132, 218, 22, 166, 137, 194, 195, 137, 106, 132, 17, 113, 91, 233, 67, 46, 236, 28, 167, 26,
    28, 226, 8, 114, 134, 198, 118, 119, 207, 101, 189,
];
pub const SAROS_DLMM_PROGRAM_ID: [u8; 32] = [
    0, 54, 243, 134, 43, 7, 87, 225, 91, 39, 36, 34, 104, 250, 223, 93, 117, 35, 114, 135, 101,
    210, 85, 71, 148, 74, 214, 126, 86, 85, 91, 81,
];
pub const VERTIGO_BUY_SELECTOR: &[u8; 8] = &[102, 6, 61, 18, 1, 218, 235, 234];
pub const VERTIGO_SELL_SELECTOR: &[u8; 8] = &[51, 230, 133, 164, 1, 127, 131, 173];

pub const KNOWN_PROGRAMS: &[Pubkey] = &[
    OBRIC_V2_PROGRAM_ID,
    LIFINITY_PROGRAM_ID,
    VERTIGO_PROGRAM_ID,
    SOLFI_PROGRAM_ID,
    STABBLE_PROGRAM_ID,
    SAROS_DLMM_PROGRAM_ID,
    SAROS_PROGRAM_ID,
    HUMIDIFI_PROGRAM_ID,
    PHONIEX_PROGRAM_ID,
    PANCAKE_SWAP_V3_PROGRAM_ID,
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
    account_infos: &[&AccountInfo],
    flags: &[u8],
    instr_data: &[u8],
) -> ProgramResult {
    //where N is max accounts used in cpi
    debug_assert_eq!(flags.len(), account_infos.len());

    let mut metas = ArrayVec::<AccountMeta, N>::new();
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

        accounts[i].write(Account::from(*acc));
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
impl<'a> Bluehouse<'a> {
    pub fn token_atas(&self, a_to_b: bool) -> (&AccountInfo, &AccountInfo) {
        if a_to_b {
            (&self.base.token_a_ata, &self.base.token_b_ata)
        } else {
            (&self.base.token_b_ata, &self.base.token_a_ata)
        }
    }
}

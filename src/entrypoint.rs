#![allow(unexpected_cfgs)]
use arrayvec::ArrayVec;
use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    default_panic_handler,
    instruction::{AccountMeta, Instruction},
    no_allocator, nostd_panic_handler,
    program::invoke_with_bounds,
    program_entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub const DLMM_PROGRAM_ID: [u8; 32] = [
    4, 233, 225, 47, 188, 132, 232, 38, 201, 50, 204, 233, 226, 100, 12, 206, 21, 89, 12, 28, 98,
    115, 176, 146, 87, 8, 186, 59, 133, 32, 176, 188,
];

pub const DAMM_PROGRAM_ID: [u8; 32] = [
    9, 45, 33, 53, 101, 122, 21, 156, 43, 135, 212, 182, 106, 112, 219, 142, 151, 82, 56, 159, 247,
    106, 175, 32, 108, 237, 6, 58, 56, 249, 90, 237,
];

const DLMM_SWAP_FLAGS: &[u8] = &[1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 2, 0, 0, 0, 0, 1, 1, 1];
const DAMM_SWAP_FLAGS: &[u8] = &[0, 1, 1, 1, 1, 1, 0, 0, 2, 0, 0, 0, 0, 0];
const SWAP_SELECTOR: &[u8; 8] = &[248, 198, 158, 145, 225, 117, 135, 200];
// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();
#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let ix = ProgramInstruction::try_from(ix_disc)?;
    match ix {
        ProgramInstruction::Swap => swap(accounts, &instruction_data)?,
    }
    Ok(())
}
#[repr(u8)]
pub enum ProgramInstruction {
    Swap,
}
impl TryFrom<&u8> for ProgramInstruction {
    type Error = ProgramError;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(ProgramInstruction::Swap),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

pub fn swap(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    /*
        wallet           0
        token_a_mint     1
        token_a_ata      2
        token_program    3
        sys_program      4
        ata_program      5
        token_b_mint     6
        token_b_program  7
        token_b_ata      8
        dex_id_          9
        ... swap accounts ...
    */
    let amount_in = bytemuck::try_from_bytes::<SwapData>(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?
        .amount_in;
    let program_id = *accounts[9].key();
    match program_id {
        x if x == DLMM_PROGRAM_ID => process_meteora_dlmm_swap(accounts, amount_in, 9)?,
        x if x == DAMM_PROGRAM_ID => process_meteora_damm_swap(accounts, amount_in, 9)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
/*
accounts passed to dammv2
program_id,offset+0
event_auth,offset+1
pool_auth,offset+2
pool,offset+3
vault_a,offset+4
vault_b,offset+5
 */
pub fn process_meteora_damm_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    offset: usize,
) -> ProgramResult {
    if accounts.len() < offset + 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new(); //8+8+8
    instr_data.try_extend_from_slice(SWAP_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in);
    instr_data.try_extend_from_slice(&1u64.to_le_bytes());

    let cpi_accounts: [AccountInfo; 14] = [
        accounts[offset + 2].clone(), // pool_auth
        accounts[offset + 3].clone(), // pool
        accounts[2].clone(),          // user token in ATA
        accounts[8].clone(),          // user token out ATA
        accounts[offset + 4].clone(), // vault A
        accounts[offset + 5].clone(), // vault B
        accounts[6].clone(),          // mint A
        accounts[1].clone(),          // mint B
        accounts[0].clone(),          // payer / user
        accounts[7].clone(),          // token A program
        accounts[3].clone(),          // token B program
        accounts[offset + 0].clone(), // refrel token account(use damm program id for now)
        accounts[offset + 1].clone(), // event auth
        accounts[offset].clone(),     // program ID (passed again)
    ];

    execute_cpi::<14>(
        &DAMM_PROGRAM_ID,
        &cpi_accounts,
        &DAMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
/*
accounts passed to dlmm
program_id,offset+0
event_auth,offset+1
pair,offset+2
vault_a,offset+3
vault_b,offset+4
oracle,offset+5
bin arrays (3)
*/
pub fn process_meteora_dlmm_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    offset: usize,
) -> ProgramResult {
    if accounts.len() < offset + 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mut instr_data: ArrayVec<u8, 24> = ArrayVec::new(); //8+8+8
    instr_data.try_extend_from_slice(SWAP_SELECTOR);
    instr_data.try_extend_from_slice(&amount_in);
    instr_data.try_extend_from_slice(&1u64.to_le_bytes()); // other_amount_threshold

    let cpi_accounts: [AccountInfo; 18] = [
        accounts[offset + 2].clone(), //pair
        accounts[offset + 0].clone(), // dlmm program (bit array---)
        accounts[offset + 3].clone(), // reserve a
        accounts[offset + 4].clone(), // reserve b
        accounts[2].clone(),          // token_in_ata
        accounts[8].clone(),          // token_out_ata
        accounts[1].clone(),          // token_a_mint
        accounts[6].clone(),          // token_b_mint
        accounts[offset + 5].clone(), // oracle
        accounts[offset + 0].clone(), // dlmm program id (host fee in)
        accounts[0].clone(),          // user
        accounts[3].clone(),          // token_a_program
        accounts[7].clone(),          // token_b_program
        accounts[offset + 1].clone(), // event authority
        accounts[offset + 0].clone(), // dlmm program
        accounts[offset + 6].clone(), // bin array
        accounts[offset + 7].clone(), // bin array
        accounts[offset + 8].clone(), // bin array
    ];

    execute_cpi::<18>(
        &DLMM_PROGRAM_ID,
        &cpi_accounts,
        &DLMM_SWAP_FLAGS,
        &instr_data,
    )?;

    Ok(())
}
pub fn execute_cpi<'a, const N: usize>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo],
    flags: &[u8],
    instr_data: &impl AsRef<[u8]>,
) -> ProgramResult {
    let mut metas = ArrayVec::<AccountMeta<'a>, N>::new();

    for (acc, flag) in accounts.iter().zip(flags.iter()) {
        let meta = match flag {
            0 => AccountMeta::readonly(acc.key()),
            1 => AccountMeta::writable(acc.key()),
            2 => AccountMeta::new(acc.key(), true, true),
            _ => return Err(ProgramError::InvalidInstructionData),
        };
        metas.push(meta);
    }

    let ix = Instruction {
        program_id,
        accounts: &metas,
        data: instr_data.as_ref(), // trait method works for all ArrayVec<u8, N>
    };
    invoke_with_bounds::<N>(&ix, &accounts.iter().collect::<ArrayVec<_, N>>())?;
    Ok(())
}
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SwapData {
    pub amount_in: [u8; 8],
}

use crate::common::*;
use arrayvec::ArrayVec;
use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError};
const HUMIDIFI_SWAP_FLAGS: &[u8] = &[1, 1, 1, 1, 1, 1, 0, 0, 0]; //9
/*
accounts provided
program id
pool
pool base ata
pool quote ata
clock
sysvar ixs
humidifi_param
*/
pub fn process_humidifi_swap(
    accounts: &[AccountInfo],
    amount_in: [u8; 8],
    a_to_b: bool,
    offset: usize,
) -> ProgramResult {
    let humidifi_param_data: &[u8; 32] = &accounts[offset + 6].key();
    let swap_id = u64::from_le_bytes(
        humidifi_param_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    );

    let swap_params: SwapParams = SwapParams {
        swap_id: swap_id,
        amount_in: u64::from_le_bytes(amount_in),
        is_base_to_quote: !a_to_b as u8,
        padding: [0; 7],
    };

    let mut instr_data: ArrayVec<u8, 25> = ArrayVec::new();
    let bytes: &[u8] = bytemuck::bytes_of(&swap_params);
    instr_data.try_extend_from_slice(bytes);
    instr_data.try_extend_from_slice(&[HUMIDIFI_SWAP_SELECTOR]);
    let (token_in_ata, token_out_ata) = token_atas(a_to_b, accounts);
    let (base_ata, quote_ata) = if a_to_b {
        (accounts[2].clone(), accounts[3].clone())
    } else {
        (accounts[3].clone(), accounts[2].clone())
    };
    spin_instruction_data(&mut instr_data);
    let cpi_accounts = [
        accounts[0].clone(),          //user
        accounts[offset + 1].clone(), //pool
        base_ata.clone(),             //pool base
        quote_ata.clone(),            //pool quote
        token_in_ata.clone(),
        token_out_ata.clone(),
        accounts[offset + 4].clone(), //clock
        accounts[3].clone(),          //token program
        accounts[offset + 5].clone(), //sysvar ix
    ];
    execute_cpi::<15>(
        &HUMIDIFI_PROGRAM_ID,
        &cpi_accounts,
        &HUMIDIFI_SWAP_FLAGS,
        &instr_data,
    )?;
    Ok(())
}
pub fn spin_instruction_data(data: &mut [u8]) {
    let mut qwords = data.chunks_exact_mut(8);
    let mut pos_mask = 0_u64;
    while let Some(qword) = qwords
        .next()
        .map(|q| unsafe { &mut *q.as_mut_ptr().cast::<u64>() })
    {
        *qword ^= HUMIDIFI_IX_DATA_KEY;
        *qword ^= pos_mask;
        pos_mask = pos_mask.wrapping_add(0x0001_0001_0001_0001);
    }
    let remainder = qwords.into_remainder();
    let mut rem = 0_u64;
    unsafe {
        core::ptr::copy_nonoverlapping(
            remainder.as_ptr(),
            &mut rem as *mut u64 as *mut u8,
            remainder.len(),
        );
    }
    rem ^= HUMIDIFI_IX_DATA_KEY;
    rem ^= pos_mask;
    unsafe {
        core::ptr::copy_nonoverlapping(
            &rem as *const u64 as *const u8,
            remainder.as_mut_ptr(),
            remainder.len(),
        )
    }
}
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SwapParams {
    pub swap_id: u64,
    pub amount_in: u64,
    pub is_base_to_quote: u8,
    pub padding: [u8; 7],
}

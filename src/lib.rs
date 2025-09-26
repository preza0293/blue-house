#![no_std]
mod common;
mod swap_handler;
use crate::swap_handler::{
    lifinity::LifinitySwapAccounts, meteora_damm_v2::DammV2SwapAccounts,
    meteora_dlmm::DlmmSwapAccounts, obric_v2::ObricV2SwapAccounts, pancake_v3::PancakeSwapAccounts,
    phoniex::PhoniexSwapAccounts, pump::PumpSwapAccounts, ray_amm::RaySwapAccounts,
    ray_clmm::RayClmmSwapAccounts, ray_cpmm::RayCpmmSwapAccounts, saros::SarosDlmmSwapAccounts,
    saros::SarosSwapAccounts, solfi::SolfiSwapAccounts, stabble::StabbleSwapAccounts,
    vertigo::VertigoSwapAccounts, whirlpool::WhirlpoolSwapV2Accounts,
};
use pinocchio::{ProgramResult, account_info::AccountInfo};
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
    ... swap accounts ...
*/

#[derive(Clone)]
pub struct BaseAccounts {
    pub payer: AccountInfo,
    pub token_a_mint: AccountInfo,
    pub token_a_ata: AccountInfo,
    pub token_a_program: AccountInfo,
    pub system_program: AccountInfo,
    pub ata_program: AccountInfo,
    pub token_b_mint: AccountInfo,
    pub token_b_program: AccountInfo,
    pub token_b_ata: AccountInfo,
}
pub struct Bluehouse {
    base: BaseAccounts,
}
impl Bluehouse {
    pub fn new(base: BaseAccounts) -> Self {
        Self { base }
    }
}
pub trait BluehouseApi {
    fn process_lifinity_swap(
        &self,
        market: &LifinitySwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_meteora_damm_v2_swap(
        &self,
        market: &DammV2SwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_meteora_dlmm_swap(
        &self,
        market: &DlmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_obric_v2_swap(
        &self,
        market: &ObricV2SwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_pancake_v3_swap(
        &self,
        market: &PancakeSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_phoniex_swap(
        &self,
        market: &PhoniexSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_pump_buy(
        &self,
        market: &PumpSwapAccounts,
        amount: u64,
        amount_out: u64,
    ) -> ProgramResult;
    fn process_pump_sell(&self, market: &PumpSwapAccounts, amount: u64) -> ProgramResult;
    fn process_ray_amm_swap(
        &self,
        market: &RaySwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_ray_cpmm_swap(
        &self,
        market: &RayCpmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_ray_clmm_swap(
        &self,
        market: &RayClmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_saros_dlmm_swap(
        &self,
        market: &SarosDlmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_saros_swap(
        &self,
        market: &SarosSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_solfi_swap(
        &self,
        market: &SolfiSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_stabble_swap(
        &self,
        market: &StabbleSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
    fn process_vertigo_sell(&self, market: &VertigoSwapAccounts, amount: u64) -> ProgramResult;
    fn process_vertigo_buy(&self, market: &VertigoSwapAccounts, amount: u64) -> ProgramResult;
    fn process_orca_swap_v2(
        &self,
        market: &WhirlpoolSwapV2Accounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult;
}
impl BluehouseApi for Bluehouse {
    fn process_lifinity_swap(
        &self,
        market: &LifinitySwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::lifinity::process_lifinity_swap(self, market, amount, a_to_b)
    }

    fn process_meteora_damm_v2_swap(
        &self,
        market: &DammV2SwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::meteora_damm_v2::process_meteora_damm_v2_swap(self, market, amount, a_to_b)
    }

    fn process_meteora_dlmm_swap(
        &self,
        market: &DlmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::meteora_dlmm::process_meteora_dlmm_swap(self, market, amount, a_to_b)
    }

    fn process_obric_v2_swap(
        &self,
        market: &ObricV2SwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::obric_v2::process_obric_v2_swap(self, market, amount, a_to_b)
    }

    fn process_pancake_v3_swap(
        &self,
        market: &PancakeSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::pancake_v3::process_pancake_v3_swap(self, market, amount, a_to_b, None)
    }

    fn process_phoniex_swap(
        &self,
        market: &PhoniexSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::phoniex::process_phoniex_swap(self, market, amount, a_to_b)
    }

    fn process_pump_buy(
        &self,
        market: &PumpSwapAccounts,
        amount: u64,
        amount_out: u64,
    ) -> ProgramResult {
        swap_handler::pump::process_pump_buy(self, market, amount, amount_out)
    }

    fn process_pump_sell(&self, market: &PumpSwapAccounts, amount: u64) -> ProgramResult {
        swap_handler::pump::process_pump_sell(self, market, amount)
    }

    fn process_ray_amm_swap(
        &self,
        market: &RaySwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::ray_amm::process_ray_amm_swap(self, market, amount, a_to_b)
    }

    fn process_ray_cpmm_swap(
        &self,
        market: &RayCpmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::ray_cpmm::process_ray_cpmm_swap(self, market, amount, a_to_b)
    }

    fn process_ray_clmm_swap(
        &self,
        market: &RayClmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::ray_clmm::process_ray_cl_swap(self, market, amount, a_to_b, None)
    }
    fn process_saros_dlmm_swap(
        &self,
        market: &SarosDlmmSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::saros::process_saros_dlmm_swap(self, market, amount, a_to_b, None)
    }

    fn process_saros_swap(
        &self,
        market: &SarosSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::saros::process_saros_swap(self, market, amount, a_to_b)
    }

    fn process_solfi_swap(
        &self,
        market: &SolfiSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::solfi::process_solfi_swap(self, market, amount, a_to_b)
    }

    fn process_stabble_swap(
        &self,
        market: &StabbleSwapAccounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::stabble::process_stabble_swap(self, market, amount, a_to_b)
    }

    fn process_vertigo_sell(&self, market: &VertigoSwapAccounts, amount: u64) -> ProgramResult {
        swap_handler::vertigo::process_vertigo_sell(self, market, amount)
    }

    fn process_vertigo_buy(&self, market: &VertigoSwapAccounts, amount: u64) -> ProgramResult {
        swap_handler::vertigo::process_vertigo_buy(self, market, amount)
    }

    fn process_orca_swap_v2(
        &self,
        market: &WhirlpoolSwapV2Accounts,
        amount: u64,
        a_to_b: bool,
    ) -> ProgramResult {
        swap_handler::whirlpool::process_orca_swap_v2(self, market, amount, a_to_b, None)
    }
}

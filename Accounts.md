```console
//
// Meteora DLMM
//
pub struct MeteoraDlmm<'a> {
    pub program_id: AccountInfo<'a>,
    pub event_auth: AccountInfo<'a>,
    pub lb_pair: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
    pub oracle: AccountInfo<'a>,
    pub bin_array_0: AccountInfo<'a>,
    pub bin_array_1: AccountInfo<'a>,
    pub bin_array_2: AccountInfo<'a>,
}

//
// Meteora DAMM v2
//
pub struct MeteoraDammV2<'a> {
    pub program_id: AccountInfo<'a>,
    pub event_auth: AccountInfo<'a>,
    pub pool_auth: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Raydium CPMM
//
pub struct RaydiumCpmm<'a> {
    pub program_id: AccountInfo<'a>,
    pub auth: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub amm_config: AccountInfo<'a>,
    pub observation: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Raydium CLMM
//
pub struct RaydiumClmm<'a> {
    pub program_id: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub amm_config: AccountInfo<'a>,
    pub observation_state: AccountInfo<'a>,
    pub b_extension: AccountInfo<'a>,
    pub vault_x: AccountInfo<'a>,
    pub vault_y: AccountInfo<'a>,
    pub tick_0: AccountInfo<'a>,
    pub tick_1: AccountInfo<'a>,
    pub tick_2: AccountInfo<'a>,
}

//
// Raydium AMM
//
pub struct RaydiumAmm<'a> {
    pub program_id: AccountInfo<'a>,
    pub auth: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Whirlpool
//
pub struct Whirlpool<'a> {
    pub program_id: AccountInfo<'a>,
    pub meme_program_v2: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub oracle: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
    pub tick_array_0: AccountInfo<'a>,
    pub tick_array_1: AccountInfo<'a>,
    pub tick_array_2: AccountInfo<'a>,
}

//
// Lifinity
//
pub struct Lifinity<'a> {
    pub program_id: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub auth: AccountInfo<'a>,
    pub pool_mint: AccountInfo<'a>,
    pub fee_account: AccountInfo<'a>,
    pub oracle_main: AccountInfo<'a>,
    pub oracle_sub: AccountInfo<'a>,
    pub oracle_pc: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Obric v2
//
pub struct ObricV2<'a> {
    pub program_id: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub protocol_fee: AccountInfo<'a>,
    pub second_ref_oracle: AccountInfo<'a>,
    pub third_ref_oracle: AccountInfo<'a>,
    pub x_price_feed: AccountInfo<'a>,
    pub y_price_feed: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Pancake v3
//
pub struct PancakeV3<'a> {
    pub program_id: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub amm_config: AccountInfo<'a>,
    pub observation: AccountInfo<'a>,
    pub bitmap_extension: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
    pub tick_array_0: AccountInfo<'a>,
    pub tick_array_1: AccountInfo<'a>,
    pub tick_array_2: AccountInfo<'a>,
}

//
// Poniex
//
pub struct Poniex<'a> {
    pub program_id: AccountInfo<'a>,
    pub log_auth: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Saros AMM
//
pub struct SarosAmm<'a> {
    pub program_id: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub pool_mint: AccountInfo<'a>,
    pub pool_fee: AccountInfo<'a>,
    pub pool_auth: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Saros DLMM
//
pub struct SarosDlmm<'a> {
    pub program_id: AccountInfo<'a>,
    pub memo_program_v2: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub event_auth: AccountInfo<'a>,
    pub bin_array_lower: AccountInfo<'a>,
    pub bin_array_upper: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Solfi
//
pub struct Solfi<'a> {
    pub program_id: AccountInfo<'a>,
    pub sysvar_ix: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Stabble
//
pub struct Stabble<'a> {
    pub program_id: AccountInfo<'a>,
    pub withdraw_auth: AccountInfo<'a>,
    pub vault_auth: AccountInfo<'a>,
    pub vault: AccountInfo<'a>,
    pub vault_program: AccountInfo<'a>,
    pub beneficiary_token_account: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}

//
// Vertigo
//
pub struct Vertigo<'a> {
    pub program_id: AccountInfo<'a>,
    pub pool: AccountInfo<'a>,
    pub pool_owner: AccountInfo<'a>,
    pub vault_a: AccountInfo<'a>,
    pub vault_b: AccountInfo<'a>,
}
```
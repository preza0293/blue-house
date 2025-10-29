pub mod helper;
use helper::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
#[test]
fn swap() {
    // --- 1. Connect to Solana RPC ---
    // Using mainnet-beta endpoint to fetch real accounts/state
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com ");

    // --- 2. Spin up a local LiteSVM instance ---
    // This acts as a lightweight local Solana VM where we can hydrate accounts
    // and simulate transactions against them.
    let mut litesvm = build_svm().unwrap();
    // --- 3. Fetch wallet + base token accounts ---
    let base_accounts = fetch_base_accounts(&litesvm, &rpc_client);

    // --- 4. Define market-specific accounts (Meteora DAMM v2) ---
    let pool_address = Pubkey::from_str_const("2mZ2VBiG8uPX6kza6WBHxcW5KcwmDvcjyXJctKVsmjSq");
    let pool_auth = Pubkey::from_str_const("HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC");
    let event_auth = Pubkey::from_str_const("3rmHSu74h1ZcmAisVcWerTCiRDQbUrBKmcwptYGjHfet");

    // --- 5. Fetch pool account data from RPC ---
    let pool = rpc_client
        .get_account(&Pubkey::from_str_const(&pool_address.to_string()))
        .unwrap();
    let data = pool.data;

    // Extract vault accounts (base & quote)
    let base_vault = Pubkey::new_from_array(data[232..264].try_into().unwrap());
    let quote_vault = Pubkey::new_from_array(data[264..296].try_into().unwrap());

    // --- 6. Fetch and hydrate market accounts ---
    let market_accounts = fetch_market_accounts(
        vec![pool_address, pool_auth, event_auth, base_vault, quote_vault],
        &rpc_client,
    );

    // Hydrate LiteSVM with accounts
    hydrate_svm(&mut litesvm, base_accounts);
    hydrate_svm(&mut litesvm, market_accounts);

    // --- 7. Prepare transaction account metas ---
    let mut accounts = get_base_accounts(&rpc_client);
    accounts.extend([
        AccountMeta::new_readonly(DAMM_V2_ID, false),
        AccountMeta::new_readonly(event_auth, false),
        AccountMeta::new_readonly(pool_auth, false),
        AccountMeta::new(pool_address, false),
        AccountMeta::new(base_vault, false),
        AccountMeta::new(quote_vault, false),
    ]);

    // --- 8. Encode instruction data ---
    // SwapData { amount_in: 10_000_000, amount_out: 0, a_to_b: false }
    let mut data = Vec::with_capacity(17);
    data.extend_from_slice(&10_000_000u64.to_le_bytes()); // amount_in
    data.extend_from_slice(&0u64.to_le_bytes()); // amount_out
    data.push(0u8); // a_to_b = false

    // --- 9. Execute transaction in LiteSVM ---
    execute_transaction(&mut litesvm, accounts, data);
}

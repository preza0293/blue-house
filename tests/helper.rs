use anyhow::{Error, Result};
use litesvm::LiteSVM;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    message::{VersionedMessage, v0::Message},
    program_pack::Pack,
    pubkey,
    pubkey::Pubkey,
    signature::Keypair,
    signer::EncodableKey,
    system_program,
    sysvar::clock::Clock,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::state::Account as TokenAccount;
const WALLET_PATH: &str = "/home/mubariz/wallnuts/mainnet-keypair.json";

// ─────────────────────────────────────────────────────────────
// Program IDs
// ─────────────────────────────────────────────────────────────

pub const PROGRAM_ID: Pubkey = pubkey!("b1225DYKeTyLGd4SrNZFPcgzCC76Q9qniud7XgwB7C4");

// DEX and protocol program IDs
pub const DLMM_ID: Pubkey = pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");
pub const DAMM_V2_ID: Pubkey = pubkey!("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG");
pub const RAYDIUM_AMM_ID: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
pub const RAYDIUM_CPMM_ID: Pubkey = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
pub const RAYDIUM_CL_ID: Pubkey = pubkey!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
pub const WHIRLPOOL_ID: Pubkey = pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
pub const PUMP_AMM_ID: Pubkey = pubkey!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");
pub const PUMP_FEE_CONFIG_ID: Pubkey = pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ");
pub const LIFINITY_ID: Pubkey = pubkey!("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c");
pub const OBRIC_V2_ID: Pubkey = pubkey!("obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y");
pub const PANCAKE_V3_ID: Pubkey = pubkey!("HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq");
pub const PHOENIX_ID: Pubkey = pubkey!("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");
pub const SAROS_AMM_ID: Pubkey = pubkey!("SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr");
pub const SAROS_DLMM_ID: Pubkey = pubkey!("1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE");
pub const SOLFI_ID: Pubkey = pubkey!("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe");
pub const STABBLE_ID: Pubkey = pubkey!("swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ");
pub const VERTIGO_ID: Pubkey = pubkey!("vrTGoBuy5rYSxAfV3jaRJWHH6nN9WK4NRExGxsk1bCJ");

// Misc IDs
pub const WALLET: Pubkey = pubkey!("5BvrQfDzwjFFjpaAys2KA1a7GuuhLXKJoCWykhsoyHet");
pub const TOKEN_A_MINT: Pubkey = pubkey!("5arEQv5tGJj8UbhnPrc1WL5KMmijCe4FuexCaEXUBAGS");
pub const TOKEN_B_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const ATA_PROGRAM_ID: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

// ─────────────────────────────────────────────────────────────
// 🧠 Utility: Build LiteSVM with selected programs
// ─────────────────────────────────────────────────────────────

pub fn build_svm() -> Result<LiteSVM> {
    let mut svm = LiteSVM::new();

    // Main program under test
    svm.add_program_from_file(PROGRAM_ID, "../blue-house/target/deploy/blue_house.so")?;

    // Load whichever market programs you need for the test
    svm.add_program_from_file(DAMM_V2_ID, "tests/program-files/damm_v2.so")?;

    // Uncomment as needed:
    /*
    svm.add_program_from_file(DLMM_ID, "tests/program-files/meteora_dlmm.so")?;
    svm.add_program_from_file(LIFINITY_ID, "tests/program-files/lifinity.so")?;
    svm.add_program_from_file(OBRIC_V2_ID, "tests/program-files/obric_v2.so")?;
    svm.add_program_from_file(PANCAKE_V3_ID, "tests/program-files/pancake_v3.so")?;
    svm.add_program_from_file(PHOENIX_ID, "tests/program-files/phoenix.so")?;
    svm.add_program_from_file(PUMP_AMM_ID, "tests/program-files/pump_amm.so")?;
    svm.add_program_from_file(PUMP_FEE_CONFIG_ID, "tests/program-files/pump_fee_config.so")?;
    svm.add_program_from_file(RAYDIUM_AMM_ID, "tests/program-files/ray_amm.so")?;
    svm.add_program_from_file(RAYDIUM_CL_ID, "tests/program-files/ray_cl.so")?;
    svm.add_program_from_file(RAYDIUM_CPMM_ID, "tests/program-files/ray_cpmm.so")?;
    svm.add_program_from_file(SAROS_AMM_ID, "tests/program-files/saros_amm.so")?;
    svm.add_program_from_file(SAROS_DLMM_ID, "tests/program-files/saros_dlmm.so")?;
    svm.add_program_from_file(SOLFI_ID, "tests/program-files/solfi.so")?;
    svm.add_program_from_file(STABBLE_ID, "tests/program-files/stabble_swap.so")?;
    svm.add_program_from_file(VERTIGO_ID, "tests/program-files/vertigo.so")?;
    svm.add_program_from_file(WHIRLPOOL_ID, "tests/program-files/whirlpool.so")?;
    */

    Ok(svm)
}

// ─────────────────────────────────────────────────────────────
// 🪙 Utility: Create dummy SPL Token account
// ─────────────────────────────────────────────────────────────

pub fn get_dummy_token_account(
    svm: &LiteSVM,
    owner: Pubkey,
    mint: Pubkey,
    token_program: Pubkey,
) -> Result<Account, Error> {
    let token_account = TokenAccount {
        mint,
        owner,
        amount: 1_000_000_000,
        delegate: None.into(),
        state: spl_token::state::AccountState::Initialized,
        is_native: None.into(),
        delegated_amount: 0,
        close_authority: None.into(),
    };

    let mut data = vec![0; TokenAccount::LEN];
    TokenAccount::pack(token_account, &mut data)?;

    Ok(Account {
        lamports: svm.minimum_balance_for_rent_exemption(TokenAccount::LEN),
        data,
        owner: token_program,
        executable: false,
        rent_epoch: 0,
    })
}

/// Returns the user's associated token accounts and their respective token program owners.
pub fn get_user_associated_token_addresses(
    rpc_client: &RpcClient,
) -> (Pubkey, Pubkey, Pubkey, Pubkey) {
    let mint_owner_a = rpc_client.get_account(&TOKEN_A_MINT).unwrap().owner;
    let wallet_token_account_a =
        get_associated_token_address_with_program_id(&WALLET, &TOKEN_A_MINT, &mint_owner_a);

    println!("Token mint A: {}", TOKEN_A_MINT);
    println!("Wallet ATA A: {}", wallet_token_account_a);

    let mint_owner_b = rpc_client.get_account(&TOKEN_B_MINT).unwrap().owner;
    let wallet_token_account_b =
        get_associated_token_address_with_program_id(&WALLET, &TOKEN_B_MINT, &mint_owner_b);

    println!("Token mint B: {}", TOKEN_B_MINT);
    println!("Wallet ATA B: {}", wallet_token_account_b);

    (
        wallet_token_account_a,
        mint_owner_a,
        wallet_token_account_b,
        mint_owner_b,
    )
}

/// Fetches all base accounts required for the program and hydrates missing ones with dummy data.
pub fn fetch_base_accounts(litesvm: &LiteSVM, rpc_client: &RpcClient) -> Vec<(Pubkey, Account)> {
    let mut accounts = Vec::new();

    let (wallet_ata_a, token_a_program, wallet_ata_b, token_b_program) =
        get_user_associated_token_addresses(rpc_client);

    // --- Token A ---
    let ata_a = rpc_client.get_account(&wallet_ata_a).unwrap_or_else(|_| {
        get_dummy_token_account(litesvm, WALLET, TOKEN_A_MINT, token_a_program).unwrap()
    });
    accounts.push((wallet_ata_a, ata_a));

    // --- Token B ---
    let ata_b = rpc_client.get_account(&wallet_ata_b).unwrap_or_else(|_| {
        get_dummy_token_account(litesvm, WALLET, TOKEN_B_MINT, token_b_program).unwrap()
    });
    accounts.push((wallet_ata_b, ata_b));

    // --- Other base accounts ---
    let base_keys = vec![
        WALLET,
        TOKEN_A_MINT,
        token_a_program,
        TOKEN_B_MINT,
        token_b_program,
        ATA_PROGRAM_ID,
    ];

    let base_accounts = rpc_client.get_multiple_accounts(&base_keys).unwrap();

    for (key, acc_opt) in base_keys.into_iter().zip(base_accounts) {
        if let Some(acc) = acc_opt {
            accounts.push((key, acc));
        }
    }

    accounts
}

/// Inserts a list of accounts into the LiteSVM environment.
pub fn hydrate_svm(litesvm: &mut LiteSVM, accounts: Vec<(Pubkey, Account)>) {
    for (addr, acc) in accounts {
        litesvm.set_account(addr, acc).unwrap();
    }
}

pub fn fetch_market_accounts(
    addresses: Vec<Pubkey>,
    rpc_client: &RpcClient,
) -> Vec<(Pubkey, Account)> {
    let mut accounts: Vec<(Pubkey, Account)> = Vec::new();
    let base_accounts = rpc_client.get_multiple_accounts(&addresses).unwrap();

    for (i, acc_opt) in base_accounts.into_iter().enumerate() {
        if let Some(acc) = acc_opt {
            accounts.push((addresses[i], acc));
        }
    }
    accounts
}
/// Builds the list of base `AccountMeta` entries required for instruction construction.
pub fn get_base_accounts(rpc_client: &RpcClient) -> Vec<AccountMeta> {
    let (ata_a, token_a_prog, ata_b, token_b_prog) =
        get_user_associated_token_addresses(rpc_client);

    vec![
        AccountMeta::new(WALLET, true),                 // Wallet (signer)
        AccountMeta::new_readonly(TOKEN_A_MINT, false), // Token A mint
        AccountMeta::new(ata_a, false),                 // Wallet ATA A
        AccountMeta::new_readonly(token_a_prog, false), // Token A program
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(ATA_PROGRAM_ID, false),
        AccountMeta::new_readonly(TOKEN_B_MINT, false), // Token B mint
        AccountMeta::new_readonly(token_b_prog, false), // Token B program
        AccountMeta::new(ata_b, false),                 // Wallet ATA B
    ]
}

/// Executes a transaction locally in the LiteSVM sandbox and prints the program logs.
pub fn execute_transaction(litesvm: &mut LiteSVM, accounts: Vec<AccountMeta>, data: Vec<u8>) {
    let payer = Keypair::read_from_file(WALLET_PATH).unwrap();

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts,
        data,
    };

    let message = Message::try_compile(&WALLET, &[ix], &[], litesvm.latest_blockhash()).unwrap();
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(message), &[payer]).unwrap();

    // Set deterministic timestamp
    let mut clock = litesvm.get_sysvar::<Clock>();
    clock.unix_timestamp = 1_772_448_000; //some programs require specififc timestamp to allow swap
    litesvm.set_sysvar::<Clock>(&clock);

    // Execute transaction
    let result = litesvm.send_transaction(tx).unwrap();
    println!("{:#?}", result.logs);
}

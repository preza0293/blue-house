# Blue House

![blue_house2](https://github.com/user-attachments/assets/eb6082e3-2708-4ee3-9412-4bba07c8b131)

**Blue House** is a Solana program that provides a unified interface for performing swaps across multiple decentralized exchanges (DEXes).  
Instead of implementing each market individually in your own program or trading bot, you can simply call **Blue House** — either directly in a transaction or through a CPI (cross-program invocation).  

---

## Program ID
The program is deployed at:
```console
b1225DYKeTyLGd4SrNZFPcgzCC76Q9qniud7XgwB7C4
```
Features

🔄 Unified Swaps — One interface for 15+ Solana DEXs and protocols

⚡ Low Overhead — Minimal abstractions to preserve performance

🛠 Extensible — Easily add support for more markets in the future
## Base Accounts
``` console
        wallet           0
        token_a_mint     1
        token_a_ata      2
        token_program    3
        sys_program      4
        ata_program      5
        token_b_mint     6
        token_b_program  7
        token_b_ata      8
```
## Example
```rust
    let ata_info = get_user_associated_token_addresses(&rpc_client);
    let mut accounts = vec![
        AccountMeta::new(WALLET, true), // 0. Wallet (signer)
        AccountMeta::new_readonly(TOKEN_A_MINT, false),
        AccountMeta::new(ata_info.0, false),          //ATA_A
        AccountMeta::new_readonly(ata_info.1, false), //TOKEN_PROGRAM_A
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(ATA_PROGRAM, false),
        AccountMeta::new_readonly(TOKEN_B_MINT, false),
        AccountMeta::new_readonly(ata_info.3, false), //TOKEN_PROGRAM_B
        AccountMeta::new(ata_info.2, false),
    ];
    accounts.push(AccountMeta::new_readonly(DAMM_V2_ID, false));
    accounts.push(AccountMeta::new_readonly(event_auth, false));
    accounts.push(AccountMeta::new_readonly(pool_auth, false));
    accounts.push(AccountMeta::new(pool_address, false));
    accounts.push(AccountMeta::new(base_vault, false));
    accounts.push(AccountMeta::new(quote_vault, false));

    let mut data = vec![17u8];
    data.extend_from_slice(&10000000u64.to_le_bytes()); // amount_in
    data.extend_from_slice(&0u64.to_le_bytes()); // amount_out_min
    data.extend_from_slice(&0u8.to_le_bytes()); // a_to_b = false
```
Dex Accounts details [here](https://github.com/mubarizkyc/blue-house/blob/main/Accounts.md)

[Simulation Example](https://github.com/mubarizkyc/blue-house/tree/main/src)

[Telegram](https://t.me/kyc1104)


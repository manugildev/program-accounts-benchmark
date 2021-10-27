use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, MemcmpEncoding, RpcFilterType};
use solana_program::pubkey::Pubkey;
use solana_sdk::{ account::Account, commitment_config::CommitmentConfig };

use std::time::{Duration, Instant};
use std::str::FromStr;

const RPC_URL : &str = "https://api.mainnet-beta.solana.com";
const SOLEND_PROGRAM: &str = "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo";
const LENDING_MARKET: &str = "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY";
const OBLIGATION_LEN: u64 = 1300;

fn main() {
    println!("=======================================");
    println!("Running Rust benchmark");
    let start_time = Instant::now();
    let accounts = get_obligations();
    let duration = start_time.elapsed();
    println!("Retrieved {} accounts in {}ms.", accounts.len(), duration.as_millis());
}

fn get_obligations() -> Vec<(Pubkey, Account)> {
    let client = RpcClient::new_with_timeout_and_commitment(RPC_URL.to_string(), Duration::from_secs(320), CommitmentConfig::confirmed());
    let solend_program_pk = Pubkey::from_str(SOLEND_PROGRAM).unwrap();
    let program_accounts_config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::Memcmp(Memcmp {
                offset: 10,
                bytes: MemcmpEncodedBytes::Base58(LENDING_MARKET.to_string()),
                encoding: Some(MemcmpEncoding::Binary),
            }),
            RpcFilterType::DataSize(OBLIGATION_LEN),
        ]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(client.commitment()),
            ..RpcAccountInfoConfig::default()
        },
        ..RpcProgramAccountsConfig::default()
    };

    // Get Obligations
    let program_accounts = client.get_program_accounts_with_config(&solend_program_pk, program_accounts_config)
        .unwrap();
    return program_accounts;
}

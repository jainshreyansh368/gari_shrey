use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;
use chrono::{Local, NaiveDate, TimeZone};

const PUBKEY: &str = "CKaKtYvz6dKPyMvYq9Rh3UBrnNqYZAyd7iF4hJtjUvks";


pub async fn get_signatures(
    client: &RpcClient,
) -> Vec<RpcConfirmedTransactionStatusWithSignature> {
    // fetch 1000 signatures for tokens
    let configure = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(1000),
        commitment: Some(CommitmentConfig::confirmed()),
    };

    client
        .get_signatures_for_address_with_config(&Pubkey::from_str(PUBKEY).unwrap(), configure)
        .await.unwrap()
}

pub async fn fetch_txn(
    client: &RpcClient,
    signatures: &Vec<RpcConfirmedTransactionStatusWithSignature>,
) -> Vec<EncodedConfirmedTransactionWithStatusMeta> {
    let mut txs: Vec<EncodedConfirmedTransactionWithStatusMeta> = Vec::new();

    for sig in signatures {
        txs.push(
            client
                .get_transaction(
                    &Signature::from_str(&sig.signature).unwrap(),
                    UiTransactionEncoding::JsonParsed,
                )
                .await.unwrap(),
        );
    }
    
    txs
}

pub fn get_client() -> RpcClient {
    RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    )
}

pub fn sort_data(
    txs: Vec<EncodedConfirmedTransactionWithStatusMeta>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Vec<EncodedConfirmedTransactionWithStatusMeta> {
    let filtered_data: Vec<EncodedConfirmedTransactionWithStatusMeta> = txs
        .into_iter()
        .filter(|txn| {
            let transaction_date = Local
                .timestamp_opt(
                    txn.block_time
                        .unwrap(),
                    0,
                )
                .unwrap()
                .date_naive();

            transaction_date >= start_date && transaction_date <= end_date
        })
        .collect();

    filtered_data
}


use alloy::primitives::{Address, ChainId, U256, hex, FixedBytes};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use crate::{tx_signing::{TxInput, tx_signing_hashes}, cli::TransactionArgs};
use safe_utils::SafeWalletVersion;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeTransaction {
    pub safe: Address,
    pub to: Address,
    pub value: String,
    pub data: String,
    pub data_decoded: Option<DataDecoded>,
    pub operation: u8,
    pub gas_token: Address,
    pub safe_tx_gas: u64,
    pub base_gas: u64,
    pub gas_price: String,
    pub refund_receiver: Address,
    pub nonce: u64,
    pub safe_tx_hash: String,
    pub confirmations_required: u64,
    pub confirmations: Vec<Confirmation>,
    pub signatures: String,
    pub proposer: Option<Address>,
    pub proposed_by_delegate: Option<Address>,
    pub execution_date: Option<String>,
    pub submission_date: String,
    pub modified: String,
    pub block_number: u64,
    pub transaction_hash: Option<String>,
    pub executor: Option<Address>,
    pub is_executed: bool,
    pub is_successful: bool,
    pub eth_gas_price: String,
    pub gas_used: u64,
    pub fee: String,
    pub origin: String,
    pub trusted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataDecoded {
    pub method: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub r#type: String,
    pub value: String,
    pub value_decoded: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Confirmation {
    pub owner: Address,
    pub submission_date: String,
    pub transaction_hash: Option<String>,
    pub signature_type: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafeApiResponse {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<SafeTransaction>,
}

const API_BASE_URL: &str = "https://safe-client.safe.global/v1";

pub fn get_safe_transaction(chain_id: u64, safe_address: Address, nonce: u64) -> Result<SafeTransaction, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/chains/{}/safes/{}/multisig-transactions/raw?nonce={}",
        API_BASE_URL, chain_id, safe_address, nonce
    );
    println!("Fetching transaction from API: {}", url);
    let response = reqwest::blocking::get(&url)?;
    let api_response: SafeApiResponse = response.json()?;

    if api_response.count == 0 {
        return Err("No transaction found for the specified nonce".into());
    }

    if api_response.count > 1 {
        return Err("Multiple transactions found for the specified nonce. Please specify more details to identify the correct transaction.".into());
    }

    Ok(api_response.results[0].clone())
}

pub fn validate_transaction_details(
    api_tx: &SafeTransaction,
    user_to: Option<Address>,
    user_value: Option<U256>,
    user_data: Option<String>,
) -> Result<(), String> {
    if let Some(to) = user_to {
        if to != api_tx.to {
            return Err(format!(
                "Transaction 'to' address mismatch. API: {}, User provided: {}",
                api_tx.to, to
            ));
        }
    }

    if let Some(value) = user_value {
        let api_value = U256::from_str_radix(&api_tx.value, 10)
            .map_err(|e| format!("Failed to parse API value: {}", e))?;
        if value != api_value {
            return Err(format!(
                "Transaction value mismatch. API: {}, User provided: {}",
                api_value, value
            ));
        }
    }

    if let Some(data) = user_data {
        if data != api_tx.data {
            return Err(format!(
                "Transaction data mismatch. API: {}, User provided: {}",
                api_tx.data, data
            ));
        }
    }

    Ok(())
}

pub fn validate_safe_tx_hash(
    api_tx: &SafeTransaction,
    calculated_hash: &FixedBytes<32>,
) -> Result<(), String> {
    // Remove 0x prefix if present and parse as hex
    let api_hash = U256::from_str_radix(api_tx.safe_tx_hash.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Failed to parse API safe_tx_hash: {}", e))?;
    let calculated_bytes: [u8; 32] = calculated_hash.as_slice().try_into().unwrap();
    if api_hash != U256::from_be_bytes(calculated_bytes) {
        return Err(format!(
            "Safe Transaction Hash mismatch. API: {}, Calculated: {}",
            api_tx.safe_tx_hash,
            hex::encode(calculated_hash)
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_decode_api_response() {
        let json = fs::read_to_string("../../test/client_tx_response.json").expect("Failed to read test file");
        let response: SafeApiResponse = serde_json::from_str(&json).expect("Failed to decode JSON");
        
        assert_eq!(response.count, 1);
        assert_eq!(response.results.len(), 1);
        
        let tx = &response.results[0];
        assert_eq!(tx.safe, Address::from_str("0x1c694Fc3006D81ff4a56F97E1b99529066a23725").unwrap());
        assert_eq!(tx.to, Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap());
        assert_eq!(tx.value, "0");
        assert_eq!(tx.nonce, 63);
        assert_eq!(tx.confirmations_required, 2);
        assert_eq!(tx.confirmations.len(), 2);
        assert!(tx.is_executed);
        assert!(tx.is_successful);
    }
} 
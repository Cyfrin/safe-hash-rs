use alloy::primitives::{Address, ChainId, U256, hex, FixedBytes};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use crate::{cli::TransactionArgs, output::Mismatch, tx_signing::{tx_signing_hashes, TxInput}};
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
    user_args: &TransactionArgs,
) -> Result<(), Vec<Mismatch>> {
    let mut errors = Vec::new();

    if let Some(to) = user_args.to {
        if to != api_tx.to {
            errors.push(Mismatch {
                field: "to".to_string(),
                api_value: api_tx.to.to_string(),
                user_value: to.to_string(),
            });
        }
    }

    // If user_args.value is not zero, validate it
    if user_args.value != U256::ZERO {
        match U256::from_str_radix(&api_tx.value, 10) {
            Ok(api_value) => {
                if user_args.value != api_value {
                    errors.push(Mismatch {
                        field: "value".to_string(),
                        api_value: api_value.to_string(),
                        user_value: user_args.value.to_string(),
                    });
                }
            }
            Err(e) => {
                errors.push(Mismatch {
                    field: "value".to_string(),
                    api_value: "".to_string(),
                    user_value: format!("Failed to parse API value: {}", e),
                });
            }
        }
    }

    if user_args.data != "0x" {
        if user_args.data != api_tx.data {
            errors.push(Mismatch {
                field: "data".to_string(),
                api_value: api_tx.data.clone(),
                user_value: user_args.data.clone(),
            });
        }
    }

    if user_args.operation != api_tx.operation {
        errors.push(Mismatch {
            field: "operation".to_string(),
            api_value: api_tx.operation.to_string(),
            user_value: user_args.operation.to_string(),
        });
    }

    if user_args.gas_token != Address::ZERO {
        if user_args.gas_token != api_tx.gas_token {
            errors.push(Mismatch {
                field: "gas_token".to_string(),
                api_value: api_tx.gas_token.to_string(),
                user_value: user_args.gas_token.to_string(),
            });
        }
    }

    if user_args.refund_receiver != Address::ZERO {
        if user_args.refund_receiver != api_tx.refund_receiver {
            errors.push(Mismatch {
                field: "refund_receiver".to_string(),
                api_value: api_tx.refund_receiver.to_string(),
                user_value: user_args.refund_receiver.to_string(),
            });
        }
    }

    if user_args.safe_tx_gas != U256::ZERO {
        if user_args.safe_tx_gas != U256::from(api_tx.safe_tx_gas) {
            errors.push(Mismatch {
                field: "safe_tx_gas".to_string(),
                api_value: api_tx.safe_tx_gas.to_string(),
                user_value: user_args.safe_tx_gas.to_string(),
            });
        }
    }

    if user_args.base_gas != U256::ZERO {
        if user_args.base_gas != U256::from(api_tx.base_gas) {
            errors.push(Mismatch {
                field: "base_gas".to_string(),
                api_value: api_tx.base_gas.to_string(),
                user_value: user_args.base_gas.to_string(),
            });
        }
    }

    if user_args.gas_price != U256::ZERO {
        if user_args.gas_price != U256::from_str_radix(&api_tx.gas_price, 10).unwrap_or(U256::ZERO) {
            errors.push(Mismatch {
                field: "gas_price".to_string(),
                api_value: api_tx.gas_price.clone(),
                user_value: user_args.gas_price.to_string(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_safe_tx_hash(
    api_tx: &SafeTransaction,
    calculated_hash: &FixedBytes<32>,
) -> Result<(), Mismatch> {
    // Remove 0x prefix if present and parse as hex
    let api_hash = U256::from_str_radix(api_tx.safe_tx_hash.trim_start_matches("0x"), 16)
        .map_err(|e| Mismatch {
            field: "safe_tx_hash".to_string(),
            api_value: "".to_string(),
            user_value: format!("Failed to parse API safe_tx_hash: {}", e),
        })?;
    let calculated_bytes: [u8; 32] = calculated_hash.as_slice().try_into().unwrap();
    if api_hash != U256::from_be_bytes(calculated_bytes) {
        return Err(Mismatch {
            field: "safe_tx_hash".to_string(),
            api_value: api_tx.safe_tx_hash.clone(),
            user_value: hex::encode(calculated_hash),
        });
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

    fn create_test_tx() -> SafeTransaction {
        SafeTransaction {
            safe: Address::from_str("0x1c694Fc3006D81ff4a56F97E1b99529066a23725").unwrap(),
            to: Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap(),
            value: "1000000".to_string(),
            data: "0x1234".to_string(),
            data_decoded: None,
            operation: 0,
            gas_token: Address::ZERO,
            safe_tx_gas: 100000,
            base_gas: 50000,
            gas_price: "1000000000".to_string(),
            refund_receiver: Address::ZERO,
            nonce: 1,
            safe_tx_hash: "0x1234".to_string(),
            confirmations_required: 2,
            confirmations: vec![],
            signatures: "".to_string(),
            proposer: None,
            proposed_by_delegate: None,
            execution_date: None,
            submission_date: "".to_string(),
            modified: "".to_string(),
            block_number: 1,
            transaction_hash: None,
            executor: None,
            is_executed: false,
            is_successful: false,
            eth_gas_price: "".to_string(),
            gas_used: 0,
            fee: "".to_string(),
            origin: "".to_string(),
            trusted: false,
        }
    }

    #[test]
    fn test_validate_transaction_details_no_mismatches() {
        let api_tx = create_test_tx();
        let user_args = TransactionArgs {
            to: Some(api_tx.to),
            value: U256::from_str_radix(&api_tx.value, 10).unwrap(),
            data: api_tx.data.clone(),
            operation: api_tx.operation,
            gas_token: api_tx.gas_token,
            safe_tx_gas: U256::from(api_tx.safe_tx_gas),
            base_gas: U256::from(api_tx.base_gas),
            gas_price: U256::from_str_radix(&api_tx.gas_price, 10).unwrap(),
            refund_receiver: api_tx.refund_receiver,
            ..Default::default()
        };

        assert!(validate_transaction_details(&api_tx, &user_args).is_ok());
    }

    #[test]
    fn test_validate_transaction_details_to_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.to = Some(Address::from_str("0x0000000000000000000000000000000000000001").unwrap());

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "to");
        assert_eq!(result[0].api_value, api_tx.to.to_string());
        assert_eq!(result[0].user_value, user_args.to.unwrap().to_string());
    }

    #[test]
    fn test_validate_transaction_details_value_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.value = U256::from(2000000);

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "value");
        assert_eq!(result[0].api_value, api_tx.value);
        assert_eq!(result[0].user_value, user_args.value.to_string());
    }

    #[test]
    fn test_validate_transaction_details_data_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.data = "0x5678".to_string();

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "data");
        assert_eq!(result[0].api_value, api_tx.data);
        assert_eq!(result[0].user_value, user_args.data);
    }

    #[test]
    fn test_validate_transaction_details_operation_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.operation = 1;

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "operation");
        assert_eq!(result[0].api_value, api_tx.operation.to_string());
        assert_eq!(result[0].user_value, user_args.operation.to_string());
    }

    #[test]
    fn test_validate_transaction_details_gas_token_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.gas_token = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "gas_token");
        assert_eq!(result[0].api_value, api_tx.gas_token.to_string());
        assert_eq!(result[0].user_value, user_args.gas_token.to_string());
    }

    #[test]
    fn test_validate_transaction_details_refund_receiver_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.refund_receiver = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "refund_receiver");
        assert_eq!(result[0].api_value, api_tx.refund_receiver.to_string());
        assert_eq!(result[0].user_value, user_args.refund_receiver.to_string());
    }

    #[test]
    fn test_validate_transaction_details_safe_tx_gas_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.safe_tx_gas = U256::from(200000);

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "safe_tx_gas");
        assert_eq!(result[0].api_value, api_tx.safe_tx_gas.to_string());
        assert_eq!(result[0].user_value, user_args.safe_tx_gas.to_string());
    }

    #[test]
    fn test_validate_transaction_details_base_gas_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.base_gas = U256::from(100000);

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "base_gas");
        assert_eq!(result[0].api_value, api_tx.base_gas.to_string());
        assert_eq!(result[0].user_value, user_args.base_gas.to_string());
    }

    #[test]
    fn test_validate_transaction_details_gas_price_mismatch() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.gas_price = U256::from(2000000000);

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field, "gas_price");
        assert_eq!(result[0].api_value, api_tx.gas_price);
        assert_eq!(result[0].user_value, user_args.gas_price.to_string());
    }

    #[test]
    fn test_validate_transaction_details_multiple_mismatches() {
        let api_tx = create_test_tx();
        let mut user_args = TransactionArgs::default();
        user_args.to = Some(Address::from_str("0x0000000000000000000000000000000000000001").unwrap());
        user_args.value = U256::from(2000000);
        user_args.data = "0x5678".to_string();

        let result = validate_transaction_details(&api_tx, &user_args).unwrap_err();
        assert_eq!(result.len(), 3);
        
        // Check all mismatches are present
        let fields: Vec<_> = result.iter().map(|m| &m.field).collect();
        assert!(fields.contains(&&"to".to_string()));
        assert!(fields.contains(&&"value".to_string()));
        assert!(fields.contains(&&"data".to_string()));
    }
} 
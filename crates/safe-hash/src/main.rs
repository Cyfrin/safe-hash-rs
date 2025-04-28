mod api;
mod cli;
mod etherscan;
mod msg_signing;
mod output;
mod tx_signing;
mod warn;

use alloy::primitives::{ChainId, U256};
use clap::Parser;
use cli::{CliArgs, Mode};
use msg_signing::*;
use output::{
    SafeWarnings, display_api_transaction_details, display_eip712_hash, display_hashes,
    display_warnings,
};
use safe_utils::{Eip712Hasher, Of};
use std::fs;
use tx_signing::*;
use warn::check_suspicious_content;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();

    match args.mode {
        Mode::Transaction(tx_args) => {
            let chain_id = ChainId::of(&tx_args.chain).unwrap_or_else(|_| {
                panic!("chain {:?} is supported but id is not found", tx_args.chain)
            });

            // Try to get transaction details from API
            let api_tx = match api::get_safe_transaction(
                chain_id,
                tx_args.safe_address,
                tx_args.nonce as u64,
            ) {
                Ok(tx) => Some(tx),
                Err(e) => {
                    eprintln!("Warning: Could not fetch transaction from API: {}", e);
                    eprintln!("Falling back to offline mode with provided parameters");
                    None
                }
            };

            let mut warnings = SafeWarnings::new();
            let tx_data = if let Some(api_tx) = &api_tx {
                // Display API transaction details
                display_api_transaction_details(api_tx);

                // Validate that user-provided details match API data if any were provided
                if let Err(errors) = api::validate_transaction_details(api_tx, &tx_args) {
                    if !errors.is_empty() {
                        warnings.argument_mismatches.extend(errors);
                        display_warnings(&warnings);
                        return;
                    }
                }

                // Use API data for transaction
                TxInput::new(
                    api_tx.to,
                    U256::from_str_radix(&api_tx.value, 10).unwrap_or(U256::ZERO),
                    api_tx.data.clone(),
                    api_tx.operation,
                    U256::from(api_tx.safe_tx_gas),
                    U256::from(api_tx.base_gas),
                    U256::from_str_radix(&api_tx.gas_price, 10).unwrap_or(U256::ZERO),
                    api_tx.gas_token,
                    api_tx.refund_receiver,
                    api_tx.signatures.clone(),
                )
            } else {
                // Use user-provided data for transaction
                TxInput::new(
                    tx_args.to.unwrap_or_else(|| {
                        panic!("'--to' address is required in offline mode. When API data cannot be fetched, you must provide the destination address manually.")
                    }),
                    tx_args.value,
                    tx_args.data.clone(),
                    tx_args.operation,
                    tx_args.safe_tx_gas,
                    tx_args.base_gas,
                    tx_args.gas_price,
                    tx_args.gas_token,
                    tx_args.refund_receiver,
                    String::new(), // No signatures needed for signing
                )
            };

            // Calculate hashes
            let hashes =
                tx_signing_hashes(&tx_data, &tx_args, chain_id, tx_args.safe_version.clone());

            // Validate Safe Transaction Hash against API data if available
            if let Some(api_tx) = &api_tx {
                if let Err(e) = api::validate_safe_tx_hash(api_tx, &hashes.safe_tx_hash) {
                    warnings.argument_mismatches.push(e);
                }
            }

            // Display hashes
            display_hashes(&hashes);

            // Check for suspicious content and union warnings
            warnings.union(check_suspicious_content(&tx_data, Some(chain_id)));

            // Display warnings after the hashes
            display_warnings(&warnings);
        }
        Mode::Message(msg_args) => {
            let chain_id = ChainId::of(&msg_args.chain).unwrap_or_else(|_| {
                panic!("chain {:?} is supported but id is not found", msg_args.chain)
            });

            let message = fs::read_to_string(&msg_args.input_file)
                .unwrap_or_else(|_| panic!("Failed to read message file: {}", msg_args.input_file));
            let msg_data = MsgInput::new(message);
            let hashes = msg_signing_hashes(&msg_data, &msg_args, chain_id);
            display_hashes(&hashes);
        }
        Mode::Eip712(eip712_args) => {
            let message = fs::read_to_string(&eip712_args.file).unwrap_or_else(|_| {
                panic!("Failed to read file: {}", eip712_args.file.as_os_str().to_string_lossy())
            });
            let msg_data = Eip712Hasher::new(message);
            display_eip712_hash(&msg_data.hash().expect("Failed to EIP 712 Hash"));
        }
    }
}

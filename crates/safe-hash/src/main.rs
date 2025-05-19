mod api;
mod cli;
mod etherscan;
mod msg_signing;
mod output;
mod tx_signing;
mod warn;

use alloy::{
    hex::{self},
    primitives::{Address, B256, ChainId, U256},
};
use clap::Parser;
use cli::{CliArgs, Eip712Args, Mode};
use msg_signing::*;
use output::{
    SafeWarnings, display_api_transaction_details, display_eip712_hash, display_full_tx,
    display_hashes, display_warnings,
};
use safe_utils::{DomainHasher, Eip712Hasher, FullTx, MessageHasher, Of};
use std::fs;
use tx_signing::*;
use warn::check_suspicious_content;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();
    args.validate_to_for_offline();
    args.validate_eip712_args();

    match args.mode {
        Mode::Transaction(tx_args) => {
            let chain_id = ChainId::of(&tx_args.chain).unwrap_or_else(|_| {
                panic!("chain {:?} is supported but id is not found", tx_args.chain)
            });

            // Try to get transaction details from API
            let api_tx = if tx_args.offline {
                Ok(None)
            } else {
                match api::get_safe_transaction(
                    chain_id,
                    tx_args.safe_address,
                    tx_args.nonce as u64,
                ) {
                    Ok(tx) => Ok(Some(tx)),
                    Err(e) => {
                        eprintln!("Warning: Could not fetch transaction from API: {}", e);
                        eprintln!("Falling back to offline mode with provided parameters");
                        Err(e)
                    }
                }
            };

            let mut warnings = SafeWarnings::new();
            let tx_data = if let Ok(Some(api_tx)) = &api_tx {
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
                    tx_args.to.expect("--to not found"),
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

            if let Ok(Some(api_tx)) = &api_tx {
                let full_tx = FullTx::new(
                    api_tx.to,
                    U256::from_str_radix(&api_tx.value, 10).unwrap_or(U256::ZERO),
                    api_tx.data.clone(),
                    api_tx.operation,
                    U256::from(api_tx.safe_tx_gas),
                    U256::from(api_tx.base_gas),
                    U256::from_str_radix(&api_tx.gas_price, 10).unwrap_or(U256::ZERO),
                    api_tx.gas_token,
                    api_tx.refund_receiver,
                    U256::from(api_tx.nonce),
                    api_tx.signatures.clone(),
                );
                display_full_tx(full_tx.calldata(), full_tx.calldata_hash().unwrap_or_default());
            }
            // Calculate hashes
            let hashes = tx_signing_hashes(
                &tx_data,
                tx_args.safe_address,
                tx_args.nonce,
                chain_id,
                tx_args.safe_version.clone(),
            );

            let nested_tx_data: Option<TxInput> =
                match (tx_args.nested_safe_address, tx_args.nested_safe_nonce) {
                    (Some(_nested_safe_address), Some(_)) => {
                        let data = format!("0xd4d9bdcd{}", hex::encode(hashes.safe_tx_hash));
                        Some(TxInput::new(
                            tx_args.safe_address,
                            U256::ZERO,
                            data,
                            0,
                            U256::ZERO,
                            U256::ZERO,
                            U256::ZERO,
                            Address::ZERO,
                            Address::ZERO,
                            String::new(),
                        ))
                    }
                    (_, _) => None,
                };

            // Validate Safe Transaction Hash against API data if available
            if let Ok(Some(api_tx)) = &api_tx {
                // Display API transaction details
                display_api_transaction_details(api_tx);

                let dangerous_methods =
                    ["addOwnerWithThreshold", "removeOwner", "swapOwner", "changeThreshold"];

                if let Some(decoded) = &api_tx.data_decoded {
                    if dangerous_methods.iter().any(|m| *m == decoded.method) {
                        warnings.dangerous_methods = true;
                    }
                }

                if let Err(e) = api::validate_safe_tx_hash(api_tx, &hashes.safe_tx_hash) {
                    warnings.argument_mismatches.push(e);
                }
            }

            // Check for suspicious content and union warnings
            warnings.union(check_suspicious_content(&tx_data, Some(chain_id)));

            // Display hashes
            println!("\nMain transaction");
            display_hashes(&hashes);

            // Calculate nested hashes
            if let Some(nested_tx_data) = nested_tx_data {
                let nhashes = tx_signing_hashes(
                    &nested_tx_data,
                    tx_args.nested_safe_address.expect("--nested-safe-address not provided"),
                    tx_args.nested_safe_nonce.expect("--nested-safe-none not provided"),
                    chain_id,
                    tx_args.safe_version.clone(),
                );

                println!("\nNested transaction");
                display_hashes(&nhashes);
            }

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
            let message = msg_data.hash().expect("Failed to EIP 712 hash");
            display_eip712_hash(&message);

            let Eip712Args { safe_version, chain, safe_address, standalone, .. } = eip712_args;

            if !standalone {
                let msg_hash = {
                    let msg_hasher = MessageHasher::new_from_bytes(B256::from_slice(
                        &hex::decode(message.eip_712_hash.clone()).unwrap(),
                    ));

                    msg_hasher.hash()
                };

                let domain_hash = {
                    let domain_hasher = DomainHasher::new(
                        safe_version.unwrap(),
                        ChainId::of(&chain.unwrap()).unwrap(),
                        safe_address.unwrap(),
                    );

                    domain_hasher.hash()
                };

                println!("Safe UI values");
                println!("Domain Hash {}", domain_hash);
                println!("Message Hash {}", msg_hash);
            }
        }
    }
}

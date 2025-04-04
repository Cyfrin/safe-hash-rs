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

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn test_safe_hash_tx_signing_cli_output() {
        // Run the safe-hash command with some test arguments
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("tx")
            .arg("--chain")
            .arg("ethereum")
            .arg("--nonce")
            .arg("63")
            .arg("--safe-address")
            .arg("0x1c694Fc3006D81ff4a56F97E1b99529066a23725")
            .arg("--to")
            .arg("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")
            .arg("--data")
            .arg("0xa9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00")
            .output()
            .expect("Failed to execute command");

        // Assert that the command executed successfully
        assert!(
            output.status.success(),
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        // Check for essential content without formatting
        // Domain hash
        assert!(
            stdout.contains("1655e94a9bcc5a957daa1acae692b4c22e7aaf146b4deb9194f8221d2f09d8c3")
        );
        // Message hash
        assert!(
            stdout.contains("f22754eba5a2b230714534b4657195268f00dc0031296de4b835d82e7aa1e574")
        );
        // Safe transaction hash
        assert!(
            stdout.contains("ad06b099fca34e51e4886643d95d9a19ace2cd024065efb66662a876e8c40343")
        );
    }

    #[test]
    fn test_safe_hash_msg_signing_cli_output() {
        // Run the safe-hash command with some test arguments
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("msg")
            .arg("--chain")
            .arg("sepolia")
            .arg("--safe-address")
            .arg("0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1")
            .arg("--input-file")
            .arg("../../test/test_message.txt")
            .output()
            .expect("Failed to execute command");

        // Assert that the command executed successfully
        assert!(
            output.status.success(),
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        // Check for essential content without formatting
        // Domain hash
        assert!(
            stdout.contains("611379c19940caee095cdb12bebe6a9fa9abb74cdb1fbd7377c49a1f198dc24f")
        );
        // Message hash
        assert!(
            stdout.contains("a5d2f507a16279357446768db4bd47a03bca0b6acac4632a4c2c96af20d6f6e5")
        );
        // Safe transaction hash
        assert!(
            stdout.contains("1866b559f56261ada63528391b93a1fe8e2e33baf7cace94fc6b42202d16ea08")
        );
    }

    #[test]
    fn test_eip712_hash() {
        // Run the safe-hash command with some test arguments
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("typed")
            .arg("--file")
            .arg("../../test/eip712_message.json")
            .output()
            .expect("Failed to execute command");

        // Assert that the command executed successfully
        assert!(
            output.status.success(),
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        // Check for essential content without formatting
        // Check for hash
        assert!(
            stdout.contains("a85c2e2b118698e88db68a8105b794a8cc7cec074e89ef991cb4f5f533819cc2")
        );
    }
}

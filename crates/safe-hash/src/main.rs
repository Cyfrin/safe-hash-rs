mod cli;
mod etherscan;
mod output;
mod tx_signing;
mod tx_file;
mod warn;

use alloy::primitives::ChainId;
use clap::Parser;
use cli::{CliArgs, Mode};
use output::{display_hashes, display_warnings};
use safe_utils::{Of, SafeWalletVersion};
use tx_signing::*;
use tx_file::TxInput;
use warn::check_suspicious_content;
use std::fs;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();

    match args.mode {
        
        Mode::Transaction(tx_args) => {
            let chain_id = ChainId::of(&tx_args.chain)
                .unwrap_or_else(|_| panic!("chain {:?} is supported but id is not found", tx_args.chain));

            let tx_data = TxInput::new(
                tx_args.to,
                tx_args.value,
                tx_args.data.clone(),
                tx_args.operation,
                tx_args.safe_tx_gas,
                tx_args.base_gas,
                tx_args.gas_price,
                tx_args.gas_token,
                tx_args.refund_receiver,
                String::new(), // No signatures needed for signing
            );

            let hashes = handle_checks_for_signing(&tx_data, &tx_args, chain_id, tx_args.safe_version.clone());
            display_hashes(&hashes);

            let warnings = check_suspicious_content(&tx_data, Some(chain_id));
            display_warnings(&warnings);
        }
        Mode::Message(msg_args) => {
            let chain_id = ChainId::of(&msg_args.chain)
                .unwrap_or_else(|_| panic!("chain {:?} is supported but id is not found", msg_args.chain));
            let message = fs::read_to_string(&msg_args.input_file)
                .unwrap_or_else(|_| panic!("Failed to read message file: {}", msg_args.input_file));
            
            // TODO: Implement message signing logic here
            // This will require implementing a new function in sign_checks.rs
            // to handle message signing with the appropriate Safe message format
            println!("Message signing mode not yet implemented");
            println!("Message content: {}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_safe_hash_cli_output() {
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
        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        // Check for essential content without formatting
        // Domain hash
        assert!(stdout.contains("1655e94a9bcc5a957daa1acae692b4c22e7aaf146b4deb9194f8221d2f09d8c3"));
        // Message hash
        assert!(stdout.contains("f22754eba5a2b230714534b4657195268f00dc0031296de4b835d82e7aa1e574"));
        // Safe transaction hash
        assert!(stdout.contains("ad06b099fca34e51e4886643d95d9a19ace2cd024065efb66662a876e8c40343"));
    }
}

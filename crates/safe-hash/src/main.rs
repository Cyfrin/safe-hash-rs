mod cli;
mod etherscan;
mod output;
mod sign_checks;
mod tx_file;
mod warn;

use alloy::primitives::ChainId;
use clap::Parser;
use cli::{CliArgs, Mode};
use output::{display_hashes, display_warnings};
use safe_utils::{Of, SafeWalletVersion};
use sign_checks::*;
use tx_file::TxInput;
use warn::check_suspicious_content;
use std::fs;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();

    let chain_id = ChainId::of(&args.chain)
        .unwrap_or_else(|_| panic!("chain {:?} is supported but id is not found", args.chain));

    match args.mode {
        Mode::Transaction(tx_args) => {
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

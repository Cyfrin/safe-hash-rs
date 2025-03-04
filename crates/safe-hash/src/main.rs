mod cli;
mod etherscan;
mod output;
mod sign_checks;
mod tx_file;
mod warn;

use alloy::primitives::ChainId;
use clap::Parser;
use cli::CliArgs;
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

    if args.is_tx_mode() {
        let safe_version = args.safe_version.as_ref().unwrap().clone();
        let data = args.data.as_ref().unwrap().clone();
        
        let tx_data = TxInput::new(
            args.to.unwrap(),
            args.value.unwrap(),
            data,
            args.operation.unwrap(),
            args.safe_tx_gas.unwrap(),
            args.base_gas.unwrap(),
            args.gas_price.unwrap(),
            args.gas_token.unwrap(),
            args.refund_receiver.unwrap(),
            String::new(), // No signatures needed for signing
        );

        let hashes = handle_checks_for_signing(
            &tx_data,
            &args,
            chain_id,
            safe_version,
        );
        display_hashes(&hashes);

        let warnings = check_suspicious_content(&tx_data, Some(chain_id));
        display_warnings(&warnings);
    } else {
        let message = fs::read_to_string(&args.input_file.unwrap())
            .unwrap_or_else(|_| panic!("Failed to read message file"));
        
        // TODO: Implement message signing logic here
        // This will require implementing a new function in sign_checks.rs
        // to handle message signing with the appropriate Safe message format
        println!("Message signing mode not yet implemented");
        println!("Message content: {}", message);
    }
}

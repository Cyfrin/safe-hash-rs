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
use safe_utils::Of;
use sign_checks::*;
use tx_file::TxInput;
use warn::check_suspicious_content;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();

    let tx_data = TxInput::new(
        args.to,
        args.value,
        args.data.clone(),
        args.operation,
        args.safe_tx_gas,
        args.base_gas,
        args.gas_price,
        args.gas_token,
        args.refund_receiver,
        String::new(), // No signatures needed for signing
    );

    let chain_id = ChainId::of(&args.chain)
        .unwrap_or_else(|_| panic!("chain {:?} is supported but id is not found", args.chain));

    let hashes = handle_checks_for_signing(&tx_data, &args, chain_id, args.safe_version.clone());
    display_hashes(&hashes);

    let warnings = check_suspicious_content(&tx_data, Some(chain_id));
    display_warnings(&warnings);
}

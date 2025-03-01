mod cli;
mod exec_checks;
mod sign_checks;
mod tx_file;

use alloy::primitives::ChainId;
use clap::Parser;
use cli::CliArgs;
use exec_checks::*;
use safe_utils::Of;
use sign_checks::*;
use tx_file::TenderlyTxInput;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();
    args.validate_checks_asked();

    // Gather and validate input
    let chain_id = ChainId::of(&args.chain)
        .expect(&format!("chain {:?} is supported but id is not found", args.chain));

    let tx_data: TenderlyTxInput = {
        let tx_json = std::fs::read_to_string(&args.tx_file).expect("unable to read file");
        serde_json::from_str(&tx_json).expect("poorly formatted tx json")
    };

    // Analyze and show hashes
    if args.check_for_signing {
        handle_checks_for_signing(&tx_data, &args, chain_id, args.safe_version.clone());
    }
    if args.check_for_executing {
        handle_checks_for_executing(&tx_data);
    }

    // Suspicious content warning
    if tx_data.operation == 1 {
        println!();
        println!("WARNING: delegatecall found in operation!");
    }

    // TODO:
    // - Check if to contract is verified on Etherscan when data is not empty ?
}

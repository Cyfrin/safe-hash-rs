mod cli;
mod exec_checks;
mod message_checks;
mod sign_checks;
mod tx_file;

use alloy::primitives::ChainId;
use clap::Parser;
use cli::CliArgs;
use exec_checks::*;
use message_checks::*;
use safe_utils::Of;
use sign_checks::*;
use tx_file::TenderlyTxInput;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();
    args.validate_checks_asked();
    args.validate_nonce();
    args.validate_tx_file();
    args.validate_message_hash();

    // Gather and validate input
    let chain_id = ChainId::of(&args.chain)
        .expect(&format!("chain {:?} is supported but id is not found", args.chain));

    if args.check_for_signing || args.check_for_executing {
        let tx_data: TenderlyTxInput = {
            let tx_json =
                std::fs::read_to_string(&args.tx_file.clone().expect("tx_file not available"))
                    .expect("unable to read file");
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
    }

    if args.check_for_message_hash {
        handle_checks_for_message_hash(&args, chain_id, args.safe_version.clone());
    }

    // TODO:
    // - Check if to contract is verified on Etherscan when data is not empty ?
}

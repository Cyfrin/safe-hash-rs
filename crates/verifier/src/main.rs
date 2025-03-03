mod cli;
mod etherscan;
mod exec_checks;
mod message_checks;
mod sign_checks;
mod tx_file;
mod warn;

use alloy::primitives::ChainId;
use clap::Parser;
use cli::CliArgs;
use exec_checks::*;
use message_checks::*;
use safe_utils::Of;
use sign_checks::*;
use tx_file::TenderlyTxInput;
use warn::warn_suspicious_content;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();
    args.validate_checks_asked();
    args.validate_nonce();
    args.validate_tx_file();
    args.validate_message_hash();

    if args.check_for_signing || args.check_for_executing {
        let tx_data: TenderlyTxInput = {
            let tx_json =
                std::fs::read_to_string(&args.tx_file.clone().expect("tx_file not available"))
                    .expect("unable to read file");
            serde_json::from_str(&tx_json).expect("poorly formatted tx json")
        };

        if args.check_for_signing {
            let chain_id = ChainId::of(
                &args.chain.clone().expect("chain is not provided for checking the signing tx"),
            )
            .expect(&format!("chain {:?} is supported but id is not found", args.chain));

            handle_checks_for_signing(&tx_data, &args, chain_id, args.safe_version.clone());
            warn_suspicious_content(&tx_data, Some(chain_id));
        }
        if args.check_for_executing {
            handle_checks_for_executing(&tx_data);
            let chain_id =
                args.chain.clone().map(|c| ChainId::of(&c).expect("unsupported chain name"));
            warn_suspicious_content(&tx_data, chain_id);
        }
    }

    if args.check_for_message_hash {
        let chain_id = ChainId::of(
            &args.chain.clone().expect("chain is not provided for checking signing of message"),
        )
        .expect(&format!("chain {:?} is supported but id is not found", args.chain));

        handle_checks_for_message_hash(&args, chain_id, args.safe_version.clone());
    }
}

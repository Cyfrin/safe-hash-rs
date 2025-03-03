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
use message_checks::*;
use safe_utils::Of;
use sign_checks::*;
use tx_file::TxInput;
use warn::warn_suspicious_content;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();
    args.validate_message_hash();
    args.validate_transaction_params();

    if args.tx_signing {
        let tx_data: TxInput = if let Some(tx_file) = &args.tx_file {
            let tx_json = std::fs::read_to_string(tx_file)
                .unwrap_or_else(|_| panic!("unable to read file: {:?}", tx_file));
            serde_json::from_str(&tx_json)
                .unwrap_or_else(|_| panic!("poorly formatted tx json in file: {:?}", tx_file))
        } else {
            let to = args.to.unwrap();
            TxInput::new(
                to,
                args.value,
                args.data.clone(),
                args.operation,
                args.safe_tx_gas,
                args.base_gas,
                args.gas_price,
                args.gas_token,
                args.refund_receiver,
                String::new(), // No signatures needed for signing
            )
        };

        let chain_id = ChainId::of(&args.chain)
            .unwrap_or_else(|_| panic!("chain {:?} is supported but id is not found", args.chain));

        handle_checks_for_signing(&tx_data, &args, chain_id, args.safe_version.clone());
        warn_suspicious_content(&tx_data, Some(chain_id));
    }

    if args.msg_signing {
        let chain_id = ChainId::of(&args.chain)
            .unwrap_or_else(|_| panic!("chain {:?} is supported but id is not found", args.chain));

        handle_checks_for_message_hash(&args, chain_id, args.safe_version.clone());
    }
}

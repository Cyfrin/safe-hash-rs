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
use tx_file::TxInput;
use warn::warn_suspicious_content;

fn main() {
    let args = CliArgs::parse();
    args.validate_safe_version();
    args.validate_chain();
    args.validate_message_hash();
    args.validate_transaction_params();

    if args.tx_signing || args.tx_executing {
        let tx_data: TxInput = if let Some(tx_file) = &args.tx_file {
            let tx_json = std::fs::read_to_string(tx_file)
                .expect(&format!("unable to read file: {:?}", tx_file));
            serde_json::from_str(&tx_json)
                .expect(&format!("poorly formatted tx json in file: {:?}", tx_file))
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
                args.signatures.clone().unwrap_or_default(),
            )
        };

        if args.tx_signing {
            let chain_id = ChainId::of(&args.chain)
                .expect(&format!("chain {:?} is supported but id is not found", args.chain));

            handle_checks_for_signing(&tx_data, &args, chain_id, args.safe_version.clone());
            warn_suspicious_content(&tx_data, Some(chain_id));
        }
        if args.tx_executing {
            handle_checks_for_executing(&tx_data);
            let chain_id = ChainId::of(&args.chain).ok();
            warn_suspicious_content(&tx_data, chain_id);
        }
    }

    if args.msg_signing {
        let chain_id = ChainId::of(&args.chain)
            .expect(&format!("chain {:?} is supported but id is not found", args.chain));

        handle_checks_for_message_hash(&args, chain_id, args.safe_version.clone());
    }
}

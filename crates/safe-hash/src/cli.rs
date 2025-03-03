use alloy::primitives::{Address, U256};
use clap::Parser;
use safe_utils::{SafeWalletVersion, get_all_supported_chain_names};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Chain
    /// - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea,
    /// mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia,
    /// gnosis-chiado, polygon-zkevm
    #[arg(short, long, required = true)]
    pub chain: String,

    /// Transaction nonce of the safe contract
    #[arg(short, long, required = true)]
    pub nonce: u8,

    /// Address of the safe contract
    #[arg(short = 's', long = "safe-address", required = true)]
    pub safe_address: Address,

    /// Safe Contract version
    #[arg(short = 'u', long, default_value = "1.3.0")]
    pub safe_version: SafeWalletVersion,

    /// Path to JSON file containing all the transaction data
    /// If provided, this will override any manually provided transaction parameters
    #[arg(
        short,
        long,
        conflicts_with_all=["to", "value", "data", "operation", "safe_tx_gas", "base_gas", "gas_price", "gas_token", "refund_receiver"]
    )]
    pub tx_file: Option<PathBuf>,

    /// Address of the contract to which the safe-contract sends calldata to.
    #[arg(long, required_unless_present = "tx_file")]
    pub to: Option<Address>,

    /// Value asked in the transaction (relates to eth)
    #[arg(long, default_value_t = U256::ZERO)]
    pub value: U256,

    /// Raw calldata encoded in hex
    #[arg(long, default_value = "0x")]
    pub data: String,

    /// Call or delegate call (0 or 1)
    #[arg(long, default_value_t = 0)]
    pub operation: u8,

    #[arg(long, default_value_t = U256::ZERO)]
    pub safe_tx_gas: U256,

    #[arg(long, default_value_t = U256::ZERO)]
    pub base_gas: U256,

    #[arg(long, default_value_t = U256::ZERO)]
    pub gas_price: U256,

    #[arg(long, default_value_t = Address::ZERO)]
    pub gas_token: Address,

    #[arg(long, default_value_t = Address::ZERO)]
    pub refund_receiver: Address,

    /// Required when checking for execution
    #[arg(long)]
    pub signatures: Option<String>,

    /// Path to message file containing message in plain text
    #[arg(short, long)]
    pub message_file: Option<PathBuf>,

    /// Check for signing the transaction
    #[arg(short = 'k', long, group = "check_for")]
    pub check_for_signing: bool,

    /// Check for executing the transaction
    #[arg(short = 'x', long, group = "check_for")]
    pub check_for_executing: bool,

    /// Check message hashes off-chain
    #[arg(short = 'o', long, group = "check_for")]
    pub check_for_message_hash: bool,
}

impl CliArgs {
    pub fn validate_chain(&self) {
        let valid_names = get_all_supported_chain_names();
        if !valid_names.contains(&self.chain) {
            eprintln!("chain {:?} is not supported", self.chain);
            std::process::exit(1);
        }
    }

    pub fn validate_checks_asked(&self) {
        if !self.check_for_signing && !self.check_for_executing && !self.check_for_message_hash {
            eprintln!(
                "please use one of --check-for-signing or --check-for-executing or --check-for-message-hash"
            );
            std::process::exit(1);
        }
    }

    pub fn validate_safe_version(&self) {
        if self.safe_version < SafeWalletVersion::new(0, 1, 0) {
            eprintln!("{} version of Safe Wallet is not supported", self.safe_version);
            std::process::exit(1);
        }
    }

    pub fn validate_message_hash(&self) {
        if self.check_for_message_hash && self.message_file.is_none() {
            eprintln!("message file must be specified when checking for message hash");
            std::process::exit(1);
        }
    }

    pub fn validate_transaction_params(&self) {
        if (self.check_for_signing || self.check_for_executing) && self.tx_file.is_none() && self.to.is_none() {
            eprintln!("Either tx-file or 'to' address must be specified when checking for signing or executing");
            std::process::exit(1);
        }
    }
}

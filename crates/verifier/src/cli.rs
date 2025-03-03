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
    #[arg(short, long)]
    pub chain: Option<String>,

    /// Transaction nonce of the safe contract
    #[arg(short, long)]
    pub nonce: Option<u8>,

    /// Address of the safe contract
    #[arg(short, long)]
    pub safe_contract: Option<Address>,

    /// Safe Contract version
    #[arg(short = 'u', long, default_value = "1.3.0")]
    pub safe_version: SafeWalletVersion,

    /// Path to JSON file containing all the transaction data [clean way] 
    /// Most suitable if copying the input from Tenderly.
    ///
    /// The alternative, is to pass the tx details yourself as individual command line arguments.
    /// This includes `to` and optionally `value`, `data`, `operation`, `safe-tx-gas`, `base-gas`, `gas-price`,
    /// `gas-token`, `refund-receiver`, `signatures`
    #[arg(
        short,
        long, 
        conflicts_with_all=["to", "value", "data", "operation", "safe_tx_gas", "base_gas", "gas_price", "gas_token", "refund_receiver"]
    )]
    pub tx_file: Option<PathBuf>,

    /// Address of the contract to which the safe-contract sends calldata to.
    #[arg(long)]
    pub to: Option<Address>,

    /// Value asked in the transaction (relates to eth)
    #[arg(long, default_value_t = U256::ZERO, requires="to")]
    pub value: U256,

    /// Raw calldata encoded in hex
    #[arg(long, default_value = "0x", requires="to")]
    pub data: String,

    /// Call or delegate call (0 or 1)
    #[arg(long, default_value_t = 0, requires="to")]
    pub operation: u8,

    #[arg(long, default_value_t = U256::ZERO, requires="to")]
    pub safe_tx_gas: U256,

    #[arg(long, default_value_t = U256::ZERO, requires="to")]
    pub base_gas: U256,

    #[arg(long, default_value_t = U256::ZERO, requires="to")]
    pub gas_price: U256,

    #[arg(long, default_value_t = Address::ZERO, requires="to")]
    pub gas_token: Address,

    #[arg(long, default_value_t = Address::ZERO, requires="to")]
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
        if let Some(chain) = &self.chain {
            if !valid_names.contains(chain) {
                eprintln!("chain {:?} is not supported", self.chain);
                std::process::exit(1);
            }
        } else if self.check_for_signing || self.check_for_message_hash {
            eprintln!("chain needs to be specified when checking for signing tx or message hash");
            std::process::exit(1);
        }
    }
    pub fn validate_safe_contract(&self) {
        if self.safe_contract.is_none() && (self.check_for_signing || self.check_for_message_hash) {
            eprintln!(
                "safe-contract needs to specified when checking for signing tx or message hash"
            );
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
    pub fn validate_nonce(&self) {
        if self.check_for_signing && self.nonce.is_none() {
            eprintln!("nonce needs to be specified when checking for signing");
            std::process::exit(1);
        }
    }
    pub fn validate_tx_file(&self) {
        if (self.check_for_executing || self.check_for_signing) && self.tx_file.is_none() {
            eprintln!("txfile needs to be specified when checking for executing");
            std::process::exit(1);
        }
    }
    pub fn validate_message_hash(&self) {
        if self.check_for_message_hash && self.message_file.is_none() {
            eprintln!("message file must be specified when checking for message hash");
            std::process::exit(1);
        }
    }
}

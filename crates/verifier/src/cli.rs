use std::path::PathBuf;

use alloy::primitives::Address;
use clap::Parser;
use safe_utils::{SafeWalletVersion, get_all_supported_chain_names};

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

    /// Path to JSON file containing the input from Tenderly's simulation summary
    #[arg(short, long)]
    pub tx_file: Option<PathBuf>,

    /// Path to message file for offchain message hashes
    #[arg(short, long)]
    pub message_file: Option<PathBuf>,

    /// Safe Contract version
    #[arg(long, default_value = "1.3.0")]
    pub safe_version: SafeWalletVersion,

    /// Check for signing the transaction
    #[arg(long)]
    pub check_for_signing: bool,

    /// Check for executing the transaction
    #[arg(long)]
    pub check_for_executing: bool,

    /// Check message hashes off-chain
    #[arg(long)]
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
        }
    }
    pub fn validate_checks_asked(&self) {
        if !self.check_for_signing && !self.check_for_executing && !self.check_for_message_hash {
            eprintln!(
                "please use at least one of --check-for-signing or --check-for-executing or --check-for-message-hash"
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

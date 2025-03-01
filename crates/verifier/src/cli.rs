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
    pub chain: String,

    /// Transaction nonce of the safe contract
    #[arg(short, long)]
    pub nonce: u8,

    /// Address of the safe contract
    #[arg(short, long)]
    pub safe_contract: Address,

    /// Path to JSON file containing the input from Tenderly's simulation summary
    #[arg(short, long)]
    pub tx_file: PathBuf,

    /// Safe Contract version
    #[arg(long, default_value = "1.3.0")]
    pub safe_version: SafeWalletVersion,

    /// Check for signing the transaction
    #[arg(long)]
    pub check_for_signing: bool,

    /// Check for executing the transaction
    #[arg(long)]
    pub check_for_executing: bool,
}

impl CliArgs {
    pub fn validate_chain(&self) {
        let valid_names = get_all_supported_chain_names();
        if !valid_names.contains(&self.chain) {
            eprintln!("Chain {:?} is not supported!", self.chain);
            std::process::exit(1);
        }
    }
    pub fn validate_checks_asked(&self) {
        if !self.check_for_signing && !self.check_for_executing {
            eprintln!("Please use at least one of --check-for-signing or --check-for-executing");
            std::process::exit(1);
        }
    }
    pub fn validate_safe_version(&self) {
        if self.safe_version < SafeWalletVersion::new(0, 1, 0) {
            eprintln!("{} version of Safe Wallet is not supported", self.safe_version);
            std::process::exit(1);
        }
    }
}

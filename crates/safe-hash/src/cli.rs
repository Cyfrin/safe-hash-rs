use std::path::PathBuf;

use alloy::primitives::{Address, U256};
use clap::{Parser, Subcommand};
use safe_utils::{SafeWalletVersion, get_all_supported_chain_names};
use semver::Version;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Transaction signing mode
    #[command(name = "tx")]
    Transaction(TransactionArgs),

    /// Message signing mode
    #[command(name = "msg")]
    Message(MessageArgs),

    /// Encode EIP 712 typed message
    #[command(name = "typed")]
    Eip712(Eip712Args),
}

#[derive(Parser, Debug)]
pub struct TransactionArgs {
    /// Chain
    /// - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea,
    /// mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia,
    /// gnosis-chiado, polygon-zkevm
    #[arg(short, long, required = true)]
    pub chain: String,

    /// Transaction nonce of the safe address
    #[arg(short, long, required = true)]
    pub nonce: u64,

    /// Address of the safe address
    #[arg(short = 's', long = "safe-address", required = true)]
    pub safe_address: Address,

    /// Safe Contract version
    #[arg(short = 'u', long)]
    pub safe_version: SafeWalletVersion,

    /// Address of the contract to which the safe-address sends calldata to.
    #[arg(short, long)]
    pub to: Option<Address>,

    /// Value asked in the transaction (relates to eth)
    #[arg(long, default_value_t = U256::ZERO)]
    pub value: U256,

    /// Raw calldata encoded in hex
    #[arg(short, long, default_value = "0x")]
    pub data: String,

    /// Call or delegate call (0 or 1)
    #[arg(long, default_value_t = 0)]
    pub operation: u8,

    /// Nested safe address
    #[arg(long)]
    pub nested_safe_address: Option<Address>,

    /// Nested safe nonce
    #[arg(long)]
    pub nested_safe_nonce: Option<u64>,

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

    #[arg(long)]
    pub offline: bool,
}

impl Default for TransactionArgs {
    fn default() -> Self {
        Self {
            to: None,
            value: U256::ZERO,
            data: "0x".to_string(),
            operation: 0,
            gas_token: Address::ZERO,
            safe_tx_gas: U256::ZERO,
            base_gas: U256::ZERO,
            gas_price: U256::ZERO,
            refund_receiver: Address::ZERO,
            nonce: 0,
            safe_address: Address::ZERO,
            chain: "ethereum".to_string(),
            safe_version: Version::new(1, 3, 0),
            nested_safe_address: None,
            nested_safe_nonce: None,
            offline: false,
        }
    }
}

#[derive(Parser, Debug)]
pub struct MessageArgs {
    /// Chain
    /// - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea,
    /// mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia,
    /// gnosis-chiado, polygon-zkevm
    #[arg(short, long, required = true)]
    pub chain: String,

    /// Address of the safe address
    #[arg(short = 's', long = "safe-address", required = true)]
    pub safe_address: Address,

    /// Safe Contract version
    #[arg(short = 'u', long)]
    pub safe_version: SafeWalletVersion,

    /// Path to the message file to be signed
    #[arg(short, long, required = true)]
    pub input_file: String,
}

#[derive(Parser, Debug)]
pub struct Eip712Args {
    /// Chain
    /// - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea,
    /// mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia,
    /// gnosis-chiado, polygon-zkevm
    #[arg(short, long)]
    pub chain: Option<String>,

    /// Address of the safe address
    #[arg(short = 's', long = "safe-address")]
    pub safe_address: Option<Address>,

    /// Safe Contract version
    #[arg(short = 'u', long)]
    pub safe_version: Option<SafeWalletVersion>,

    #[arg(long)]
    pub standalone: bool,

    /// File contiaing the JSON formatted EIP 712 spec
    #[arg(short, long, required = true)]
    pub file: PathBuf,
}

impl CliArgs {
    pub fn validate_eip712_args(&self) {
        if let Mode::Eip712(Eip712Args { chain, safe_address, safe_version, standalone, .. }) =
            &self.mode
        {
            if *standalone {
                return;
            }
            if chain.is_none() || safe_address.is_none() || safe_version.is_none() {
                eprintln!("include `--standalone` flag");
                std::process::exit(1);
            }
        }
    }

    pub fn validate_safe_version(&self) {
        if let Mode::Transaction(tx_args) = &self.mode {
            if tx_args.safe_version < SafeWalletVersion::new(0, 1, 0) {
                eprintln!("{} version of Safe Wallet is not supported", tx_args.safe_version);
                std::process::exit(1);
            }
        }
    }

    pub fn validate_chain(&self) {
        if let Mode::Transaction(tx_args) = &self.mode {
            let valid_names = get_all_supported_chain_names();
            if !valid_names.contains(&tx_args.chain) {
                eprintln!("chain {:?} is not supported", tx_args.chain);
                std::process::exit(1);
            }
        } else if let Mode::Message(msg_args) = &self.mode {
            let valid_names = get_all_supported_chain_names();
            if !valid_names.contains(&msg_args.chain) {
                eprintln!("chain {:?} is not supported", msg_args.chain);
                std::process::exit(1);
            }
        }
    }

    pub fn validate_to_for_offline(&self) {
        if let Mode::Transaction(tx_args) = &self.mode {
            if tx_args.to.is_none() && tx_args.offline {
                eprintln!(
                    "--to <address> must be provided in offline mode. When API data cannot be fetched, you must provide the destination address manually."
                );
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;
    use clap::Parser;

    fn base_args() -> Vec<String> {
        vec![
            "safe-hash".to_string(),
            "tx".to_string(),
            "--chain".to_string(),
            "ethereum".to_string(),
            "--nonce".to_string(),
            "42".to_string(),
            "--safe-address".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
        ]
    }

    fn manual_args() -> Vec<String> {
        let mut args = base_args();
        args.extend_from_slice(&[
            "--to".to_string(),
            "0x2234567890123456789012345678901234567890".to_string(),
            "--data".to_string(),
            "0xabcd".to_string(),
        ]);
        args
    }

    #[test]
    fn test_all_gas_params() {
        let mut args = manual_args();
        args.extend_from_slice(&[
            "--safe-tx-gas".to_string(),
            "100000".to_string(),
            "--base-gas".to_string(),
            "21000".to_string(),
            "--gas-price".to_string(),
            "50000000000".to_string(),
            "--gas-token".to_string(),
            "0x3234567890123456789012345678901234567890".to_string(),
            "--refund-receiver".to_string(),
            "0x4234567890123456789012345678901234567890".to_string(),
            "--safe-version".to_string(),
            "1.3.0".to_string(),
        ]);

        let cli = CliArgs::try_parse_from(&args).unwrap();
        if let Mode::Transaction(tx_args) = cli.mode {
            assert_eq!(tx_args.safe_tx_gas, U256::from(100000));
            assert_eq!(tx_args.base_gas, U256::from(21000));
            assert_eq!(tx_args.gas_price, U256::from(50000000000u64));
            assert_eq!(tx_args.gas_token, address!("0x3234567890123456789012345678901234567890"));
            assert_eq!(
                tx_args.refund_receiver,
                address!("0x4234567890123456789012345678901234567890")
            );
        } else {
            panic!("Expected Transaction mode");
        }
    }

    #[test]
    fn test_message_mode() {
        let args = vec![
            "safe-hash".to_string(),
            "msg".to_string(),
            "--chain".to_string(),
            "ethereum".to_string(),
            "--safe-address".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
            "--input-file".to_string(),
            "message.txt".to_string(),
            "--safe-version".to_string(),
            "1.3.0".to_string(),
        ];

        let cli = CliArgs::try_parse_from(&args).unwrap();
        if let Mode::Message(msg_args) = cli.mode {
            assert_eq!(msg_args.chain, "ethereum");
            assert_eq!(msg_args.input_file, "message.txt");
            assert_eq!(
                msg_args.safe_address,
                address!("0x1234567890123456789012345678901234567890")
            );
        } else {
            panic!("Expected Message mode");
        }
    }
}

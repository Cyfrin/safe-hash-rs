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

    /// Transaction nonce of the safe address
    #[arg(short, long, required = true)]
    pub nonce: u8,

    /// Address of the safe address
    #[arg(short = 's', long = "safe-address", required = true)]
    pub safe_address: Address,

    /// Safe Contract version
    #[arg(short = 'u', long, default_value = "1.3.0")]
    pub safe_version: SafeWalletVersion,

    /// Address of the contract to which the safe-address sends calldata to.
    #[arg(short, long, required_unless_present = "tx_file")]
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

    /// Path to JSON file containing all the transaction data
    /// If provided, this will override any manually provided transaction parameters
    #[arg(
        long,
        conflicts_with_all=["to", "value", "data", "operation", "safe_tx_gas", "base_gas", "gas_price", "gas_token", "refund_receiver"]
    )]
    pub tx_file: Option<PathBuf>,

    /// Path to message file containing message in plain text
    #[arg(short, long)]
    pub message_file: Option<PathBuf>,

    /// Check transaction signing (default if no mode specified)
    #[arg(long = "tx-signing", group = "mode", default_value_t = true)]
    pub tx_signing: bool,

    /// Check message signing
    #[arg(long = "msg-signing", group = "mode")]
    pub msg_signing: bool,
}

impl CliArgs {
    pub fn validate_chain(&self) {
        let valid_names = get_all_supported_chain_names();
        if !valid_names.contains(&self.chain) {
            eprintln!("chain {:?} is not supported", self.chain);
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
        if self.msg_signing && self.message_file.is_none() {
            eprintln!("message file must be specified when checking for message signing");
            std::process::exit(1);
        }
    }

    pub fn validate_transaction_params(&self) {
        if self.tx_signing && self.tx_file.is_none() && self.to.is_none() {
            eprintln!("Either tx-file or 'to' address must be specified when checking for signing");
            std::process::exit(1);
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
            "--chain".to_string(),
            "ethereum".to_string(),
            "--nonce".to_string(),
            "42".to_string(),
            "--safe-address".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
        ]
    }

    fn tx_file_args() -> Vec<String> {
        let mut args = base_args();
        args.extend_from_slice(&["--tx-file".to_string(), "tx.json".to_string()]);
        args
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
    fn test_tx_signing_with_manual_params() {
        let mut args = manual_args();

        let cli = CliArgs::try_parse_from(&args).unwrap();
        assert!(cli.tx_signing);
        assert!(!cli.msg_signing);
        assert_eq!(cli.chain, "ethereum");
        assert_eq!(cli.nonce, 42);
        assert_eq!(cli.safe_address, address!("0x1234567890123456789012345678901234567890"));
        assert_eq!(cli.to.unwrap(), address!("0x2234567890123456789012345678901234567890"));
        assert_eq!(cli.value, U256::from(0));
        assert_eq!(cli.data, "0xabcd");
    }

    #[test]
    fn test_tx_signing_with_tx_file() {
        let mut args = tx_file_args();

        let cli = CliArgs::try_parse_from(&args).unwrap();
        assert!(cli.tx_signing);
        assert!(!cli.msg_signing);
        assert!(cli.tx_file.is_some());
        assert_eq!(cli.tx_file.unwrap().to_str().unwrap(), "tx.json");
    }

    // #[test]
    // fn test_msg_signing() {
    //     let mut args = base_args();
    //     args.extend_from_slice(&[
    //         "--msg-signing".to_string(),
    //         "--message-file".to_string(),
    //         "message.txt".to_string(),
    //     ]);

    //     let cli = CliArgs::try_parse_from(&args).unwrap();
    //     assert!(!cli.tx_signing);
    //     assert!(cli.msg_signing);
    //     assert!(cli.message_file.is_some());
    //     assert_eq!(cli.message_file.unwrap().to_str().unwrap(), "message.txt");
    // }

    #[test]
    fn test_tx_file_conflicts_with_manual_params() {
        let mut args = base_args();
        args.extend_from_slice(&[
            "--tx-file".to_string(),
            "tx.json".to_string(),
            "--to".to_string(),
            "0x2234567890123456789012345678901234567890".to_string(),
        ]);

        let result = CliArgs::try_parse_from(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_modes_are_mutually_exclusive() {
        let mut args = base_args();
        args.extend_from_slice(&["--tx-signing".to_string(), "--msg-signing".to_string()]);

        let result = CliArgs::try_parse_from(&args);
        assert!(result.is_err());
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
        ]);

        let cli = CliArgs::try_parse_from(&args).unwrap();
        assert_eq!(cli.safe_tx_gas, U256::from(100000));
        assert_eq!(cli.base_gas, U256::from(21000));
        assert_eq!(cli.gas_price, U256::from(50000000000u64));
        assert_eq!(cli.gas_token, address!("0x3234567890123456789012345678901234567890"));
        assert_eq!(cli.refund_receiver, address!("0x4234567890123456789012345678901234567890"));
    }
}

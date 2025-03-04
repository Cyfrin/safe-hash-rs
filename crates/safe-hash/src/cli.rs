use alloy::primitives::{Address, U256};
use clap::{Parser, Subcommand};
use safe_utils::{SafeWalletVersion, get_all_supported_chain_names};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Chain
    /// - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea,
    /// mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia,
    /// gnosis-chiado, polygon-zkevm
    #[arg(short, long, required = true)]
    pub chain: String,

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
}

#[derive(Parser, Debug)]
pub struct TransactionArgs {
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
    #[arg(short, long, required = true)]
    pub to: Address,

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
}

#[derive(Parser, Debug)]
pub struct MessageArgs {
    /// Path to the message file to be signed
    #[arg(short, long, required = true)]
    pub input_file: String,
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
        if let Mode::Transaction(tx_args) = &self.mode {
            if tx_args.safe_version < SafeWalletVersion::new(0, 1, 0) {
                eprintln!("{} version of Safe Wallet is not supported", tx_args.safe_version);
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
            "--chain".to_string(),
            "ethereum".to_string(),
            "tx".to_string(),
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
    fn test_manual_params() {
        let args = manual_args();

        let cli = CliArgs::try_parse_from(&args).unwrap();
        assert_eq!(cli.chain, "ethereum");
        if let Mode::Transaction(tx_args) = cli.mode {
            assert_eq!(tx_args.nonce, 42);
            assert_eq!(tx_args.safe_address, address!("0x1234567890123456789012345678901234567890"));
            assert_eq!(tx_args.to, address!("0x2234567890123456789012345678901234567890"));
            assert_eq!(tx_args.value, U256::from(0));
            assert_eq!(tx_args.data, "0xabcd");
        } else {
            panic!("Expected Transaction mode");
        }
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
        if let Mode::Transaction(tx_args) = cli.mode {
            assert_eq!(tx_args.safe_tx_gas, U256::from(100000));
            assert_eq!(tx_args.base_gas, U256::from(21000));
            assert_eq!(tx_args.gas_price, U256::from(50000000000u64));
            assert_eq!(tx_args.gas_token, address!("0x3234567890123456789012345678901234567890"));
            assert_eq!(tx_args.refund_receiver, address!("0x4234567890123456789012345678901234567890"));
        } else {
            panic!("Expected Transaction mode");
        }
    }

    #[test]
    fn test_message_mode() {
        let args = vec![
            "safe-hash".to_string(),
            "--chain".to_string(),
            "ethereum".to_string(),
            "msg".to_string(),
            "--input-file".to_string(),
            "message.txt".to_string(),
        ];

        let cli = CliArgs::try_parse_from(&args).unwrap();
        assert_eq!(cli.chain, "ethereum");
        if let Mode::Message(msg_args) = cli.mode {
            assert_eq!(msg_args.input_file, "message.txt");
        } else {
            panic!("Expected Message mode");
        }
    }
}

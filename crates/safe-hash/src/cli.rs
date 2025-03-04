use alloy::primitives::{Address, U256};
use clap::Parser;
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

    /// Transaction signing mode (default)
    #[arg(long, conflicts_with = "msg")]
    pub tx: bool,

    /// Message signing mode
    #[arg(long, conflicts_with = "tx")]
    pub msg: bool,

    /// Transaction nonce of the safe address (required for tx mode)
    #[arg(short, long, required_if_eq("tx", "true"))]
    pub nonce: Option<u8>,

    /// Address of the safe address (required for tx mode)
    #[arg(short = 's', long = "safe-address", required_if_eq("tx", "true"))]
    pub safe_address: Option<Address>,

    /// Safe Contract version (required for tx mode)
    #[arg(short = 'u', long, default_value = "1.3.0", required_if_eq("tx", "true"))]
    pub safe_version: Option<SafeWalletVersion>,

    /// Address of the contract to which the safe-address sends calldata to (required for tx mode)
    #[arg(short, long, required_if_eq("tx", "true"))]
    pub to: Option<Address>,

    /// Value asked in the transaction (relates to eth)
    #[arg(long, default_value = "0", required_if_eq("tx", "true"))]
    pub value: Option<U256>,

    /// Raw calldata encoded in hex
    #[arg(short, long, default_value = "0x", required_if_eq("tx", "true"))]
    pub data: Option<String>,

    /// Call or delegate call (0 or 1)
    #[arg(long, default_value = "0", required_if_eq("tx", "true"))]
    pub operation: Option<u8>,

    #[arg(long, default_value = "0", required_if_eq("tx", "true"))]
    pub safe_tx_gas: Option<U256>,

    #[arg(long, default_value = "0", required_if_eq("tx", "true"))]
    pub base_gas: Option<U256>,

    #[arg(long, default_value = "0", required_if_eq("tx", "true"))]
    pub gas_price: Option<U256>,

    #[arg(long, default_value = "0x0000000000000000000000000000000000000000", required_if_eq("tx", "true"))]
    pub gas_token: Option<Address>,

    #[arg(long, default_value = "0x0000000000000000000000000000000000000000", required_if_eq("tx", "true"))]
    pub refund_receiver: Option<Address>,

    /// Path to the message file to be signed (required for msg mode)
    #[arg(short, long, required_if_eq("msg", "true"))]
    pub input_file: Option<String>,
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
        if self.tx {
            if let Some(safe_version) = &self.safe_version {
                if *safe_version < SafeWalletVersion::new(0, 1, 0) {
                    eprintln!("{} version of Safe Wallet is not supported", safe_version);
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn is_tx_mode(&self) -> bool {
        self.tx || (!self.tx && !self.msg) // Default to tx mode if neither flag is set
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
        assert!(cli.is_tx_mode());
        assert_eq!(cli.nonce.unwrap(), 42);
        assert_eq!(cli.safe_address.unwrap(), address!("0x1234567890123456789012345678901234567890"));
        assert_eq!(cli.to.unwrap(), address!("0x2234567890123456789012345678901234567890"));
        assert_eq!(cli.value.unwrap(), U256::from(0));
        assert_eq!(cli.data.unwrap(), "0xabcd");
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
        assert!(cli.is_tx_mode());
        assert_eq!(cli.safe_tx_gas.unwrap(), U256::from(100000));
        assert_eq!(cli.base_gas.unwrap(), U256::from(21000));
        assert_eq!(cli.gas_price.unwrap(), U256::from(50000000000u64));
        assert_eq!(cli.gas_token.unwrap(), address!("0x3234567890123456789012345678901234567890"));
        assert_eq!(cli.refund_receiver.unwrap(), address!("0x4234567890123456789012345678901234567890"));
    }

    #[test]
    fn test_message_mode() {
        let args = vec![
            "safe-hash".to_string(),
            "--chain".to_string(),
            "ethereum".to_string(),
            "--msg".to_string(),
            "--input-file".to_string(),
            "message.txt".to_string(),
        ];

        let cli = CliArgs::try_parse_from(&args).unwrap();
        assert_eq!(cli.chain, "ethereum");
        assert!(!cli.is_tx_mode());
        assert_eq!(cli.input_file.unwrap(), "message.txt");
    }

    #[test]
    fn test_default_mode() {
        let args = vec![
            "safe-hash".to_string(),
            "--chain".to_string(),
            "ethereum".to_string(),
        ];

        let cli = CliArgs::try_parse_from(&args).unwrap();
        assert!(cli.is_tx_mode());
    }
}

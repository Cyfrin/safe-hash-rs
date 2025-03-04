use crate::{cli::TransactionArgs, output::SafeHashes, tx_file::TxInput};
use alloy::primitives::{ChainId, U256};
use safe_utils::{CallDataHasher, DomainHasher, SafeHasher, SafeWalletVersion, TxMessageHasher};

pub fn handle_checks_for_signing(
    tx_data: &TxInput,
    args: &TransactionArgs,
    chain_id: ChainId,
    safe_version: SafeWalletVersion,
) -> SafeHashes {
    // Calculate hashes
    let domain_hash = {
        let domain_hasher = DomainHasher::new(safe_version.clone(), chain_id, args.safe_address);
        domain_hasher.hash()
    };

    let message_hash = {
        let calldata_hash = {
            let calldata_hasher = CallDataHasher::new(tx_data.data.clone());
            calldata_hasher.hash().unwrap_or_else(|_| panic!("unable to hash {:?}", tx_data.data))
        };
        let message_hasher = TxMessageHasher::new(
            safe_version,
            tx_data.to,
            tx_data.value,
            calldata_hash,
            tx_data.operation,
            tx_data.safe_tx_gas,
            tx_data.base_gas(),
            tx_data.gas_price,
            tx_data.gas_token,
            tx_data.refund_receiver,
            U256::from(args.nonce),
        );
        message_hasher.hash()
    };

    let safe_hash = {
        let safe_hasher = SafeHasher::new(domain_hash, message_hash);
        safe_hasher.hash()
    };

    SafeHashes { domain_hash, message_hash, safe_tx_hash: safe_hash }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, ChainId, FixedBytes, U256, hex};
    use safe_utils::{Of, SafeWalletVersion};
    use std::str::FromStr;

    #[test]
    fn test_handle_checks_for_signing() {
        // Create test inputs
        let safe_address = Address::from_str("0x1c694Fc3006D81ff4a56F97E1b99529066a23725").unwrap();
        let to_address = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
        let nonce = 63;
        let data = "0xa9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00".to_string();

        let args = TransactionArgs {
            chain: "ethereum".to_string(),
            nonce,
            safe_address,
            safe_version: SafeWalletVersion::new(1, 3, 0),
            to: to_address,
            value: U256::ZERO,
            data: data.clone(),
            operation: 0,
            safe_tx_gas: U256::ZERO,
            base_gas: U256::ZERO,
            gas_price: U256::ZERO,
            gas_token: Address::ZERO,
            refund_receiver: Address::ZERO,
        };

        let tx_data = TxInput::new(
            to_address,
            U256::ZERO,
            data,
            0,
            U256::ZERO,
            U256::ZERO,
            U256::ZERO,
            Address::ZERO,
            Address::ZERO,
            String::new(),
        );

        let chain_id = ChainId::of("ethereum").unwrap();
        let hashes =
            handle_checks_for_signing(&tx_data, &args, chain_id, args.safe_version.clone());

        // Expected outputs
        let expected_domain = FixedBytes::new(
            hex::decode("1655e94a9bcc5a957daa1acae692b4c22e7aaf146b4deb9194f8221d2f09d8c3")
                .unwrap()
                .try_into()
                .unwrap(),
        );
        let expected_message = FixedBytes::new(
            hex::decode("f22754eba5a2b230714534b4657195268f00dc0031296de4b835d82e7aa1e574")
                .unwrap()
                .try_into()
                .unwrap(),
        );
        let expected_safe = FixedBytes::new(
            hex::decode("ad06b099fca34e51e4886643d95d9a19ace2cd024065efb66662a876e8c40343")
                .unwrap()
                .try_into()
                .unwrap(),
        );

        // Assert outputs match expected values
        assert_eq!(hashes.domain_hash, expected_domain, "Domain hash mismatch");
        assert_eq!(hashes.message_hash, expected_message, "Message hash mismatch");
        assert_eq!(hashes.safe_tx_hash, expected_safe, "Safe transaction hash mismatch");
    }
}

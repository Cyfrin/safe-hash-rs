use crate::{cli::TransactionArgs, output::SafeHashes};
use alloy::primitives::{Address, ChainId, U256};
use safe_utils::{CallDataHasher, DomainHasher, SafeHasher, SafeWalletVersion, TxMessageHasher};
use serde::{Deserialize, Serialize};

// Upon simulation (in Tenderly), the Summary section contains input and output json.
// This module will try to deserialize the input json to have a strongly typed representation.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxInput {
    base_gas: Option<U256>,
    data_gas: Option<U256>,
    pub data: String,
    pub gas_price: U256,
    pub gas_token: Address,
    pub operation: u8,
    pub refund_receiver: Address,
    pub safe_tx_gas: U256,
    pub to: Address,
    pub value: U256,
    pub signatures: String,
}

impl TxInput {
    pub fn new(
        to: Address,
        value: U256,
        data: String,
        operation: u8,
        safe_tx_gas: U256,
        base_gas: U256,
        gas_price: U256,
        gas_token: Address,
        refund_receiver: Address,
        signatures: String,
    ) -> Self {
        Self {
            to,
            value,
            data,
            operation,
            safe_tx_gas,
            base_gas: Some(base_gas),
            data_gas: None,
            gas_price,
            gas_token,
            refund_receiver,
            signatures,
        }
    }

    pub fn base_gas(&self) -> U256 {
        if let Some(base_gas) = self.base_gas {
            return base_gas;
        }
        self.data_gas.expect("neither data_gas not base_gas was found")
    }
}

pub fn tx_signing_hashes(
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

    let safe_tx_hash = {
        let safe_hasher = SafeHasher::new(domain_hash, message_hash);
        safe_hasher.hash()
    };

    SafeHashes { domain_hash, message_hash, safe_tx_hash, raw_message_hash: None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, ChainId, FixedBytes, U256, address, hex};
    use safe_utils::{Of, SafeWalletVersion};
    use std::str::FromStr;

    #[test]
    fn test_tx_signing_hashes() {
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
            to: Some(to_address),
            value: U256::ZERO,
            data: data.clone(),
            operation: 0,
            safe_tx_gas: U256::ZERO,
            base_gas: U256::ZERO,
            gas_price: U256::ZERO,
            gas_token: Address::ZERO,
            refund_receiver: Address::ZERO,
            offline: false,
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
        let hashes = tx_signing_hashes(&tx_data, &args, chain_id, args.safe_version.clone());

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

    #[test]
    fn test_sample_tx_file_is_deserializable() {
        let input = r#"
            {
              "to": "0x1f28d065e77c8cb223bbb7c5edb3a432268e5811",
              "value": "0",
              "data": "0xa9059cbb000000000000000000000000fa5aa11148a2181b1b6e4e290709b665fd1781f500000000000000000000000000000000000000000000000000000000001e8480",
              "operation": 0,
              "safeTxGas": "0",
              "baseGas": "0",
              "gasPrice": "0",
              "gasToken": "0x0000000000000000000000000000000000000000",
              "refundReceiver": "0x0000000000000000000000000000000000000000",
              "signatures": "0x00000000000000000000000012345647579d3685e2f908fc3d3b9df7320149d400000000000000000000000000000000000000000000000000000000000000000152e3037047687bbfc1d4df0b140431dae7b6190040f94017095e060cc8a799c260c88ffd7f82d6f5f63305b729090a580558f57b05671dace5bb3fa149c691c71b"
            }
        "#;
        let tx: TxInput = serde_json::from_str(input).unwrap();
        assert_eq!(tx.to, address!("0x1f28d065e77c8cb223bbb7c5edb3a432268e5811"));
        assert_eq!(tx.value, U256::from(0));
        assert_eq!(
            tx.data,
            "0xa9059cbb000000000000000000000000fa5aa11148a2181b1b6e4e290709b665fd1781f500000000000000000000000000000000000000000000000000000000001e8480"
        );
        assert_eq!(tx.operation, 0);
        assert_eq!(tx.safe_tx_gas, U256::ZERO);
        assert_eq!(tx.base_gas(), U256::ZERO);
        assert_eq!(tx.gas_price, U256::ZERO);
        assert_eq!(tx.gas_token, Address::ZERO);
        assert_eq!(tx.refund_receiver, Address::ZERO);
        assert_eq!(
            tx.signatures,
            "0x00000000000000000000000012345647579d3685e2f908fc3d3b9df7320149d400000000000000000000000000000000000000000000000000000000000000000152e3037047687bbfc1d4df0b140431dae7b6190040f94017095e060cc8a799c260c88ffd7f82d6f5f63305b729090a580558f57b05671dace5bb3fa149c691c71b"
        );
    }
}

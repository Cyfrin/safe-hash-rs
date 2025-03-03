//! Upon simulation (in Tenderly), the Summary section contains input and output json.
//! This module will try to deserialize the input json to have a strongly typed representation.

use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

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
    pub fn base_gas(&self) -> U256 {
        if let Some(base_gas) = self.base_gas {
            return base_gas;
        }
        self.data_gas.expect("neither data_gas not base_gas was found")
    }
}

#[cfg(test)]
mod tests {

    use alloy::primitives::address;

    use super::*;

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

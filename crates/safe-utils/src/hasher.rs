use crate::{Result, SafeWalletVersion};

use alloy::{
    dyn_abi::DynSolValue,
    hex,
    primitives::{Address, B256, ChainId, U256, eip191_hash_message, keccak256},
};

pub struct DomainHasher {
    safe_version: SafeWalletVersion,
    chain_id: ChainId,
    safe_contract: Address,
}

pub struct CallDataHasher {
    calldata: String,
}

pub struct TxMessageHasher {
    safe_version: SafeWalletVersion,
    to: Address,
    value: U256,
    data_hashed: B256,
    operation: u8,
    safe_tx_gas: U256,
    base_gas: U256,
    gas_price: U256,
    gas_token: Address,
    refund_receiver: Address,
    nonce: U256,
}

pub struct SafeHasher {
    domain_hash: B256,
    message_hash: B256,
}

#[derive(Debug)]
pub struct ExecuteTxHasher {
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
}

pub struct MessageHasher {
    message: String,
}

impl DomainHasher {
    pub fn new(safe_version: SafeWalletVersion, chain_id: ChainId, safe_contract: Address) -> Self {
        Self { safe_version, chain_id, safe_contract }
    }
    pub fn hash(&self) -> B256 {
        if self.safe_version >= SafeWalletVersion::new(1, 3, 0) {
            return keccak256(
                DynSolValue::Tuple(vec![
                    DynSolValue::FixedBytes(
                        keccak256("EIP712Domain(uint256 chainId,address verifyingContract)"),
                        32,
                    ),
                    DynSolValue::Uint(U256::from(self.chain_id), 256),
                    self.safe_contract.into(),
                ])
                .abi_encode(),
            );
        }
        keccak256(
            DynSolValue::Tuple(vec![
                DynSolValue::FixedBytes(keccak256("EIP712Domain(address verifyingContract)"), 32),
                self.safe_contract.into(),
            ])
            .abi_encode(),
        )
    }
}

impl CallDataHasher {
    pub fn new(calldata: String) -> Self {
        Self { calldata }
    }
    pub fn hash(&self) -> Result<B256> {
        let bytes = hex::decode(self.calldata.clone())?;
        Ok(keccak256(bytes))
    }
}

impl TxMessageHasher {
    pub fn new(
        safe_version: SafeWalletVersion,
        to: Address,
        value: U256,
        data_hashed: B256,
        operation: u8,
        safe_tx_gas: U256,
        base_gas: U256,
        gas_price: U256,
        gas_token: Address,
        refund_receiver: Address,
        nonce: U256,
    ) -> Self {
        Self {
            safe_version,
            to,
            value,
            data_hashed,
            operation,
            safe_tx_gas,
            base_gas,
            gas_price,
            gas_token,
            refund_receiver,
            nonce,
        }
    }

    pub fn hash(&self) -> B256 {
        let typehash = if self.safe_version >= SafeWalletVersion::new(1, 0, 0) {
            keccak256(
                "SafeTx(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 baseGas,uint256 gasPrice,address gasToken,address refundReceiver,uint256 nonce)",
            )
        } else {
            keccak256(
                "SafeTx(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 dataGas,uint256 gasPrice,address gasToken,address refundReceiver,uint256 nonce)",
            )
        };
        return keccak256(
            DynSolValue::Tuple(vec![
                DynSolValue::FixedBytes(typehash, 32),
                self.to.into(),
                self.value.into(),
                DynSolValue::FixedBytes(self.data_hashed, 32),
                self.operation.into(),
                self.safe_tx_gas.into(),
                self.base_gas.into(),
                self.gas_price.into(),
                self.gas_token.into(),
                self.refund_receiver.into(),
                self.nonce.into(),
            ])
            .abi_encode(),
        );
    }
}

impl SafeHasher {
    pub fn new(domain_hash: B256, message_hash: B256) -> Self {
        Self { domain_hash, message_hash }
    }
    pub fn hash(&self) -> B256 {
        keccak256([&[0x19, 0x01], &self.domain_hash[..], &self.message_hash[..]].concat())
    }
}

impl ExecuteTxHasher {
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
            base_gas,
            gas_price,
            gas_token,
            refund_receiver,
            signatures,
        }
    }

    pub fn calldata(&self) -> Vec<u8> {
        let function_selector = &keccak256(
            "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)",
        )[..4];

        let data_bytes = hex::decode(self.data.clone()).expect("corrupted calldata");
        let signature_bytes = hex::decode(self.signatures.clone()).expect("corrupted signature");

        let encoded_arguments = DynSolValue::Tuple(vec![
            DynSolValue::Address(self.to),
            DynSolValue::Uint(self.value, 256),
            DynSolValue::Bytes(data_bytes),
            DynSolValue::Uint(U256::from(self.operation), 8),
            DynSolValue::Uint(U256::from(self.safe_tx_gas), 256),
            DynSolValue::Uint(U256::from(self.base_gas), 256),
            DynSolValue::Uint(U256::from(self.gas_price), 256),
            DynSolValue::Address(self.gas_token),
            DynSolValue::Address(self.refund_receiver),
            DynSolValue::Bytes(signature_bytes),
        ])
        .abi_encode_params();

        [function_selector, &encoded_arguments[..]].concat()
    }

    pub fn calldata_hash(&self) -> B256 {
        keccak256(self.calldata())
    }
}

impl MessageHasher {
    pub fn new(message: String) -> Self {
        Self { message }
    }
    pub fn hash(&self) -> B256 {
        keccak256(
            DynSolValue::Tuple(vec![
                DynSolValue::FixedBytes(keccak256("SafeMessage(bytes message)"), 32),
                DynSolValue::FixedBytes(
                    keccak256(
                        DynSolValue::FixedBytes(eip191_hash_message(self.message.clone()), 32)
                            .abi_encode(),
                    ),
                    32,
                ),
            ])
            .abi_encode(),
        )
    }
}

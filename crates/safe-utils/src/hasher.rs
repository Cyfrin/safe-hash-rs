use crate::{Result, SafeWalletVersion};

use alloy::{
    dyn_abi::DynSolValue,
    hex,
    primitives::{Address, B256, ChainId, U256, eip191_hash_message, keccak256},
};

pub struct DomainHasher {
    safe_version: SafeWalletVersion,
    chain_id: ChainId,
    safe_address: Address,
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

pub struct MessageHasher {
    message: String,
}

impl DomainHasher {
    pub fn new(safe_version: SafeWalletVersion, chain_id: ChainId, safe_address: Address) -> Self {
        Self { safe_version, chain_id, safe_address }
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
                    self.safe_address.into(),
                ])
                .abi_encode(),
            );
        }
        keccak256(
            DynSolValue::Tuple(vec![
                DynSolValue::FixedBytes(keccak256("EIP712Domain(address verifyingContract)"), 32),
                self.safe_address.into(),
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
        keccak256(
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
        )
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

impl MessageHasher {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn raw_hash(&self) -> B256 {
        eip191_hash_message(self.message.clone())
    }

    pub fn hash(&self) -> B256 {
        let hashed_message = self.raw_hash();
        keccak256(
            DynSolValue::Tuple(vec![
                DynSolValue::FixedBytes(keccak256("SafeMessage(bytes message)"), 32),
                DynSolValue::FixedBytes(
                    keccak256(DynSolValue::FixedBytes(hashed_message, 32).abi_encode()),
                    32,
                ),
            ])
            .abi_encode(),
        )
    }
}

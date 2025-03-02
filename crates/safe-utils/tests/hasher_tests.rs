use alloy::{
    hex,
    primitives::{Address, ChainId, U256, address},
};
use safe_utils::*;

#[test]
fn test_signing_hasher() {
    let to = address!("0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9");
    let chain_id = ChainId::of("sepolia").expect("failed to find sepolia's chain id");
    let nonce = 6;
    let safe_contract = address!("0x86D46EcD553d25da0E3b96A9a1B442ac72fa9e9F");

    // Default
    let safe_version = SafeWalletVersion::new(1, 3, 0);

    // Optional
    let data = CallDataHasher::new("0x095ea7b3000000000000000000000000fe2f653f6579de62aaf8b186e618887d03fa31260000000000000000000000000000000000000000000000000000000000000001".to_string());

    let message_hasher = TxMessageHasher::new(
        safe_version.clone(),
        to,
        U256::ZERO,
        data.hash().unwrap(),
        0,
        U256::ZERO,
        U256::ZERO,
        U256::ZERO,
        Address::ZERO,
        Address::ZERO,
        U256::from(nonce),
    );
    let domain_hasher = DomainHasher::new(safe_version, chain_id, safe_contract);
    let safe_hasher = SafeHasher::new(domain_hasher.hash(), message_hasher.hash());
    let readable_hash = hex::encode(safe_hasher.hash());
    assert_eq!(
        readable_hash,
        "213be037275c94449a28b4edead76b0d63c7e12b52257f9d5686d98b9a1a5ff4".to_string()
    );
}

#[test]
fn test_execution_hasher() {
    let to = address!("0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9");

    let executor = ExecuteTxHasher::new(
        to,
        U256::ZERO,
        "0x095ea7b3000000000000000000000000fe2f653f6579de62aaf8b186e618887d03fa31260000000000000000000000000000000000000000000000000000000000000001".to_string(),
        0,
        U256::ZERO,
        U256::ZERO,
        U256::ZERO,
        Address::ZERO,
        Address::ZERO,
        "0x13b45080c8c2bf4df80f215bd6db3a685d44e884e152f210d33a710e8c4a0cd7690a0870ff4b6c91caace6805c3089e969127bda662115303e2b4d32613e30161c52e3137647687bbfc1d4df0be44431dae7b6192f4cf94e17395e866cc8a799c260c88ffd7f82d6f5f63305b729090a580558f57b05671daee5bb3fa249c691c71b".to_string(),
    );

    assert_eq!(
        executor.calldata_hash().to_string(),
        "0xf618babd29e892fab850c54beeb23fa111479553afcd3f549f0f1a69dbd24a98".to_string()
    );
}

use alloy::{
    hex,
    primitives::{Address, ChainId, U256, address},
};
use safe_utils::*;

#[test]
fn test_hasher() {
    let to = address!("0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9");
    let chain_id = ChainId::of("sepolia").expect("failed to find sepolia's chain id");
    let nonce = 6;
    let safe_contract = address!("0x86D46EcD553d25da0E3b96A9a1B442ac72fa9e9F");

    // Default
    let safe_version = SafeWalletVersion::new(1, 3, 0);

    // Optional
    let data = CallDataHasher::new("0x095ea7b3000000000000000000000000fe2f653f6579de62aaf8b186e618887d03fa31260000000000000000000000000000000000000000000000000000000000000001".to_string());

    let message_hasher = MessageHasher::new(
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
    let safe_hasher = SafeTxHasher::new(domain_hasher.hash(), message_hasher.hash());
    let readable_hash = hex::encode(safe_hasher.hash());
    assert_eq!(
        readable_hash,
        "213be037275c94449a28b4edead76b0d63c7e12b52257f9d5686d98b9a1a5ff4".to_string()
    );
}

#[test]
fn test_execution_hasher() {
    //ExecuteTxHasher::calldata();
}

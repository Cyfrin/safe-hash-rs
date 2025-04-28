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
    let safe_address = address!("0x86D46EcD553d25da0E3b96A9a1B442ac72fa9e9F");

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
    let domain_hasher = DomainHasher::new(safe_version, chain_id, safe_address);
    let safe_hasher = SafeHasher::new(domain_hasher.hash(), message_hasher.hash());
    let readable_hash = hex::encode(safe_hasher.hash());
    assert_eq!(
        readable_hash,
        "213be037275c94449a28b4edead76b0d63c7e12b52257f9d5686d98b9a1a5ff4".to_string()
    );
}

use std::process::Command;

#[test]
fn test_safe_hash_tx_signing_cli_output() {
    // Run the safe-hash command with some test arguments
    let output = Command::new("cargo")
            .arg("run")
        .arg("-p")
        .arg("safe-hash")
            .arg("--")
            .arg("tx")
            .arg("--chain")
            .arg("ethereum")
            .arg("--nonce")
            .arg("63")
            .arg("--safe-address")
            .arg("0x1c694Fc3006D81ff4a56F97E1b99529066a23725")
            .arg("--to")
            .arg("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")
            .arg("--data")
            .arg("0xa9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00")
            .output()
            .expect("Failed to execute command");

    // Assert that the command executed successfully
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    // Check for essential content without formatting
    // Domain hash
    assert!(stdout.contains("1655e94a9bcc5a957daa1acae692b4c22e7aaf146b4deb9194f8221d2f09d8c3"));
    // Message hash
    assert!(stdout.contains("f22754eba5a2b230714534b4657195268f00dc0031296de4b835d82e7aa1e574"));
    // Safe transaction hash
    assert!(stdout.contains("ad06b099fca34e51e4886643d95d9a19ace2cd024065efb66662a876e8c40343"));
}

#[test]
fn test_safe_hash_msg_signing_cli_output() {
    // Run the safe-hash command with some test arguments
    let output = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("safe-hash")
        .arg("--")
        .arg("msg")
        .arg("--chain")
        .arg("sepolia")
        .arg("--safe-address")
        .arg("0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1")
        .arg("--input-file")
        .arg("../../test/test_message.txt")
        .output()
        .expect("Failed to execute command");

    // Assert that the command executed successfully
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    // Check for essential content without formatting
    // Domain hash
    assert!(stdout.contains("611379c19940caee095cdb12bebe6a9fa9abb74cdb1fbd7377c49a1f198dc24f"));
    // Message hash
    assert!(stdout.contains("a5d2f507a16279357446768db4bd47a03bca0b6acac4632a4c2c96af20d6f6e5"));
    // Safe transaction hash
    assert!(stdout.contains("1866b559f56261ada63528391b93a1fe8e2e33baf7cace94fc6b42202d16ea08"));
}

#[test]
fn test_eip712_hash() {
    // Run the safe-hash command with some test arguments
    let output = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("safe-hash")
        .arg("--")
        .arg("typed")
        .arg("--file")
        .arg("../../test/pat_eip712_message.json")
        .output()
        .expect("Failed to execute command");

    // Assert that the command executed successfully
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    let expected_eip712_hash = "0x76ea36b85e6de361baa7cb21a064a2a985bd2ce751407345d408cee923e94a41";
    let expected_domain_hash = "0x4b060214423f60c76da4f8d80253b7c6b565786aaae7afef3be4ae257c66761b";
    let expected_message_hash =
        "0xdc266ecad0ca5863351fbfa07d82d034df09779408013c1b4b28a2317e5c7909";

    assert!(stdout.contains(expected_eip712_hash));
    assert!(stdout.contains(expected_domain_hash));
    assert!(stdout.contains(expected_message_hash));
}

#[test]
fn test_eip712_hash_2() {
    // Run the safe-hash command with some test arguments
    let output = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("safe-hash")
        .arg("--")
        .arg("typed")
        .arg("--file")
        .arg("../../test/another_example.json")
        .output()
        .expect("Failed to execute command");

    // Assert that the command executed successfully
    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    let expected_eip712_hash = "0x1497354498e270c5ae2652ab217b16deb04c5017635f1f5f229f541998495a6a";
    let expected_domain_hash = "0x04836bd2dbdcbf68aef3b82e83babee1e988fb3c6fe1e9319d77a125e02a66cd";
    let expected_message_hash =
        "0x9b80505023ee7505d826afcf368ef1df4d18e088e773cf51dc5484858be18187";

    assert!(stdout.contains(expected_eip712_hash));
    assert!(stdout.contains(expected_domain_hash));
    assert!(stdout.contains(expected_message_hash));
}

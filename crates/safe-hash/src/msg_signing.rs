use crate::{cli::MessageArgs, output::SafeHashes};
use alloy::primitives::ChainId;
use safe_utils::{DomainHasher, MessageHasher, SafeHasher, SafeWalletVersion};

pub struct MsgInput {
    pub message: String,
}

impl MsgInput {
    pub fn new(message: String) -> Self {
        let message_raw = message.replace("\r\n", "\n");
        Self { message: message_raw }
    }
}

pub fn msg_signing_hashes(
    msg_data: &MsgInput,
    args: &MessageArgs,
    chain_id: ChainId,
) -> SafeHashes {
    // Calculate hashes
    let domain_hash = {
        let domain_hasher =
            DomainHasher::new(args.safe_version.clone(), chain_id, args.safe_address);
        domain_hasher.hash()
    };

    let raw_message_hash = {
        let message_hasher = MessageHasher::new(msg_data.message.clone());
        message_hasher.raw_hash()
    };

    let message_hash = {
        let message_hasher = MessageHasher::new(msg_data.message.clone());
        message_hasher.hash()
    };

    let safe_tx_hash = {
        let safe_hasher = SafeHasher::new(domain_hash, message_hash);
        safe_hasher.hash()
    };

    SafeHashes { domain_hash, message_hash, safe_tx_hash, raw_message_hash: Some(raw_message_hash) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, ChainId, FixedBytes, hex};
    use safe_utils::{Of, SafeWalletVersion};
    use std::{fs, str::FromStr};
    #[test]
    fn test_msg_signing_hashes() {
        // Create test inputs
        let safe_address = Address::from_str("0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1").unwrap();

        let args = MessageArgs {
            chain: "sepolia".to_string(),
            input_file: "../../test/test_message.txt".to_string(),
            safe_address,
            safe_version: SafeWalletVersion::new(1, 3, 0),
        };

        let message = fs::read_to_string(&args.input_file)
            .unwrap_or_else(|_| panic!("Failed to read message file: {}", args.input_file));
        let msg_data = MsgInput::new(message);
        let chain_id = ChainId::of("sepolia").unwrap();
        let hashes = msg_signing_hashes(&msg_data, &args, chain_id);

        // Note: These expected values are placeholders and need to be replaced with actual values
        // from a known good test case
        let expected_domain = FixedBytes::new(
            hex::decode("611379c19940caee095cdb12bebe6a9fa9abb74cdb1fbd7377c49a1f198dc24f")
                .unwrap()
                .try_into()
                .unwrap(),
        );
        let expected_message = FixedBytes::new(
            hex::decode("a5d2f507a16279357446768db4bd47a03bca0b6acac4632a4c2c96af20d6f6e5")
                .unwrap()
                .try_into()
                .unwrap(),
        );
        let expected_safe = FixedBytes::new(
            hex::decode("1866b559f56261ada63528391b93a1fe8e2e33baf7cace94fc6b42202d16ea08")
                .unwrap()
                .try_into()
                .unwrap(),
        );

        // Assert outputs match expected values
        assert_eq!(hashes.domain_hash, expected_domain, "Domain hash mismatch");
        assert_eq!(hashes.message_hash, expected_message, "Message hash mismatch");
        assert_eq!(hashes.safe_tx_hash, expected_safe, "Safe message hash mismatch");
    }
}

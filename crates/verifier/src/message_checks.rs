use crate::cli::CliArgs;
use alloy::{hex, primitives::ChainId};
use safe_utils::{DomainHasher, MessageHasher, SafeHasher, SafeWalletVersion};

pub fn handle_checks_for_message_hash(
    args: &CliArgs,
    chain_id: ChainId,
    safe_verion: SafeWalletVersion,
) {
    // Calculate hashes
    let domain_hash = {
        let domain_hasher = DomainHasher::new(safe_verion.clone(), chain_id, args.safe_contract);
        domain_hasher.hash()
    };

    let message_hash = {
        let content =
            std::fs::read_to_string(args.message_file.clone().expect("message file not found"))
                .expect("unable to read message file");

        let message_hasher = MessageHasher::new(content);
        message_hasher.hash()
    };

    let safe_hash = {
        let safe_hasher = SafeHasher::new(domain_hash, message_hash);
        safe_hasher.hash()
    };

    // Output hashes
    println!("Calculated hashes");
    println!("Domain Hash       :{}", hex::encode(domain_hash));
    println!("Message Hash      :{}", hex::encode(message_hash));
    println!("Safe Message Hash :{}", hex::encode(safe_hash));
    println!()
}

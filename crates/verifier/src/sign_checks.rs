use crate::{cli::CliArgs, tx_file::TenderlyTxInput};
use alloy::{
    hex,
    primitives::{ChainId, U256},
};
use safe_utils::{CallDataHasher, DomainHasher, MessageHasher, SafeTxHasher, SafeWalletVersion};

pub fn handle_checks_for_signing(
    tx_data: &TenderlyTxInput,
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
        let calldata_hash = {
            let calldata_hasher = CallDataHasher::new(tx_data.data.clone());
            calldata_hasher.hash().expect(&format!("unable to hash {:?}", tx_data.data))
        };
        let message_hasher = MessageHasher::new(
            safe_verion,
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
        let safe_hasher = SafeTxHasher::new(domain_hash, message_hash);
        safe_hasher.hash()
    };

    // Output hashes
    println!("Calculated hashes");
    println!("Domain Hash           :{}", hex::encode(domain_hash));
    println!("Message Hash          :{}", hex::encode(message_hash));
    println!("Safe Transaction Hash :{}", hex::encode(safe_hash));
    println!()
}

use crate::{cli::CliArgs, tx_file::TxInput};
use ::sty::{magenta_bright, underline};
use alloy::{
    hex,
    primitives::{ChainId, U256},
};
use cli_table::{Cell, Style, Table};
use color_print::{cprintln, cstr};
use safe_utils::{CallDataHasher, DomainHasher, SafeHasher, SafeWalletVersion, TxMessageHasher};

pub fn handle_checks_for_signing(
    tx_data: &TxInput,
    args: &CliArgs,
    chain_id: ChainId,
    safe_verion: SafeWalletVersion,
) {
    // Calculate hashes
    let domain_hash = {
        let domain_hasher = DomainHasher::new(safe_verion.clone(), chain_id, args.safe_address);
        domain_hasher.hash()
    };

    let message_hash = {
        let calldata_hash = {
            let calldata_hasher = CallDataHasher::new(tx_data.data.clone());
            calldata_hasher.hash().unwrap_or_else(|_| panic!("unable to hash {:?}", tx_data.data))
        };
        let message_hasher = TxMessageHasher::new(
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
        let safe_hasher = SafeHasher::new(domain_hash, message_hash);
        safe_hasher.hash()
    };

    println!("{}", sty::sty!("SIGNATURE CHECKS", [magenta_bright, underline]));
    println!();

    // Print hashes
    let table = vec![
        vec![cstr!("<green>Domain Hash</>").cell(), hex::encode(domain_hash).cell()],
        vec![cstr!("<green>Message Hash</>").cell(), hex::encode(message_hash).cell()],
        vec![cstr!("<green>Safe Transaction Hash</>").cell(), hex::encode(safe_hash).cell()],
    ]
    .table()
    .title(vec![
        cstr!("<cyan>TYPE</>").cell().bold(true),
        cstr!("<cyan>CALCULATED HASHES</cyan>").cell().bold(true),
    ])
    .bold(true);

    let table_display = table.display().unwrap();
    println!("{}", table_display);

    cprintln!(
        "<bold>Verify the above value as the Safe Tx Hash when signing the message from the ledger.</bold>"
    );
    println!(
        "This also assumes that you are happy with the Tenderly simulation and you are okay with signing the same.\n"
    );
}

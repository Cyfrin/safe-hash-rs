use crate::cli::CliArgs;
use alloy::{hex, primitives::ChainId};
use cli_table::{Cell, Style, Table};
use color_print::cstr;
use safe_utils::{DomainHasher, MessageHasher, SafeHasher, SafeWalletVersion};
use sty::{magenta_bright, underline};

pub fn handle_checks_for_message_hash(
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

    println!("{}", sty::sty!("MESSAGE CHECKS", [magenta_bright, underline]));
    println!();

    // Print hashes
    let table = vec![
        vec![cstr!("<green>Domain Hash</>").cell(), hex::encode(domain_hash).cell()],
        vec![cstr!("<green>Message Hash</>").cell(), hex::encode(message_hash).cell()],
        vec![cstr!("<green>Safe Message Hash</>").cell(), hex::encode(safe_hash).cell()],
    ]
    .table()
    .title(vec![
        cstr!("<cyan>TYPE</>").cell().bold(true),
        cstr!("<cyan>CALCULATED HASHES</cyan>").cell().bold(true),
    ])
    .bold(true);

    let table_display = table.display().unwrap();
    println!("{}\n", table_display);
}

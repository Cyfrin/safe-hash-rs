use alloy::{hex, primitives::FixedBytes};
use cli_table::{Cell, Style, Table};
use color_print::{cprintln, cstr};

pub struct SafeHashes {
    pub domain_hash: FixedBytes<32>,
    pub message_hash: FixedBytes<32>,
    pub safe_tx_hash: FixedBytes<32>,
}

pub struct SafeWarnings {
    pub zero_address: bool,
    pub zero_value: bool,
    pub empty_data: bool,
    pub delegatecall: bool,
    pub non_zero_gas_token: bool,
    pub non_zero_refund_receiver: bool,
}

impl SafeWarnings {
    pub fn new() -> Self {
        Self {
            zero_address: false,
            zero_value: false,
            empty_data: false,
            delegatecall: false,
            non_zero_gas_token: false,
            non_zero_refund_receiver: false,
        }
    }

    pub fn has_warnings(&self) -> bool {
        self.zero_address
            || self.zero_value
            || self.empty_data
            || self.delegatecall
            || self.non_zero_gas_token
            || self.non_zero_refund_receiver
    }
}

pub fn display_hashes(hashes: &SafeHashes) {
    let table = vec![
        vec![cstr!("<green>Domain Hash</>").cell(), hex::encode(hashes.domain_hash).cell()],
        vec![cstr!("<green>Message Hash</>").cell(), hex::encode(hashes.message_hash).cell()],
        vec![
            cstr!("<green>Safe Transaction Hash</>").cell(),
            hex::encode(hashes.safe_tx_hash).cell(),
        ],
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
}

pub fn display_warnings(warnings: &SafeWarnings) {
    if warnings.has_warnings() {
        cprintln!("<bold>Warnings:</bold>");
        if warnings.zero_address {
            cprintln!("• Transaction is being sent to the zero address");
        }
        if warnings.zero_value {
            cprintln!("• Transaction has zero value");
        }
        if warnings.empty_data {
            cprintln!("• Transaction has empty data");
        }
        if warnings.delegatecall {
            cprintln!("• Transaction is using delegatecall");
        }
        if warnings.non_zero_gas_token {
            cprintln!("• Transaction is using a non-zero gas token");
        }
        if warnings.non_zero_refund_receiver {
            cprintln!("• Transaction has a non-zero refund receiver");
        }
        println!();
        cprintln!("<bold>Please review the above warnings before signing the transaction.</bold>");
    }
}

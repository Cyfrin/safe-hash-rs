use alloy::{hex, primitives::FixedBytes};
use cli_table::{Cell, Style, Table};
use color_print::{cprintln, cstr};

pub struct SafeHashes {
    pub raw_message_hash: Option<FixedBytes<32>>,
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

pub fn display_api_transaction_details(tx: &crate::api::SafeTransaction) {
    let mut table_rows = Vec::new();

    // Add basic transaction details
    table_rows.push(vec![cstr!("<green>Safe Address</>").cell(), tx.safe.to_string().cell()]);
    table_rows.push(vec![cstr!("<green>To</>").cell(), tx.to.to_string().cell()]);
    table_rows.push(vec![cstr!("<green>Value</>").cell(), tx.value.clone().cell()]);
    table_rows.push(vec![cstr!("<green>Data</>").cell(), tx.data.clone().cell()]);
    table_rows.push(vec![cstr!("<green>Operation</>").cell(), tx.operation.to_string().cell()]);
    table_rows.push(vec![cstr!("<green>Nonce</>").cell(), tx.nonce.to_string().cell()]);

    // Add gas details
    table_rows.push(vec![cstr!("<green>Safe Tx Gas</>").cell(), tx.safe_tx_gas.to_string().cell()]);
    table_rows.push(vec![cstr!("<green>Base Gas</>").cell(), tx.base_gas.to_string().cell()]);
    table_rows.push(vec![cstr!("<green>Gas Price</>").cell(), tx.gas_price.clone().cell()]);
    table_rows.push(vec![cstr!("<green>Gas Token</>").cell(), tx.gas_token.to_string().cell()]);
    table_rows.push(vec![cstr!("<green>Refund Receiver</>").cell(), tx.refund_receiver.to_string().cell()]);

    // Add confirmation details
    table_rows.push(vec![cstr!("<green>Confirmations Required</>").cell(), tx.confirmations_required.to_string().cell()]);
    table_rows.push(vec![cstr!("<green>Confirmations Count</>").cell(), tx.confirmations.len().to_string().cell()]);

    // Add decoded data if available
    if let Some(decoded) = &tx.data_decoded {
        // Create parameters table with method and parameters
        let mut param_rows = Vec::new();
        param_rows.push(vec![
            cstr!("<blue>Method</>").cell(),
            decoded.method.clone().cell(),
        ]);
        
        for param in &decoded.parameters {
            param_rows.push(vec![
                cstr!("<blue>Parameter</>").cell(),
                format!("{}: {}", param.r#type, param.value).cell(),
            ]);
        }
        
        // Print the main transaction details table
        let table = table_rows.table()
            .title(vec![
                cstr!("<cyan>FIELD</>").cell().bold(true),
                cstr!("<cyan>VALUE</>").cell().bold(true),
            ])
            .bold(true);
        println!("{}", table.display().unwrap());
        
        // Print the parameters table if we have any
        if !param_rows.is_empty() {
            println!(); // Add spacing between tables
            let param_table = param_rows.table()
                .title(vec![
                    cstr!("<cyan>SELECTOR</>").cell().bold(true),
                    cstr!("<cyan>VALUE</>").cell().bold(true),
                ])
                .bold(true);
            println!("{}", param_table.display().unwrap());
        }
    } else {
        // If no decoded data, just print the main table
        let table = table_rows.table()
            .title(vec![
                cstr!("<cyan>FIELD</>").cell().bold(true),
                cstr!("<cyan>VALUE</>").cell().bold(true),
            ])
            .bold(true);
        println!("{}", table.display().unwrap());
    }
}

pub fn display_hashes(hashes: &SafeHashes) {
    let mut table_rows = Vec::new();

    // Add raw message hash if available
    if let Some(raw_hash) = hashes.raw_message_hash {
        table_rows
            .push(vec![cstr!("<green>Raw Message Hash</>").cell(), hex::encode(raw_hash).cell()]);
    }

    // Add the standard hashes
    table_rows
        .push(vec![cstr!("<green>Domain Hash</>").cell(), hex::encode(hashes.domain_hash).cell()]);
    table_rows.push(vec![
        cstr!("<green>Message Hash</>").cell(),
        hex::encode(hashes.message_hash).cell(),
    ]);
    table_rows.push(vec![
        cstr!("<green>Safe Transaction Hash</>").cell(),
        hex::encode(hashes.safe_tx_hash).cell(),
    ]);

    let table = table_rows
        .table()
        .title(vec![
            cstr!("<cyan>HASH TYPE</>").cell().bold(true),
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

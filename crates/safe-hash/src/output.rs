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
    pub argument_mismatches: Vec<String>,
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
            argument_mismatches: Vec::new(),
        }
    }

    pub fn has_warnings(&self) -> bool {
        self.zero_address
            || self.zero_value
            || self.empty_data
            || self.delegatecall
            || self.non_zero_gas_token
            || self.non_zero_refund_receiver
            || !self.argument_mismatches.is_empty()
    }

    pub fn union(&mut self, other: Self) {
        // Merge boolean flags using OR
        self.zero_address |= other.zero_address;
        self.zero_value |= other.zero_value;
        self.empty_data |= other.empty_data;
        self.delegatecall |= other.delegatecall;
        self.non_zero_gas_token |= other.non_zero_gas_token;
        self.non_zero_refund_receiver |= other.non_zero_refund_receiver;
        
        // Merge vectors using extend
        self.argument_mismatches.extend(other.argument_mismatches);
    }
}

pub fn display_api_transaction_details(tx: &crate::api::SafeTransaction) {
    let mut table_rows = Vec::new();

    // Add basic transaction details
    table_rows.push(vec![cstr!("<yellow>Safe Address</>").cell(), tx.safe.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>To</>").cell(), tx.to.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Value</>").cell(), tx.value.clone().cell()]);
    table_rows.push(vec![cstr!("<yellow>Data</>").cell(), tx.data.clone().cell()]);
    table_rows.push(vec![cstr!("<yellow>Operation</>").cell(), tx.operation.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Nonce</>").cell(), tx.nonce.to_string().cell()]);

    // Add gas details
    table_rows.push(vec![cstr!("<yellow>Safe Tx Gas</>").cell(), tx.safe_tx_gas.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Base Gas</>").cell(), tx.base_gas.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Gas Price</>").cell(), tx.gas_price.clone().cell()]);
    table_rows.push(vec![cstr!("<yellow>Gas Token</>").cell(), tx.gas_token.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Refund Receiver</>").cell(), tx.refund_receiver.to_string().cell()]);

    // Add confirmation details
    table_rows.push(vec![cstr!("<yellow>Confirmations Required</>").cell(), tx.confirmations_required.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Confirmations Count</>").cell(), tx.confirmations.len().to_string().cell()]);

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
        println!(); // Add spacing before warnings
        cprintln!("<bold><red>⚠️  WARNINGS:</red></bold>");
        
        // Display standard warnings
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

        // Display argument mismatches prominently
        if !warnings.argument_mismatches.is_empty() {
            println!(); // Add spacing between standard warnings and argument mismatches
            cprintln!("<bold><red>🚨 ARGUMENT MISMATCHES:</red></bold>");
            
            for mismatch in &warnings.argument_mismatches {
                // Parse the mismatch message to extract API and user values
                if let Some((field, api_value, user_value)) = parse_mismatch_message(mismatch) {
                    let mut mismatch_rows = Vec::new();
                    mismatch_rows.push(vec![
                        cstr!("<yellow>API:</>").cell(),
                        api_value.cell(),
                    ]);
                    mismatch_rows.push(vec![
                        cstr!("<yellow>User:</>").cell(),
                        user_value.cell(),
                    ]);

                    // Print the mismatch table
                    let mismatch_table = mismatch_rows.table()
                        .title(vec![
                            format!("{} MISMATCH", field.to_uppercase()).cell().bold(true),
                            cstr!("<cyan>VALUE</>").cell().bold(true),
                        ])
                        .bold(true);
                    println!("{}", mismatch_table.display().unwrap());
                    println!(); // Add spacing between tables
                } else {
                    // Fallback for any unparseable messages
                    let mut mismatch_rows = Vec::new();
                    mismatch_rows.push(vec![
                        cstr!("<red>Mismatch:</>").cell(),
                        mismatch.cell(),
                    ]);

                    let mismatch_table = mismatch_rows.table()
                        .title(vec![
                            cstr!("<cyan>MISMATCH</>").cell().bold(true),
                            cstr!("<cyan>DETAILS</>").cell().bold(true),
                        ])
                        .bold(true);
                    println!("{}", mismatch_table.display().unwrap());
                    println!(); // Add spacing between tables
                }
            }
        }

        println!(); // Add spacing after warnings
        cprintln!("<bold><red>Please review the above warnings before signing the transaction.</red></bold>");
    }
}

fn parse_mismatch_message(message: &str) -> Option<(String, String, String)> {
    // Handle different message formats
    if let Some(field_start) = message.find("'") {
        if let Some(field_end) = message[field_start + 1..].find("'") {
            let field = message[field_start + 1..field_start + 1 + field_end].to_string();
            
            if let Some(api_start) = message.find("API: ") {
                if let Some(user_start) = message.find("User provided: ") {
                    let api_value = message[api_start + 5..user_start].trim().trim_end_matches(',').to_string();
                    let user_value = message[user_start + 14..].trim().to_string();
                    return Some((field, api_value, user_value));
                }
            }
        }
    } else if message.contains("Transaction data mismatch") {
        // Handle data mismatch case
        if let Some(api_start) = message.find("API: ") {
            if let Some(user_start) = message.find("User provided: ") {
                let api_value = message[api_start + 5..user_start].trim().trim_end_matches(',').to_string();
                let user_value = message[user_start + 14..].trim().to_string();
                return Some(("data".to_string(), api_value, user_value));
            }
        }
    }
    None
}

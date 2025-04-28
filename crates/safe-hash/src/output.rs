use alloy::{hex, primitives::FixedBytes};
use cli_table::{Cell, Style, Table};
use color_print::{cprintln, cstr};
use safe_utils::{CalldataDecoded, EIP7127HashDetails};

pub struct SafeHashes {
    pub raw_message_hash: Option<FixedBytes<32>>,
    pub domain_hash: FixedBytes<32>,
    pub message_hash: FixedBytes<32>,
    pub safe_tx_hash: FixedBytes<32>,
}

pub struct Mismatch {
    pub field: String,
    pub api_value: String,
    pub user_value: String,
}

pub struct SafeWarnings {
    pub zero_address: bool,
    pub zero_value: bool,
    pub empty_data: bool,
    pub delegatecall: bool,
    pub non_zero_gas_token: bool,
    pub non_zero_refund_receiver: bool,
    pub argument_mismatches: Vec<Mismatch>,
    pub dangerous_methods: bool,
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
            dangerous_methods: false,
        }
    }

    pub fn has_warnings(&self) -> bool {
        self.zero_address
            || self.zero_value
            || self.empty_data
            || self.delegatecall
            || self.non_zero_gas_token
            || self.non_zero_refund_receiver
            || self.dangerous_methods
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
        self.dangerous_methods |= other.dangerous_methods;
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
    table_rows
        .push(vec![cstr!("<yellow>Safe Tx Gas</>").cell(), tx.safe_tx_gas.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Base Gas</>").cell(), tx.base_gas.to_string().cell()]);
    table_rows.push(vec![cstr!("<yellow>Gas Price</>").cell(), tx.gas_price.clone().cell()]);
    table_rows.push(vec![cstr!("<yellow>Gas Token</>").cell(), tx.gas_token.to_string().cell()]);
    table_rows.push(vec![
        cstr!("<yellow>Refund Receiver</>").cell(),
        tx.refund_receiver.to_string().cell(),
    ]);

    // Add confirmation details
    table_rows.push(vec![
        cstr!("<yellow>Confirmations Required</>").cell(),
        tx.confirmations_required.to_string().cell(),
    ]);
    table_rows.push(vec![
        cstr!("<yellow>Confirmations Count</>").cell(),
        tx.confirmations.len().to_string().cell(),
    ]);

    // Add decoded data if available
    if let Some(decoded) = &tx.data_decoded {
        // Create parameters table with method and parameters
        let mut param_rows = Vec::new();
        param_rows.push(vec![cstr!("<blue>Method</>").cell(), decoded.method.clone().cell()]);

        for param in &decoded.parameters {
            param_rows.push(vec![
                cstr!("<blue>Parameter</>").cell(),
                format!("{}: {}", param.r#type, param.value).cell(),
            ]);
        }

        // Print the main transaction details table
        let table = table_rows
            .table()
            .title(vec![
                cstr!("<cyan>FIELD</>").cell().bold(true),
                cstr!("<cyan>VALUE</>").cell().bold(true),
            ])
            .bold(true);
        println!("{}", table.display().unwrap());

        // Print the parameters table if we have any
        if !param_rows.is_empty() {
            println!(); // Add spacing between tables
            let param_table = param_rows
                .table()
                .title(vec![
                    cstr!("<cyan>SELECTOR</>").cell().bold(true),
                    cstr!("<cyan>VALUE</>").cell().bold(true),
                ])
                .bold(true);
            println!("{}", param_table.display().unwrap());
        }
    } else {
        // If no decoded data, just print the main table
        let table = table_rows
            .table()
            .title(vec![
                cstr!("<cyan>FIELD</>").cell().bold(true),
                cstr!("<cyan>VALUE</>").cell().bold(true),
            ])
            .bold(true);
        println!("{}", table.display().unwrap());
    }
}

pub fn display_calldata_decoded(decoded: &CalldataDecoded) {
    println!("Brute-force checking function signature of tx data");

    for decoded_unit in &decoded.options {
        let signature = &decoded_unit.signature;
        let arguments = &decoded_unit.arguments;

        println!("{:<12} {}", "Signature:", signature);

        if let Some(first_argument) = arguments.get(0) {
            println!("{:<12} {}", "Arguments:", first_argument);

            for argument in &arguments[1..] {
                println!("{:<12} {}", "", argument);
            }
        } else {
            println!("{:<12} {}", "Arguments:", "");
        }

        println!(); // add an empty line between different decoded units
    }

    println!(
        "If there are multiple signatures, verify the effects on the smart contract for each one by simulation before making the transaction."
    );
}

pub fn display_hashes(hashes: &SafeHashes) {
    if let Some(raw_hash) = hashes.raw_message_hash {
        println!("{:<24} {}", "Raw Message Hash:", hex::encode(raw_hash));
    }

    println!("{:<24} {}", "Domain Hash:", hex::encode(hashes.domain_hash));
    println!("{:<24} {}", "Message Hash:", hex::encode(hashes.message_hash));
    println!("{:<24} {}", "Safe Transaction Hash:", hex::encode(hashes.safe_tx_hash));

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
        if warnings.dangerous_methods {
            cprintln!(
                "• Transaction data matches a function signature that modifies the owners or threshold of the Safe."
            );
        }
        // Display argument mismatches prominently
        if !warnings.argument_mismatches.is_empty() {
            cprintln!("<bold><red>🚨 ARGUMENT MISMATCHES:</red></bold>");

            for mismatch in &warnings.argument_mismatches {
                // Parse the mismatch message to extract API and user values
                let mut mismatch_rows = Vec::new();
                mismatch_rows
                    .push(vec!["API Returned".cell(), mismatch.api_value.to_string().cell()]);
                mismatch_rows
                    .push(vec!["User Supplied".cell(), mismatch.user_value.to_string().cell()]);
                let mismatch_table = mismatch_rows
                    .table()
                    .title(vec![
                        cstr!("").cell().bold(true),
                        mismatch.field.to_string().cell().bold(true),
                    ])
                    .bold(true);
                println!("{}", mismatch_table.display().unwrap());
            }
        }

        println!(); // Add spacing after warnings
        cprintln!(
            "<bold><red>Please review the above warnings before signing the transaction.</red></bold>"
        );
    }
}

pub fn display_eip712_hash(hash: &EIP7127HashDetails) {
    let mut table_rows = Vec::new();

    table_rows.push(vec![cstr!("<green>EIP 712 Hash</>").cell(), hash.eip_712_hash.clone().cell()]);
    table_rows.push(vec![cstr!("<green>Domain Hash</>").cell(), hash.domain_hash.clone().cell()]);
    table_rows.push(vec![cstr!("<green>Message Hash</>").cell(), hash.message_hash.clone().cell()]);

    let table = table_rows.table().bold(true);

    let table_display = table.display().unwrap();
    println!("{}", table_display);
}

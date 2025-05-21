use alloy::{
    hex,
    primitives::{B256, FixedBytes},
};
use cli_table::{Cell, Style, Table};
use color_print::{cprintln, cstr};
use safe_utils::EIP7127HashDetails;

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
    println!("{:<24} {}", "Safe Address:", tx.safe);
    println!("{:<24} {}", "To:", tx.to);
    println!("{:<24} {}", "Value:", tx.value);
    println!("{:<24} {}", "Data:", tx.data);
    println!("{:<24} {}", "Operation:", tx.operation);
    println!("{:<24} {}", "Nonce:", tx.nonce);

    println!("{:<24} {}", "Safe Tx Gas:", tx.safe_tx_gas);
    println!("{:<24} {}", "Base Gas:", tx.base_gas);
    println!("{:<24} {}", "Gas Price:", tx.gas_price);
    println!("{:<24} {}", "Gas Token:", tx.gas_token);
    println!("{:<24} {}", "Refund Receiver:", tx.refund_receiver);

    println!("{:<24} {}", "Confirmations Required:", tx.confirmations_required);
    println!("{:<24} {}", "Confirmations Count:", tx.confirmations.len());

    if let Some(decoded) = &tx.data_decoded {
        println!();
        println!("Decoded Call:");

        println!("{:<12} {}", "Method:", decoded.method);

        for param in &decoded.parameters {
            println!("{:<12} {}: {}", "Parameter:", param.r#type, param.value);
        }
    }
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

pub fn display_message_hashes(hashes: &SafeHashes) {
    if let Some(raw_hash) = hashes.raw_message_hash {
        println!("{:<24} {}", "Safe Message:", hex::encode(raw_hash));
    }
    println!("{:<24} {}", "Safe Message Hash:", hex::encode(hashes.safe_tx_hash));
    println!("{:<24} {}", "Domain Hash:", hex::encode(hashes.domain_hash));
    println!("{:<24} {}", "Message Hash:", hex::encode(hashes.message_hash));
    cprintln!(
        "<bold>Verify the above value as the Safe Tx Hash when signing the message from the ledger.</bold>"
    );
}

pub fn display_full_tx(full_tx_calldata: String, hash: String) {
    println!("{:<24} {}", "Full Tx Calldata:", full_tx_calldata);
    println!("{:<24} {}", "Full Tx Calldata Hash:", hash);
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
    println!("{:<24} {}", "EIP 712 Hash:", hash.eip_712_hash.clone());
    println!("{:<24} {}", "Domain Hash:", hash.domain_hash.clone());
    println!("{:<24} {}", "Message Hash:", hash.message_hash.clone());
}

pub fn display_safe_ui_values_for_eip712(domain_hash: B256, msg_hash: B256, safe_hash: B256) {
    println!("\nSafe UI values: ");
    println!("{:<24} {}", "Safe Message Hash:", safe_hash);
    println!("{:<24} {}", "Domain Hash:", domain_hash);
    println!("{:<24} {}", "Message Hash:", msg_hash);
}

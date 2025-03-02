use alloy::primitives::{Address, U256};

use crate::tx_file::TenderlyTxInput;

pub fn warn_suspicious_content(tx_data: &TenderlyTxInput) {
    let mut warnings = vec![];

    // Check for delegate call
    if tx_data.operation == 1 {
        warnings.push("Delegatecall found in operation! Learn about the dangers here - https://www.youtube.com/watch?v=bqn-HzRclps");
    }

    // Check for gas attacks
    if tx_data.gas_token != Address::ZERO && tx_data.refund_receiver != Address::ZERO {
        warnings.push("Custom gas token and custom refund receiver found.");
        warnings.push(
            "This combination may be used to hide a redirection of funds through gas refunds.",
        );
    } else {
        if tx_data.refund_receiver != Address::ZERO {
            warnings.push(
                "Custom refund receiver found for the transaction. Verify that this is intentional",
            );
        } else if tx_data.gas_token != Address::ZERO {
            warnings.push(
                "Custom gas token found for the transaction. Verify that this is intentional",
            );
        }
    }

    if tx_data.gas_price != U256::ZERO {
        warnings.push("Gas price is non zero for the transaction, it increases potential for hidden value transfer.");
    }

    if !warnings.is_empty() {
        println!("WARNINGS:");
        warnings.iter().for_each(|line| println!("{}", line));
    }
}

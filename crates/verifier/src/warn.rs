use alloy::{
    hex,
    primitives::{Address, ChainId, U256, keccak256},
};

use crate::{etherscan::is_contract_verfied, tx_file::TenderlyTxInput};

pub fn warn_suspicious_content(tx_data: &TenderlyTxInput, chain_id: ChainId) {
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

    // Check calldata
    if is_suspicous_calldata(tx_data.data.clone()) {
        warnings.push("Suspicious calldata found. It may potentially modify the owners or the threshold of the safe.");
    }

    // Check `to` address contract verification status
    if let Ok(is_verified) = is_contract_verfied(&tx_data.to.to_string(), chain_id) {
        if !is_verified && !tx_data.data.is_empty() {
            warnings.push("Transaction data is not empty and the `to` address is not verified.");
        }
    }

    // Print warnings
    if !warnings.is_empty() {
        println!("WARNINGS\n");
        warnings.iter().for_each(|line| println!("{}\n", line));
    }
}

const SUSPICIOUS_FUNC_SIGNATURES: &[&str] = &[
    "addOwnerWithThreshold(address,uint256)",
    "removeOwner(address,address,uint256)",
    "swapOwner(address,address,address)",
    "changeThreshold(uint256)",
];

fn is_suspicous_calldata(calldata: String) -> bool {
    let suspicous_func_selectors = SUSPICIOUS_FUNC_SIGNATURES
        .iter()
        .map(|s| {
            let func_hash = keccak256(s);
            func_hash[..4].to_vec()
        })
        .collect::<Vec<_>>();

    let decoded_calldata = hex::decode(calldata).expect("unable to decode calldata");
    let first4bytes = &decoded_calldata[..4];

    suspicous_func_selectors.iter().any(|s| s == first4bytes)
}

#[cfg(test)]
mod suspicious_func_selector_tests {

    use super::*;

    #[test]
    fn test_func_selector() {
        // cast calldata "changeThreshold(uint256)" 12314
        let threshold_changing_calldata =
            "0x694e80c3000000000000000000000000000000000000000000000000000000000000301a";

        assert!(is_suspicous_calldata(threshold_changing_calldata.to_string()));
    }
}

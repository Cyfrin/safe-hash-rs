use crate::{etherscan::is_contract_verfied, tx_file::TxInput};
use alloy::{
    hex,
    primitives::{Address, ChainId, U256, keccak256},
};
use std::env::VarError;
use sty::{red_bright, underline};

pub fn warn_suspicious_content(tx_data: &TxInput, chain_id: Option<ChainId>) {
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
    } else if tx_data.refund_receiver != Address::ZERO {
        warnings.push(
            "Custom refund receiver found for the transaction. Verify that this is intentional",
        );
    } else if tx_data.gas_token != Address::ZERO {
        warnings
            .push("Custom gas token found for the transaction. Verify that this is intentional");
    }

    if tx_data.gas_price != U256::ZERO {
        warnings.push("Gas price is non zero for the transaction, it increases potential for hidden value transfer.");
    }

    // Check calldata
    if is_suspicous_calldata(tx_data.data.clone()) {
        warnings.push("Suspicious calldata found. It may potentially modify the owners or the threshold of the safe.");
    }

    // Check `to` address contract verification status
    let f = &format!(
        "Since the tx carries non zero value, check to see {} is a verified contract. Set ETHERSCAN_API_KEY in env and specify the chain for auto check",
        tx_data.to
    );
    if !tx_data.value.is_zero() {
        match chain_id.map(|chain_id| is_contract_verfied(&tx_data.to.to_string(), chain_id)) {
            Some(Ok(false)) => {
                warnings.push(
                    "Transaction carries non zero value but the `to` address is not verified.",
                );
            }
            Some(Err(err)) => {
                if err.downcast_ref::<VarError>().is_some() {
                    warnings.push(f);
                }
            }
            None => {
                warnings.push(f);
            }
            _ => {}
        };
    }

    // Print warnings
    if !warnings.is_empty() {
        println!("{}\n", sty::sty!("WARNINGS", [red_bright, underline]));
        warnings.iter().for_each(|line| println!("• {}\n", line));
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

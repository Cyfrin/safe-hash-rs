use crate::{etherscan::is_contract_verfied, output::SafeWarnings, tx_file::TxInput};
use alloy::{
    hex,
    primitives::{Address, ChainId, U256, keccak256},
};
use std::env::VarError;

pub fn check_suspicious_content(tx_data: &TxInput, chain_id: Option<ChainId>) -> SafeWarnings {
    let mut warnings = SafeWarnings::new();

    // Check for delegate call
    if tx_data.operation == 1 {
        warnings.delegatecall = true;
    }

    // Check for gas attacks
    if tx_data.gas_token != Address::ZERO && tx_data.refund_receiver != Address::ZERO {
        warnings.non_zero_gas_token = true;
        warnings.non_zero_refund_receiver = true;
    } else if tx_data.refund_receiver != Address::ZERO {
        warnings.non_zero_refund_receiver = true;
    } else if tx_data.gas_token != Address::ZERO {
        warnings.non_zero_gas_token = true;
    }

    if tx_data.gas_price != U256::ZERO {
        // Note: We don't have a field for gas price warnings in SafeWarnings
        // We could add one if needed
    }

    // Check calldata
    if is_suspicous_calldata(tx_data.data.clone()) {
        // Note: We don't have a field for suspicious calldata in SafeWarnings
        // We could add one if needed
    }

    // Check `to` address contract verification status
    if !tx_data.value.is_zero() {
        match chain_id.map(|chain_id| is_contract_verfied(&tx_data.to.to_string(), chain_id)) {
            Some(Ok(false)) => {
                // Note: We don't have a field for unverified contract in SafeWarnings
                // We could add one if needed
            }
            Some(Err(err)) => {
                if err.downcast_ref::<VarError>().is_some() {
                    // Note: We don't have a field for API key warning in SafeWarnings
                    // We could add one if needed
                }
            }
            None => {
                // Note: We don't have a field for missing chain warning in SafeWarnings
                // We could add one if needed
            }
            _ => {}
        };
    }

    warnings
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

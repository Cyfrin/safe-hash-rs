use crate::{etherscan::is_contract_verfied, output::SafeWarnings, tx_signing::TxInput};
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

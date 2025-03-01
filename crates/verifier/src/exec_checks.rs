use crate::tx_file::TenderlyTxInput;
use alloy::hex;
use safe_utils::ExecuteTxHasher;

pub fn handle_checks_for_executing(tx_data: &TenderlyTxInput) {
    let execute_tx_hasher = ExecuteTxHasher::new(
        tx_data.to,
        tx_data.value,
        tx_data.data.clone(),
        tx_data.operation,
        tx_data.safe_tx_gas,
        tx_data.base_gas(),
        tx_data.gas_price,
        tx_data.gas_token,
        tx_data.refund_receiver,
        tx_data.signatures.clone(),
    );
    let calldata = execute_tx_hasher.calldata();
    let calldata_hash = execute_tx_hasher.calldata_hash();

    println!("Assuming we have the required amount of signers, following would hold:");
    println!("Execution Calldata      :{}", hex::encode(calldata));
    println!("Execution Calldata Hash :{}", hex::encode(calldata_hash));
    println!();
}

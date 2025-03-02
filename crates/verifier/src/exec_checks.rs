use crate::tx_file::TenderlyTxInput;
use alloy::hex;
use cli_table::{Cell, Style, Table};
use color_print::{cprintln, cstr};
use safe_utils::ExecuteTxHasher;
use sty::{magenta_bright, underline};

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

    println!("{}", sty::sty!("EXECUTION CHECKS", [magenta_bright, underline]));
    println!();

    // Print hashes
    let table =
        vec![vec![cstr!("<green>Calldata hash</>").cell(), hex::encode(calldata_hash).cell()]]
            .table()
            .title(vec![
                cstr!("<cyan>TYPE</>").cell().bold(true),
                cstr!("<cyan>CALCULATED VALUES</cyan>").cell().bold(true),
            ])
            .bold(true);

    let table_display = table.display().unwrap();
    println!("{}", table_display);

    cprintln!("<green><bold>Calldata</></>");
    println!("{}\n", hex::encode(calldata));

    cprintln!(
        "<bold>Verify the above calldata as you sign when executing the transaction from the ledger.</bold>\n"
    );
}

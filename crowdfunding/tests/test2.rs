use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("");

    //blockchain.register_contract("file:output/erc20.wasm", erc20::ContractBuilder);
    blockchain
}

#[test]
fn allowance_callercaller_rs() {
    elrond_wasm_debug::mandos_rs("./mandos/adder2.scen.json", world());
}
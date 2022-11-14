#![no_std]

elrond_wasm::imports!();


// pub trait IPlayer {
//     // TODO @RMA define player attributes
//
//     fn clone(&self) -> dyn IPlayer;
// }
//
// impl Player for dyn IPlayer {
//
// }

#[elrond_wasm::contract]
pub trait Adder { // TODO @RMA rename it

    #[storage_mapper("phase")]
    fn phase(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("enigma")]
    fn enigma(&self) -> SingleValueMapper<BigUint>;

    // #[storage_mapper("progress")]
    // fn progress(&self) -> LinkedListMapper<IPlayer>;

    #[init]
    fn init(&self, phase: BigUint, enigma: BigUint) {
        self.phase().set(phase);
        self.enigma().set(enigma);

        // get_sc_balance
    }

    #[endpoint]
    fn update_progress(&self) {
        // TODO @RMA check if player has NFT
        // TODO @RMA update NFT
        // TODO @RMA update progress
    }

    fn is_ticket_valid(&self) -> bool {
        return true;
    }
}

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

    // TODO
    // #[storage_mapper("progress")]
    // fn progress(&self) -> LinkedListMapper<IPlayer>;

    #[init]
    fn init(&self, phase: BigUint, enigma: BigUint) {
        self.phase().set(phase);
        self.enigma().set(enigma);
    }

    #[payable("*")]
    #[endpoint]
    fn update_progress(&self) {
        require!(self.is_ticket_valid(), "Unable to update progress: Invalid ticket state");
        // let _caller = self.blockchain().get_caller();
        // let token_id = TokenIdentifier::from(""); // TODO get NFT id
        // let _nft = self.blockchain().get_esdt_balance(&_caller, &token_id, 1);

        // TODO fetch NFT from wallet get_sc_balance
        // TODO @RMA update NFT
        // TODO @RMA update progress
    }

    fn is_ticket_valid(&self) -> bool {
        // TODO @RMA check if player has NFT
        // TODO @RMA check if player is is at step
        return false;
    }
}

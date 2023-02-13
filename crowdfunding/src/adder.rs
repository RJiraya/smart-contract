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
    fn init(&self, phase: BigUint<Self::Api>, enigma: BigUint<Self::Api>) {
        self.phase().set(phase);
        self.enigma().set(enigma);
    }

    #[payable("*")]
    #[endpoint]
    fn update_progress(&self) {
        let token: EsdtTokenPayment<Self::Api> = self.call_value().single_esdt();
        let caller: ManagedAddress<Self::Api> = self.blockchain().get_caller();
        // let owner: ManagedAddress<Self::Api> = self.blockchain().get_owner_address();
        let token_data: EsdtTokenData<Self::Api> = self.blockchain().get_esdt_token_data(&caller, &token.token_identifier, token.token_nonce);

        // sc_print!("{}", owner.to_address());
        require!(self.is_ticket_valid(&token_data), "Unable to update progress: Invalid ticket state");

        // TODO @RMA update NFT
        // TODO @RMA update progress
    }

    fn is_ticket_valid(&self, token_data: &EsdtTokenData<Self::Api>) -> bool {
        let is_nft: bool = token_data.token_type == EsdtTokenType::NonFungible;
        // TODO @RMA use init variable for creator in case of NFT issuer different from this smart contract
        // let is_valid_ticket = token_data.creator.eq(&self.blockchain().get_caller());
        let is_valid_ticket: bool = token_data.creator == self.blockchain().get_owner_address();

        return is_valid_ticket;
    }
}
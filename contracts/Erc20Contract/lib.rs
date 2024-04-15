#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, Ownable, PSP22Mintable, PSP22Metadata, PSP22Burnable, PSP22Wrapper)]
#[openbrush::contract]
pub mod psp22 {
    use openbrush::{
        modifiers,
        traits::Storage,
    };
    use ink::storage::{
        traits::ManualKey,
        Mapping,
    };

    #[ink(event)]
    pub struct NftStatsUpdated {
        #[ink(topic)]
        nft_id: u16,
        #[ink(topic)]
        stats: [u8; 6],
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Erc20Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        wrapper: wrapper::Data,
        nfts_stats: Mapping<u16, [u8; 6], ManualKey<123>>,
        upgrade_cost: Balance,
        games: Mapping<AccountId, bool>,

    }

    impl Erc20Contract {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut _instance = Self::default();
            psp22::Internal::_mint_to(&mut _instance, Self::env().caller(), initial_supply).expect("Should mint");
            ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
            _instance.metadata.name.set(&name);
            _instance.metadata.symbol.set(&symbol);
            _instance.metadata.decimals.set(&decimal);
            _instance.upgrade_cost = 500_000_000_000_000; //500
            _instance
        }

        #[ink(message)]
        pub fn get_nft_stats(&self, nft_id: u16) -> Option<[u8; 6]> {
            self.nfts_stats.get(&nft_id)
        }

        #[ink(message)]
        pub fn add_nft_stats(&mut self, nft_id: u16, index: u8) {
            let caller = self.env().caller();
            let decimals = self.metadata.decimals.get().unwrap_or(0);
            let burn_amount = self.upgrade_cost * 10u128.pow(decimals.into());

            // Burn 6 tokens from the caller's balance
            psp22::Internal::_burn_from(self, caller, burn_amount).expect("Should burn tokens");

            let mut stats = self.nfts_stats.get(&nft_id).unwrap_or([0; 6]);
            if index < 6 {
                stats[index as usize] += 1;
            }
            self.nfts_stats.insert(&nft_id, &stats);

            Self::env().emit_event(NftStatsUpdated {
                nft_id,
                stats,
            });
        }

        #[ink(message, payable)]
        pub fn start_game(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_value();

            assert_eq!(value, 4_000_000_000, "Must pay 0.004 $AZERO fees to start the game");

            self.games.insert(&caller, &true);
        }

        #[ink(message)]
        pub fn check_game_status(&self, user: AccountId) -> bool {
            self.games.get(&user).unwrap_or(false)
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn end_game(&mut self, user: AccountId, mint_amount: Balance) -> Result<(), PSP22Error> {
            // Check if the user has an ongoing game
            let has_ongoing_game = self.check_game_status(user);
        
            // Assert that the user has an ongoing game
            assert!(has_ongoing_game, "User does not have an ongoing game");
        
            // End the game for the user
            self.games.insert(&user, &false);
        
            // Mint the specified amount of tokens
            psp22::Internal::_mint_to(self, user, mint_amount)?;
        
            Ok(())
        }
    }

    #[default_impl(PSP22Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self) {}
}
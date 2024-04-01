use openbrush::traits::{Balance, String};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub last_token_id: u64,
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance,
    pub max_amount: u64,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NFTError {
    BadMintValue,
    CannotMintZeroTokens,
    CollectionIsFull,
    TooManyTokensToMint,
    WithdrawalFailed,
}

impl NFTError {
    pub fn as_str(&self) -> String {
        match self {
            NFTError::BadMintValue => String::from("BadMintValue"),
            NFTError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            NFTError::CollectionIsFull => String::from("CollectionIsFull"),
            NFTError::TooManyTokensToMint => String::from("TooManyTokensToMint"),
            NFTError::WithdrawalFailed => String::from("WithdrawalFailed"),
        }
    }
}

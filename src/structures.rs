use near_sdk::{
     AccountId
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
pub type TokenId = String;
//structure for whitelist information
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddNewProfile {
    pub username:AccountId,
    pub media:String,
    pub _media_banner:Option<String>,
    pub biography:String,
    pub social_media:String,
    
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddTokenToCollection {
    pub contract_id: AccountId,
    pub owner_id: AccountId,
    pub token_id: TokenId,
    pub price:u128,
    pub title:String,
    pub description:String,
    pub media:String,
    pub creator:AccountId,
    pub collection_id:u64,
    pub approval_id:String,
    
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddNewUserCollection {
    pub owner_id:AccountId,
    pub title:String,
    pub description:String,
    pub media_icon:String,
    pub media_banner:String,
    pub visibility:bool,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub _id:String,
    pub current_collection_id:u64,
    
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NewCollection {
    
    pub contract_id: AccountId,
    pub owner_id: AccountId,
    pub token_id:TokenId,
    pub price: String,
    pub title:String,
    pub description: String,
    pub media: String,
    pub creator:AccountId,
    pub approval_id:String,
    pub collection_id:String,

}
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, Gas, PanicOnDefault,
    Promise, PromiseResult, CryptoHash, BorshStorageKey,serde_json::json,
};
use std::collections::HashMap;

use crate::external::*;
use crate::internal::*;
use crate::sale::*;
use near_sdk::env::STORAGE_PRICE_PER_BYTE;
pub use crate::migrate::*;
pub use crate::events::*;

pub use crate::dao::*;
pub use crate::offers::*;
pub use crate::offer_views::*;
mod external;
mod internal;
mod nft_callbacks;
mod sale;
mod sale_views;
mod migrate;
mod events;
mod dao;
mod offers;
mod offer_views;


//GAS constants to attach to calls
const GAS_FOR_ROYALTIES: Gas = Gas(115_000_000_000_000);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);
 

//constant used to attach 0 NEAR to a call
const NO_DEPOSIT: Balance = 0;

//the minimum storage to have a sale on the contract.
const STORAGE_PER_SALE: u128 = 100 * STORAGE_PRICE_PER_BYTE;

//every sale will have a unique ID which is `CONTRACT + DELIMITER + TOKEN_ID`
static DELIMETER: &str = ".";

//Creating custom types to use within the contract. This makes things more readable. 
pub type SalePriceInYoctoNear = U128;
pub type TokenId = String;
pub type FungibleTokenId = AccountId;
pub type ContractAndTokenId = String;
//defines the payout type we'll be parsing from the NFT contract as a part of the royalty standard.
#[derive(Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
} 


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferOutMarket {
    pub buyer_id: AccountId,
    pub nft_contract_id: AccountId,
    pub token_id: Option<TokenId>,
    pub ft_token_id: AccountId, // "near" for NEAR token
    pub price: u128,
}



//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_nft)]
pub trait ExternsContract {
  
    fn nft_supply_for_owner (& self,account_id:AccountId) -> String;
    fn nft_tokens_for_owner (& self,account_id:AccountId,from_index:String,limit:u8) ;

    fn nft_last_token_for_creator(& self,account_id:AccountId) -> String;


    fn get_owner_supply(&self,contract_id: AccountId,
        owner_id: AccountId,
       
        collection_id:u64) -> String;

    fn get_owner_last_token(&self,_contract_id: AccountId,
        _title:String,
        _description:String,
        _media:String,
        _collection_id:String) -> String;
        
 }



#[derive(Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
    //creator of the token
    pub creator_id: Option <AccountId>,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    
    pub royalty: Option< HashMap<AccountId, u32> >,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<String>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<String>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<String>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}
 
//main contract struct to store all the information
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //keep track of the owner of the contract
    pub owner_id: AccountId,
    pub treasure_id: AccountId,
    /*
        to keep track of the sales, we map the ContractAndTokenId to a Sale. 
        the ContractAndTokenId is the unique identifier for every sale. It is made
        up of the `contract ID + DELIMITER + token ID`
    */
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,
    //keep track of all the Sale IDs for every account ID
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,
    pub offers_by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,
    pub offers_by_bidder_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,

    //keep track of all the token IDs for sale for a given contract
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub offers_by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keep track of the storage that accounts have payed
    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub fee_percent :f64,
    pub whitelist_contracts: LookupMap<AccountId, ExternalContract>,
    pub offers: UnorderedMap<ContractAndTokenId, Offers>,
    pub ntv_multiplier:u128,

    pub is_mining_ntv_enabled: bool,
    pub collection_id:u64,
    pub market_account : String,
    pub ntvtoken_contract:  String,

}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
 pub struct OldContract {
    //keep track of the owner of the contract
    pub owner_id: AccountId,
    pub treasure_id: AccountId,
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,
    pub offers_by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,
    pub offers_by_bidder_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,

    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub offers_by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,

    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub fee_percent :f64,
    pub whitelist_contracts: LookupMap<AccountId, ExternalContract>,
    pub offers: UnorderedMap<ContractAndTokenId, Offers>,
    pub ntv_multiplier:u128,


    pub is_mining_ntv_enabled: bool,
    pub collection_id:u64,
    pub market_account : String,
    pub ntvtoken_contract:  String,
 

}

//structure for whitelist information
#[derive(BorshDeserialize, BorshSerialize,Clone)]
pub struct ExternalContract {
    register_address: AccountId,
    contract_name: String,
    contract_balance:u128,
}
/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByOwnerIdInner { account_id_hash: CryptoHash },
    ByOffersOwnerId,
    ByOffersOwnerIdInner { account_id_hash: CryptoHash },
    ByOffersBidderId,
    ByOffersBidderIdInner { account_id_hash: CryptoHash },
    ByNFTContractId,
    ByNFTContractIdInner { account_id_hash: CryptoHash },
    ByOffersNFTContractId,
    ByOffersNFTContractIdInner { account_id_hash: CryptoHash },

    ByNFTTokenType,
    ByNFTTokenTypeInner { token_type_hash: CryptoHash },
    FTTokenIds,
    StorageDeposits,
    ContractAllowed,
    OffersOutMarket
}

#[near_bindgen]
impl Contract {
     #![allow(dead_code, irrefutable_let_patterns)]
    /*

        initialization function (can only be called once).
        this initializes the contract with default data and the owner ID
        that's passed in
    */
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            //set the owner_id field equal to the passed in owner_id. 
            owner_id:owner_id.clone(),
            treasure_id:owner_id,
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            sales: UnorderedMap::new(StorageKey::Sales),
            by_owner_id: LookupMap::new(StorageKey::ByOwnerId),
            offers_by_owner_id: LookupMap::new(StorageKey::ByOffersOwnerId),
            offers_by_bidder_id: LookupMap::new(StorageKey::ByOffersBidderId),

            by_nft_contract_id: LookupMap::new(StorageKey::ByNFTContractId),
            offers_by_nft_contract_id: LookupMap::new(StorageKey::ByOffersNFTContractId),

            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
            fee_percent:0.03,
            whitelist_contracts: LookupMap::new(StorageKey::ContractAllowed),
            offers: UnorderedMap::new(StorageKey::OffersOutMarket),
            ntv_multiplier:3,
            is_mining_ntv_enabled:true,
            collection_id:0,
            market_account :"nativo-mkt.near".to_string(),
            ntvtoken_contract:"nativo-token.near".to_string(),

        };

        //return the Contract object
        this
    }

    //Allows users to deposit storage. This is to cover the cost of storing sale objects on the contract
    //Optional account ID is to users can pay for storage for other people.
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        //get the account ID to pay for storage for
        let storage_account_id = account_id 
            //convert the valid account ID into an account ID
            .map(|a| a.into())
            //if we didn't specify an account ID, we simply use the caller of the function
            .unwrap_or_else(env::predecessor_account_id);

        //get the deposit value which is how much the user wants to add to their storage
        let deposit = env::attached_deposit();

        //make sure the deposit is greater than or equal to the minimum storage for a sale
        assert!(
            deposit >= STORAGE_PER_SALE,
            "Requires minimum deposit of {}",
            STORAGE_PER_SALE
        );

        //get the balance of the account (if the account isn't in the map we default to a balance of 0)
        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        //add the deposit to their balance
        balance += deposit;
        //insert the balance back into the map for that account ID
        self.storage_deposits.insert(&storage_account_id, &balance);
    }

    
    fn internal_storage_deposit(&mut self, account_id: Option<AccountId>) {
        
        //get the account ID to pay for storage for
        let storage_account_id = account_id 
            //convert the valid account ID into an account ID
            .map(|a| a.into())
            //if we didn't specify an account ID, we simply use the caller of the function
            .unwrap_or_else(env::predecessor_account_id);

        //get the deposit value which is how much the user wants to add to their storage
        let deposit = env::attached_deposit();
         
        //make sure the deposit is greater than or equal to the minimum storage for a sale
        assert!(
            deposit >= STORAGE_PER_SALE,
            "Requires minimum deposit of {}",
            STORAGE_PER_SALE
        );

        //get the balance of the account (if the account isn't in the map we default to a balance of 0)
        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        //add the deposit to their balance
        balance += deposit;
        //insert the balance back into the map for that account ID
        
        self.storage_deposits.insert(&storage_account_id, &balance);
    }

    //Allows users to withdraw any excess storage that they're not using. Say Bob pays 0.01N for 1 sale
    //Alice then buys Bob's token. This means bob has paid 0.01N for a sale that's no longer on the marketplace
    //Bob could then withdraw this 0.01N back into his account. 
    #[payable]
    pub fn storage_withdraw(&mut self) {
        //make sure the user attaches exactly 1 yoctoNEAR for security purposes.
        //this will redirect them to the NEAR wallet (or requires a full access key). 
        assert_one_yocto();

        //the account to withdraw storage to is always the function caller
        let owner_id = env::predecessor_account_id();
        //get the amount that the user has by removing them from the map. If they're not in the map, default to 0
        let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);
        
        //how many sales is that user taking up currently. This returns a set
        let sales = self.by_owner_id.get(&owner_id);
        //get the length of that set. 
        let len = sales.map(|s| s.len()).unwrap_or_default();
        //how much NEAR is being used up for all the current sales on the account 
        let diff = u128::from(len) * STORAGE_PER_SALE;

        //the excess to withdraw is the total storage paid - storage being used up.
        amount -= diff;

        //if that excess to withdraw is > 0, we transfer the amount to the user.
        if amount > 0 {
            Promise::new(owner_id.clone()).transfer(amount);
        }
        //we need to add back the storage being used up into the map if it's greater than 0.
        //this is so that if the user had 500 sales on the market, we insert that value here so
        //if those sales get taken down, the user can then go and withdraw 500 sales worth of storage.
        if diff > 0 {
            self.storage_deposits.insert(&owner_id, &diff);
        }
    }

    /// views
    //return the minimum storage for 1 sale
    pub fn storage_minimum_balance(&self) -> U128 {
        U128(STORAGE_PER_SALE)
    }

    //return how much storage an account has paid for
    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.storage_deposits.get(&account_id).unwrap_or(0))
    }

    pub fn stop_play_ntv_minting(&mut self) -> String {
         self.is_the_owner();
         self.is_mining_ntv_enabled= !self.is_mining_ntv_enabled;
         self.is_mining_ntv_enabled.to_string()
    }

    pub fn ntv_is_minting(& self) -> String {
       return  self.is_mining_ntv_enabled.to_string();
   }


    // create a new creator perfil to

    pub fn add_new_profile(& self,
        username:AccountId,
        media:String,
        biography:String,
        social_media:String,
        _type:String,
       ){
        assert!(username.clone().to_string() != "","the username is null ");
        assert!(media.clone().to_string() != "","the media is null ");
        assert!(biography.clone().to_string() != "","the biography is null ");
        assert!(social_media.clone().to_string() != "","the social_media is null ");

        assert!(username.clone() == env::signer_account_id(),"the caller must be the same as the username sended");
           //this method just receive the info and throws a json log that will be readed by the graph
                env::log_str(
                    &json!({
                    "type": _type,
                    "params": {
                        "username": username,
                        "media": media,
                        "biography": biography,
                        "social_media": social_media,
                    
                    }
                })
                        .to_string(),
                );
    
       }

    #[payable]
    pub fn add_token_to_collection(&mut self, 
           contract_id: AccountId,
           owner_id: AccountId,
           token_id: TokenId,
           price:u128,
           title:String,
           description:String,
           media:String,
           creator:AccountId,
           collection_id:u64) {
               assert_one_yocto();
   
               assert!(contract_id.clone().to_string() != "","the contract_id is null ");
               assert!(owner_id.clone().to_string() != "","the owner_id is null ");
               assert!(token_id.clone().to_string() != "","the token_id is null ");
               assert!(price.clone().to_string() != "","the price is null ");
               assert!(title.clone().to_string() != "","the title is null ");
               assert!(description.clone().to_string() != "","the description is null ");
               assert!(media.clone().to_string() != "","the media is null ");
               assert!(creator.clone().to_string() != "","the creator is null ");
               assert!(collection_id.clone().to_string() != "","the collection_id is null ");
   
           assert!(creator.clone() == env::signer_account_id(),"the caller must be the same as the creator sended");
   
           env::log_str(
               &json!({
               "type": "new_collection",
               "params": {
                   "contract_id": contract_id,
                   "owner_id": owner_id,
                   "token_id":token_id,
                   "price": price.to_string(),
                   "title":title,
                   "description": description,
                   "media": media,
                   "creator":creator,
                   "approval_id":"0",
                   "collection_id":collection_id,
               }
           })
                   .to_string(),
           );
   
   
   
       }
      

   // #[payable]
    pub fn add_token_to_collection_xcc(&mut self, contract_id: AccountId,title:String,description:String,media:String,collection_id:String) {
        //asserting values from the caller       
     //   assert_one_yocto();   
        assert!(contract_id.clone().to_string() != "","the contract_id is null ");
        assert!(title.clone().to_string() != "","the title is null ");
        assert!(description.clone().to_string() != "","the description is null ");
        assert!(media.clone().to_string() != "","the media is null ");
        assert!(collection_id.clone().to_string() != "","the collection_id is null ");
        //Making a XCC to the minter to get the last token id
        ext_nft::nft_last_token_for_creator(env::signer_account_id(),contract_id.clone(),0,Gas(15_000_000_000_000),) 
        //resilving the result from the minter
        .then(ext_nft::get_owner_last_token(contract_id,title,description,media,collection_id,env::current_account_id(),0,Gas(15_000_000_000_000),));
       }
      
      
    #[payable]
    pub fn add_new_user_collection(&mut self,
           title:String,
           description:String,
           media_icon:String,
           media_banner:String,
           visibility:bool,
           twitter: String,
           website: String,
           _type:String,
           _id:String){
               assert_one_yocto();
   
               let owner_id = env::signer_account_id();
               let current_collection_id= self.collection_id;
   
               
               assert!(title.clone().to_string() != "","the title is null ");
               assert!(description.clone().to_string() != "","the description is null ");
               assert!(media_icon.clone().to_string()!= "","the media_icon is null ");
               assert!(media_banner.clone().to_string() != "","the media_banner is null ");
               assert!(twitter.clone().to_string() != "","the twitter is null ");
               assert!(website.clone().to_string() != "","the website is null ");
               
                if _type == "create" {
                    env::log_str(
                        &json!({
                        "type": _type,
                        "params": {                   
                            "owner_id": owner_id,
                            "title":title,
                            "description":description,
                            "media_icon": media_icon,
                            "media_banner": media_banner,
                            "twitter": twitter,
                            "website": website,
                            "collection_id":current_collection_id,
                            "visibility":visibility,
                                 }
                         }).to_string(),
                    );
                    
                    self.collection_id+=1;

                }else if _type =="edit"{
                     env::log_str(
                        &json!({
                        "type": _type,
                        "params": {                   
                            "owner_id": owner_id,
                            "title":title,
                            "description":description,
                            "media_icon": media_icon,
                            "media_banner": media_banner,
                            "twitter": twitter,
                            "website": website,
                            "collection_id":_id.parse::<u64>().unwrap() ,
                            "visibility":visibility,
                                 }
                         }).to_string(),
                    );
                }
              
   
              
           
       }



      //get the information for a specific token ID
   pub fn get_offer_id(&self, 
    nft_contract_id: AccountId,
    token_id: TokenId,) -> Option< Vec<String>  >{
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let offers = self.offers.get(&contract_and_token_id).expect("No offers");

        //if there is some auction ID in the auctions_by_id collection
        if let offer = self.offers_by_owner_id.get(&offers.owner_id).unwrap() {
            //we'll return the data for that auction
            Some(offer.to_vec())
        } else { //if there wasn't a auction ID in the auctions_by_id collection, we return None
            None
        }
    }

       //get the information for a specific token ID
   pub fn get_offer_bidder_id(&self, 
    nft_contract_id: AccountId,
    token_id: TokenId,) -> Option< Vec<String>  >{
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let offers = self.offers.get(&contract_and_token_id).expect("No offers");

        //if there is some auction ID in the auctions_by_id collection
        if let offer = self.offers_by_owner_id.get(&offers.owner_id).unwrap() {
            //we'll return the data for that auction
            Some(offer.to_vec())
        } else { //if there wasn't a auction ID in the auctions_by_id collection, we return None
            None
        }
    }


  

// Método de procesamiento para promesa
pub fn get_owner_last_token(&mut self ,_contract_id: AccountId,_title:String,_description:String,_media:String,_collection_id:String) {
         
    assert_eq!(env::promise_results_count(),1,"This is a callbacl module");
    
    match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Failed => { env::log_str( &"the external contract failed".to_string());  }
        PromiseResult::Successful(result) => {
            let value = std::str::from_utf8(&result).unwrap();
            let last_token_id: String = near_sdk::serde_json::from_str(&value).unwrap();
 
           env::log_str(
            &json!({
            "type": "new_collection",
            "params": {
                "contract_id": _contract_id,
                "owner_id": env::signer_account_id(),
                "token_id":last_token_id,
                "price": "0".to_string(),
                "title":_title,
                "description": _description,
                "media": _media,
                "creator":env::signer_account_id(),
                "approval_id":"0",
                "collection_id":_collection_id,
            }
        }).to_string(),);
            
        }
    }
}

  



//··········











}

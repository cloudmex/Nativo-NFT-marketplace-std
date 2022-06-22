use crate::*;

use near_sdk::json_types::Base64VecU8;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
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
    pub creator_id: AccountId,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    pub royalty: HashMap<AccountId, u32>,
}

#[ext_contract(ext_nft)]
trait NonFungibleTokenApprovalsReceiver {
    fn nft_token(& self,token_id: TokenId);

}
#[ext_contract(this_contract)]
trait Callbacks {
  fn get_pool_information_callback(&mut self);  
}


#[ext_contract(ext_self)]
pub trait MyContract {
    fn get_promise_result(&self,nft_contract_id :String ) -> String;

}
#[near_bindgen]
impl Contract {
    /// views


    // //returns the number of offers the marketplace has up (as a string)
    // pub fn get_supply_offers(
    //     &self,
    // ) -> U64 {
    //     //returns the offers object length wrapped as a U64
    //     U64(self.offers.len())
    // }

    //returns the number of offers for a given account (result is a string)
    pub fn get_supply_offers_by_owner_id(
        &self,
        account_id: AccountId,
    ) -> U64 {
        //get the set of offers for the given owner Id
        let offers_by_owner_id = self.offers_by_owner_id.get(&account_id);

        //if there as some set, we return the length but if there wasn't a set, we return 0
        if let Some(offers_by_owner_id) = offers_by_owner_id {
            U64(offers_by_owner_id.len())
        } else {
            U64(0)
        }
    }
    //returns the number of offers for a given account (result is a string)
    pub fn get_supply_offers_by_bidder_id(
        &self,
        account_id: AccountId,
    ) -> U64 {
        //get the set of offers for the given bidder Id
        let offers_by_bidder_id = self.offers_by_bidder_id.get(&account_id);

        //if there as some set, we return the length but if there wasn't a set, we return 0
        if let Some(offers_by_bidder_id) = offers_by_bidder_id {
            U64(offers_by_bidder_id.len())
        } else {
            U64(0)
        }
    }




    //returns paginated offers_ objects for a given account. (result is a vector of offers_)
    pub fn get_offers_by_owner_id(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Offers> {
        //get the set of token IDs for offer for the given account ID
        let offers_by_owner_id = self.offers_by_owner_id.get(&account_id);
        //if there was some set, we set the offer variable equal to that set. If there wasn't, offers is set to an empty vector
        let offers = if let Some(offers_by_owner_id) = offers_by_owner_id {
            offers_by_owner_id
        } else {
            return vec![];
        };

        //we'll convert the UnorderedSet into a vector of strings
        let keys = offers.as_vector();

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        keys.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 0
            .take(limit.unwrap_or(0) as usize) 
            //we'll map the token IDs which are strings into offer objects
            .filter_map(|token_id|  if self.offers.get(&token_id).unwrap_or(Offers {
                token_id: "null".to_string(),
                nft_contract_id: "null".to_string(),
                owner_id: "null".to_string().try_into().unwrap(),
                buyer_id: "null".to_string().try_into().unwrap(),
                approval_id: 0 as u64,
                price: 0.into(),
                ft_token_id:Some("null".parse::<AccountId>().unwrap()),
            
            }).owner_id== account_id {
                Some(self.offers.get(&token_id).unwrap())
            }else{
                None
            }
        )
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
    ////////////////////////////////
      //returns paginated offers_ objects for a given account. (result is a vector of offers_)
      pub fn get_offers_by_bidder_id(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Offers> {
        //get the set of token IDs for offer for the given account ID
        let offers_by_bidder_id = self.offers_by_bidder_id.get(&account_id.clone());
        //if there was some set, we set the offer variable equal to that set. If there wasn't, offers is set to an empty vector
        let offers = if let Some(offers_by_bidder_id) = offers_by_bidder_id {
            offers_by_bidder_id
        } else {
            return vec![];
        };

        //we'll convert the UnorderedSet into a vector of strings
        let keys = offers.as_vector();

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        
        //iterate through the keys vector
  keys.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 0
            .take(limit.unwrap_or(0) as usize) 
            //we'll map the token IDs which are strings into offer objects
            .filter_map(|token_id|  if self.offers.get(&token_id).unwrap_or(Offers {
                token_id: "null".to_string(),
                nft_contract_id: "null".to_string(),
                owner_id: "null".to_string().try_into().unwrap(),
                buyer_id: "null".to_string().try_into().unwrap(),
                approval_id: 0 as u64,
                price: 0.into(),
                ft_token_id:Some("null".parse::<AccountId>().unwrap()),
            
            }).buyer_id== account_id {
                Some(self.offers.get(&token_id).unwrap())
            }else{
                None
            }
        )
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()

         
            
            //.iter().filter(|offer| offer.buyer_id==account_id).collect()
    }


    /// 

    //get the number of offers for an nft contract. (returns a string)
    pub fn get_supply_offers_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
    ) -> U64 {
        //get the set of tokens for associated with the given nft contract
        let offers_by_nft_contract_id = self.offers_by_nft_contract_id.get(&nft_contract_id);

        //if there was some set, return it's length. Otherwise return 0
        if let Some(offers_by_nft_contract_id) = offers_by_nft_contract_id {
            U64(offers_by_nft_contract_id.len())
        } else {
            U64(0)
        }
    }

    //returns paginated offer objects associated with a given nft contract. (result is a vector of offers)
    pub fn get_offers_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Offers> {
        //get the set of token IDs for offer for the given contract ID
        let offers_by_nft_contract_id = self.offers_by_nft_contract_id.get(&nft_contract_id.clone());

        //if there was some set, we set the offers variable equal to that set. If there wasn't, offers is set to an empty vector
        let offers = if let Some(offers_by_nft_contract_id) = offers_by_nft_contract_id {
            offers_by_nft_contract_id
        } else {
            return vec![];
        };

        //we'll convert the UnorderedSet into a vector of strings
        let keys = offers.as_vector();

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        keys.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 0
            .take(limit.unwrap_or(0) as usize) 
            //we'll map the token IDs which are strings into offer objects by passing in the unique offer ID (contract + DELIMITER + token ID)
            .filter_map(|token_id|  if self.offers.get(&token_id).unwrap_or(Offers {
                    token_id: "null".to_string(),
                    nft_contract_id: "null".to_string(),
                    owner_id: "null".to_string().try_into().unwrap(),
                    buyer_id: "null".to_string().try_into().unwrap(),
                    approval_id: 0 as u64,
                    price: 0.into(),
                    ft_token_id:Some("null".parse::<AccountId>().unwrap()),
                
                }).nft_contract_id== nft_contract_id.to_string() {
                    Some(self.offers.get(&token_id).unwrap())
                }else{
                None
            }
            )
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //get a offer information for a given unique offer ID (contract + DELIMITER + token ID)
    pub fn get_offer_by_contract(&self, nft_contract_token: ContractAndTokenId) -> Option<Offers> {
        //try and get the offer object for the given unique offer ID. Will return an option since
        //we're not guaranteed that the unique offer ID passed in will be valid.
        self.offers.get(&nft_contract_token)
    }

     //get a offer information for a given unique offer ID (contract + DELIMITER + token ID)
     pub fn get_offer_bidder(&self, bidder_id: AccountId) {
        //try and get the offer object for the given unique offer ID. Will return an option since
        //we're not guaranteed that the unique offer ID passed in will be valid.
        self.offers_by_owner_id.get(&bidder_id).expect("not found");
    }

    // pub fn get_owner22(&self, nft_contract_id: AccountId ,token_id:TokenId){
    //     let x =self.get_owner(nft_contract_id,token_id);
    //     env::log_str("this is the response:");
    //     env::log_str(x);
    // }








}
  
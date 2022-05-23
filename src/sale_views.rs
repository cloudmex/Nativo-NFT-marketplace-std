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
    
    
    //returns the number of sales the marketplace has up (as a string)
    pub fn get_supply_sales(
        &self,
    ) -> U64 {
        //returns the sales object length wrapped as a U64
        U64(self.sales.len())
    }
    
    //returns the number of sales for a given account (result is a string)
    pub fn get_supply_by_owner_id(
        &self,
        account_id: AccountId,
    ) -> U64 {
        //get the set of sales for the given owner Id
        let by_owner_id = self.by_owner_id.get(&account_id);
        
        //if there as some set, we return the length but if there wasn't a set, we return 0
        if let Some(by_owner_id) = by_owner_id {
            U64(by_owner_id.len())
        } else {
            U64(0)
        }
    }

    //returns paginated sale objects for a given account. (result is a vector of sales)
    pub fn get_sales_by_owner_id(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Sale> {
        //get the set of token IDs for sale for the given account ID
        let by_owner_id = self.by_owner_id.get(&account_id);
        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        let sales = if let Some(by_owner_id) = by_owner_id {
            by_owner_id
        } else {
            return vec![];
        };
        
        //we'll convert the UnorderedSet into a vector of strings
        let keys = sales.as_vector();

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));
        
        //iterate through the keys vector
        keys.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 0
            .take(limit.unwrap_or(0) as usize) 
            //we'll map the token IDs which are strings into Sale objects
            .map(|token_id| self.sales.get(&token_id).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //get the number of sales for an nft contract. (returns a string)
    pub fn get_supply_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
    ) -> U64 {
        //get the set of tokens for associated with the given nft contract
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);
        
        //if there was some set, return it's length. Otherwise return 0
        if let Some(by_nft_contract_id) = by_nft_contract_id {
            U64(by_nft_contract_id.len())
        } else {
            U64(0)
        }
    }

    //returns paginated sale objects associated with a given nft contract. (result is a vector of sales)
    pub fn get_sales_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Sale> {
        //get the set of token IDs for sale for the given contract ID
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);
        
        //if there was some set, we set the sales variable equal to that set. If there wasn't, sales is set to an empty vector
        let sales = if let Some(by_nft_contract_id) = by_nft_contract_id {
            by_nft_contract_id
        } else {
            return vec![];
        };

        //we'll convert the UnorderedSet into a vector of strings
        let keys = sales.as_vector();

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));
        
        //iterate through the keys vector
        keys.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 0
            .take(limit.unwrap_or(0) as usize) 
            //we'll map the token IDs which are strings into Sale objects by passing in the unique sale ID (contract + DELIMITER + token ID)
            .map(|token_id| self.sales.get(&format!("{}{}{}", nft_contract_id, DELIMETER, token_id)).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //get a sale information for a given unique sale ID (contract + DELIMITER + token ID)
    pub fn get_sale(&self, nft_contract_token: ContractAndTokenId) -> Option<Sale> {
        //try and get the sale object for the given unique sale ID. Will return an option since
        //we're not guaranteed that the unique sale ID passed in will be valid.
        self.sales.get(&nft_contract_token)
    }

    // pub fn get_owner22(&self, nft_contract_id: AccountId ,token_id:TokenId){
    //     let x =self.get_owner(nft_contract_id,token_id);
    //     env::log_str("this is the response:");
    //     env::log_str(x);
    // }

    //get the owner from the minter and fins any sale o offer in the market to update
    //this will fix the error after exteral tranfer
    pub fn update_owner_from_minter(&self, nft_contract_id: AccountId ,token_id:TokenId)-> Promise{
        let token_info:JsonToken;
       
     let p= ext_nft::nft_token(
            token_id.clone(),
            nft_contract_id.clone(), //contract account we're calling
            NO_DEPOSIT, //NEAR deposit we attach to the call
            Gas(100_000_000_000_000), //GAS we're attaching
        ) 
        .then(ext_self::get_promise_result(
            format!("{}{}{}", nft_contract_id, DELIMETER, token_id),
            market_account.parse::<AccountId>().unwrap(), // el mismo contrato local
            NO_DEPOSIT,                                             // yocto NEAR a ajuntar al callback
            Gas(15_000_000_000_000),                            // gas a ajuntar al callback
        ));
       
        p
     
    }


   



     // Método de procesamiento para promesa
     pub fn get_promise_result(&mut self ,nft_contract_id :String) {
         
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::log_str( &"falló el contracto externo".to_string());
                
            }
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
                env::log_str("regreso al market");
                env::log_str(value);
                let tg: JsonToken = near_sdk::serde_json::from_str(&value).unwrap();
              
               // let tg: JsonToken = near_sdk::serde_json::from_str(&newstring).unwrap();  
                
                 tg.owner_id.to_string();
                 
                 let mut sale = None;
                 let mut offer=None;
                 sale= self.sales.get(&nft_contract_id);
                 offer =self.offers.get(&nft_contract_id);
                 if !sale.clone().is_none() {
                    env::log_str( &"on_sale".to_string());
                //     //Copy the sale infoº
                //     let mut lastsale=sale.unwrap();
                //     //Update the owner sale with the actual minter owner
                //     lastsale.owner_id=tg.clone().owner_id;
                //     //Save the changes 
                //    self.sales.insert(&nft_contract_id,&lastsale);

                   self.sale.remove(&nft_contract_id);
                 }
                
                if !offer.is_none() {
                        env::log_str( &"on_offer".to_string());
                        //if the minter owner is the same as the offer
                        if offer.clone().unwrap().buyer_id ==tg.clone().owner_id{
                            env::log_str( &"you are the token owner with a bid,so we refound you the bid".to_string());
                            //refound your bid to the owner and delete the offers
                            //refund
                            Promise::new(offer.clone().unwrap().buyer_id).transfer(offer.clone().unwrap().price.0);
                            //remove the bid
                            self.offers.remove(&nft_contract_id);
                        }else{
                            //Copy the offer info
                            let mut lastoffer=offer.unwrap();
                            //Update the owner sale with the actual minter owner
                            lastoffer.owner_id=tg.clone().owner_id;
                            //Save the changes 
                            self.offers.insert(&nft_contract_id,&lastoffer);
                            self.offers.remove(&nft_contract_id)
                        }
                }
                 else{
                    env::log_str(  &"no sale/offer found".to_string());
                 }
                
                // let offer =self.offers.get(&nft_contract_id).expect("no offer found");
                // offer.owner_id.to_string()
            }
        }
    }
}

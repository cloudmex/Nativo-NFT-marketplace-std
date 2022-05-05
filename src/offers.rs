use crate::*;
  
 
 
//struct that holds important information about each sale on the market
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Offers {
    //actual token ID for sale
    pub token_id: String,
    //nft contract where the token was minted
    pub nft_contract_id: String,
    //owner of the sale
    pub owner_id: AccountId,
    //owner of the sale
    pub buyer_id: AccountId,
    //market contract's approval ID to transfer the token on behalf of the owner
    pub approval_id: u64,
    //sale price in yoctoNEAR that the token is listed for
    pub price: SalePriceInYoctoNear,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ft_token_id: Option<AccountId>,

}

const NTVTOKEN_CONTRACT:  &str = "nativo_token.testnet";


#[near_bindgen]
impl Contract {

    #[payable]
    pub fn add_offer( 
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        owner_id:AccountId,
     )  {
         // the variables 
         let bidder_id= env::predecessor_account_id();
         let bid_amount = env::attached_deposit();

        // create the index
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);

        assert!(
            bidder_id.clone()!=owner_id.clone(),
            "You can not add a bid for your token"
        );
        assert!(
            bid_amount.clone()> 0,
            "The bid must be more than 0"
        );
        //1 ask if is listed on the sale structures.
        let mut market_data = None;
        market_data= self.sales.get(&contract_and_token_id);
        //if yes 
        if !market_data.is_none() {
            env::log_str("a sale was found");
            //get the deposit and compare with the sales price 
              //assert the deposit is lower than the sales price
                //add offer
                assert!(
                    bid_amount.clone()< u128::from(market_data.as_ref().unwrap().price),
                    "The new bid must be lower than the current sale price: {:?}",
                    u128::from(market_data.clone().as_ref().unwrap().price)
                );
                 //add a offer 
            env::log_str("add new  makert offer");

            //look if exists a previous offer lower than the actual 
           let mut prev_offer=None;
               prev_offer= self.offers.get(&contract_and_token_id.clone());
           //if exist 
           if !prev_offer.is_none() {
               env::log_str("we found a prev bid ");
               //aassert the amount is more than the actual bid
               assert!(
                   bid_amount.clone()> u128::from(prev_offer.as_ref().unwrap().price),
                   "The new bid must more than the current bid price: {:?}",
                   u128::from(prev_offer.clone().as_ref().unwrap().price)
               );
               //assert that the bidder isnot the previous one
               env::log_str(&bidder_id.clone().to_string());
               env::log_str(&prev_offer.as_ref().unwrap().buyer_id.to_string());

               assert!(
                   bidder_id.clone()!=prev_offer.as_ref().unwrap().buyer_id,
                   "You can not add a new bid having one active"
               );
                   //refound the bid to the bidder
                   Promise::new(prev_offer.clone().unwrap().buyer_id).transfer(u128::from(prev_offer.as_ref().unwrap().price));
                   //create a new offer structure
                   let newoffer = Offers {
                       token_id: token_id.clone() ,
                       nft_contract_id: nft_contract_id.clone().into() ,
                       owner_id:  owner_id.clone()  ,
                       buyer_id: bidder_id.clone() ,
                       approval_id: market_data.clone().as_ref().unwrap().approval_id ,
                       price:  bid_amount.into(),
                       ft_token_id: None,

                   };
                   //save the offer to the contract_
                   self.offers.insert(&contract_and_token_id.clone(),&newoffer);

                   env::log_str(
                       &json!({
                       "type": "place_a_non_empty_offer",
                       "params": newoffer 
                   })
                           .to_string(),
                   );

           }else{
               env::log_str("we havent found a prev bid ");

                //create a new offer structure
                let newoffer = Offers {
                   token_id: token_id.clone() ,
                   nft_contract_id: nft_contract_id.clone().into() ,
                   owner_id:  owner_id.clone()  ,
                   buyer_id: bidder_id.clone() ,
                   approval_id: market_data.clone().as_ref().unwrap().approval_id  ,
                   price:  bid_amount.into(),
                   ft_token_id: None,

               };
               //save the offer to the contract_
               self.offers.insert(&contract_and_token_id.clone(),&newoffer);

               env::log_str(
                   &json!({
                   "type": "place_a_empty_offer",
                   "params": newoffer 
               })
                       .to_string(),
               );

           }
        }
        else {//not
            //add a offer 
            env::log_str("add new no makert offer");

             //look if exists a previous offer lower than the actual 
            let mut prev_offer=None;
                prev_offer= self.offers.get(&contract_and_token_id.clone());
            //if exist 
            if !prev_offer.is_none() {
                env::log_str("we found a prev bid ");
                //aassert the amount is more than the actual bid
                assert!(
                    bid_amount.clone()> u128::from(prev_offer.as_ref().unwrap().price),
                    "The new bid must more than the current bid price: {:?}",
                    u128::from(prev_offer.clone().as_ref().unwrap().price)
                );
                //assert that the bidder isnot the previous one
                env::log_str(&bidder_id.clone().to_string());
                env::log_str(&prev_offer.as_ref().unwrap().buyer_id.to_string());

                assert!(
                    bidder_id.clone()!=prev_offer.as_ref().unwrap().buyer_id,
                    "You can not add a new bid having one active"
                );
                    //refound the bid to the bidder
                    Promise::new(prev_offer.clone().unwrap().buyer_id).transfer(u128::from(prev_offer.as_ref().unwrap().price));
                    //create a new offer structure
                    let newoffer = Offers {
                        token_id: token_id.clone() ,
                        nft_contract_id: nft_contract_id.clone().into() ,
                        owner_id:  owner_id.clone()  ,
                        buyer_id: bidder_id.clone() ,
                        approval_id: 0 ,
                        price:  bid_amount.into(),
                        ft_token_id: None,

                    };
                    //save the offer to the contract_
                    self.offers.insert(&contract_and_token_id.clone(),&newoffer);

                    env::log_str(
                        &json!({
                        "type": "place_a_non_empty_offer",
                        "params": newoffer 
                    })
                            .to_string(),
                    );

            }else{
                env::log_str("we havent found a prev bid ");

                 //create a new offer structure
                 let newoffer = Offers {
                    token_id: token_id.clone() ,
                    nft_contract_id: nft_contract_id.clone().into() ,
                    owner_id:  owner_id.clone()  ,
                    buyer_id: bidder_id.clone() ,
                    approval_id: 0 ,
                    price:  bid_amount.into(),
                    ft_token_id: None,

                };
                //save the offer to the contract_
                self.offers.insert(&contract_and_token_id.clone(),&newoffer);

                env::log_str(
                    &json!({
                    "type": "place_a_empty_offer",
                    "params": newoffer 
                })
                        .to_string(),
                );

            }

          

        }
             
    }
    


     //returns paginated sale objects associated with a given nft contract. (result is a vector of sales)
    pub fn get_offer(
        &self,
        nft_contract_id: AccountId,
        token_id:TokenId,
    ) -> Offers {
       
        self.offers.get(&format!("{}{}{}", nft_contract_id, DELIMETER, token_id)).expect("there is not an offer for this token")
     }
    #[payable]
    pub fn delete_offer(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        
    ) {
        assert_one_yocto();

   

        let caller = env::signer_account_id();
        let index = format!("{}{}{}", &nft_contract_id, DELIMETER, &token_id);
        let offer= self.offers.get(&index.clone()).expect("there is not an offer for this token");
        

        assert!(
            caller.clone() == offer.clone().owner_id || caller.clone() == offer.clone().buyer_id ,
             "You are not allowed  "
             );
     
          
       
        //refund
        Promise::new(offer.clone().buyer_id).transfer(offer.clone().price.0);
        //erase bid
        self.offers.remove(&index.clone());

        env::log_str(
            &json!({
                "type": "delete_offer",
                "params": {
                    "nft_contract_id": offer.clone().nft_contract_id,
                    "buyer_id": offer.clone().buyer_id,
                    "token_id": offer.clone().token_id,
                     
                }
            })
            .to_string(),
        );
    }
    #[payable]
    pub fn accept_offer(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        //get the actual offer nft information
        let mut market_data = self.offers.get(&contract_and_token_id).expect("The token does not exist");
     
        let caller = env::signer_account_id();
        let old_owner=market_data.clone().owner_id;
         
        assert!(
            caller.clone()==market_data.owner_id.clone(),
            "You can not accept the offer,only the owner"
        );
        
        //if the signer is the owner he can accept  
        if caller == market_data.clone().owner_id {
            // accept
            env::log_str("it's the owner");
             
                  //remove from the sales 
                     
                    self.offers.remove(&contract_and_token_id);
                  //and process the purchase
                      self.process_purchase(
                        AccountId::new_unchecked(market_data.clone().nft_contract_id),
                            token_id.clone(),
                            market_data.clone().price,  //   selected_bid.price.clone().0.into(),
                            market_data.clone().buyer_id,
                        );  
            
                        
                        env::log_str(
                            &json!({
                            "type": "process_bid",
                            "params": {
                                "old_owner_id": old_owner,
                                "new_owner_id": caller,
                                "nft_contract_id": nft_contract_id,
                                "token_id": token_id,
                                "price": market_data.clone().price,
                              
                            }
                        })
                                .to_string(),
                        );
          
            //pay the NTV 
            let tokens_to_mint = u128::from(market_data.clone().price) * 3 ;
                    // NTV for the buyer
                    ext_nft::mint(
                        market_data.clone().buyer_id,
                        tokens_to_mint.to_string(),
                        NTVTOKEN_CONTRACT.to_string().try_into().unwrap(),
                        0000000000000000000000001,
                        10_000_000_000_000.into(),
                    );
                     // NTV for the owner
                    ext_nft::mint(
                        market_data.clone().owner_id,
                        tokens_to_mint.to_string(),
                        NTVTOKEN_CONTRACT.to_string().try_into().unwrap(),
                        0000000000000000000000001,
                        10_000_000_000_000.into(),
                    );
           
           
   
        }

       
     }
    
}

//this is the cross contract call that we call on our own contract. 
/*
    private method used to resolve the promise when calling nft_transfer_payout. This will take the payout object and 
    check to see if it's authentic and there's no problems. If everything is fine, it will pay the accounts. If there's a problem,
    it will refund the buyer for the price. 
*/
 

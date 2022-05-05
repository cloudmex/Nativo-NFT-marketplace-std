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


#[near_bindgen]
impl Contract {

    #[payable]
    pub fn add_offer_out_market( 
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId
     )  {
         // the variables 
         let bidder_id= env::predecessor_account_id();
         let bid_amount = env::attached_deposit();

        // create the index
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        //1 ask if is listed on the sale structures.
        let mut market_data = None;
        market_data= self.sales.get(&contract_and_token_id);
        //if yes 
        if !market_data.is_none() {
            env::log_str("a sale was found");
            //get the deposit and compare with the sales price 
            // assert the deposit is lower than the sales price
                //add offer
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
                        owner_id:  nft_contract_id.clone()  ,
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
                    owner_id:  nft_contract_id.clone()  ,
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
    
    
}

//this is the cross contract call that we call on our own contract. 
/*
    private method used to resolve the promise when calling nft_transfer_payout. This will take the payout object and 
    check to see if it's authentic and there's no problems. If everything is fine, it will pay the accounts. If there's a problem,
    it will refund the buyer for the price. 
*/
 

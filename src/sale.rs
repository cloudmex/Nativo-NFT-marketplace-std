use crate::*;
use near_sdk::promise_result_as_success;
use std::convert::TryInto;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub bidder_id: AccountId,
    pub price: U128,
}
pub type Bids = Vec<Bid>;

const NTVTOKEN_CONTRACT:  &str = "nativo_token.testnet";
 

 
#[ext_contract(ext_nft)]
pub trait ExternsContract {
    fn mint(&self, account_id:AccountId,amount: String) -> String;
    fn reward_player(&self,player_owner_id: String,tokens_mint: String) -> String;
}
//struct that holds important information about each sale on the market
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]

pub struct Sale {
    //owner of the sale
    pub owner_id: AccountId,
    //market contract's approval ID to transfer the token on behalf of the owner
    pub approval_id: u64,
    //nft contract where the token was minted
    pub nft_contract_id: String,
    //actual token ID for sale
    pub token_id: String,
    //sale price in yoctoNEAR that the token is listed for
    pub price: SalePriceInYoctoNear,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ft_token_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<AccountId>, // offer
    
    
    pub is_auction: Option<bool>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<AccountId>,
    
    pub bids: Option<Bids>,
}


#[near_bindgen]
impl Contract {
    
    //removes a sale from the market. 
    #[payable]
    pub fn remove_sale(&mut self, nft_contract_id: AccountId, token_id: String) {
        //assert that the user has attached exactly 1 yoctoNEAR (for security reasons)
        assert_one_yocto();
        //get the sale object as the return value from removing the sale internally
        let sale = self.internal_remove_sale(nft_contract_id.clone().into(), token_id.clone());
        //get the predecessor of the call and make sure they're the owner of the sale
        let owner_id = env::predecessor_account_id();
        //if this fails, the remove sale will revert
        assert_eq!(owner_id, sale.owner_id, "Must be sale owner");

        env::log_str(
            &json!({
            "type": "remove_sale",
            "params": {
                "owner_id": owner_id.clone(),
                "nft_contract_id": nft_contract_id.clone(),
                "token_id": token_id.clone(),
               
              
            }
        })
                .to_string(),
        );
    }

    //updates the price for a sale on the market
    #[payable]
    pub fn update_price(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        price: U128,
    ) {
        //assert that the user has attached exactly 1 yoctoNEAR (for security reasons)
        assert_one_yocto();
        
        //create the unique sale ID from the nft contract and token
        let contract_id: AccountId = nft_contract_id.clone().into();
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        
        //get the sale object from the unique sale ID. If there is no token, panic. 
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");

        //assert that the caller of the function is the sale owner
        assert_eq!(
            env::predecessor_account_id(),
            sale.owner_id,
            "Must be sale owner"
        );
        
        //set the sale conditions equal to the passed in price
        sale.price = price;
        //insert the sale back into the map for the unique sale ID
        self.sales.insert(&contract_and_token_id, &sale.clone());


        env::log_str(
            &json!({
            "type": "update_price",
            "params": {
                "owner_id": sale.owner_id,
                "nft_contract_id": nft_contract_id,
                "token_id": token_id,
                "price": price,
              
            }
        })
                .to_string(),
        );
    }

    //place an offer on a specific sale. The sale will go through as long as your deposit is greater than or equal to the list price
    #[payable]
    pub fn offer(&mut self, nft_contract_id: AccountId, token_id: String) {
        //get the attached deposit and make sure it's greater than 0
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");
        
        //this is a new method that will recover the owner in the minter and update the sales and offers before anything transaction
        //self.update_owner_from_minter(nft_contract_id.clone(), token_id.clone());

        //if exist an offer found it Here
        let mut if_offer= self.get_offer(nft_contract_id.clone(),token_id.clone());

        //convert the nft_contract_id from a AccountId to an AccountId
        let contract_id: AccountId = nft_contract_id.clone().into();
        //get the unique sale ID (contract + DELIMITER + token ID)
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        
        //get the sale object from the unique sale ID. If the sale doesn't exist, panic.
        let sale = self.sales.get(&contract_and_token_id).expect("No sale");
        
        //get the buyer ID which is the person who called the function and make sure they're not the owner of the sale
        let buyer_id = env::predecessor_account_id();
        assert_ne!(sale.owner_id, buyer_id, "Cannot bid on your own sale.");
        
        //get the u128 price of the token (dot 0 converts from U128 to u128)
        let price = sale.price.0;

        //make sure the deposit is greater than the price
        assert!(deposit >= price, "Attached deposit must be greater than or equal to the current price: {:?}", price);

        // if the token have a bid update the owner
        if if_offer.token_id!="null".to_string() {
            //set the new owner
            if_offer.owner_id=buyer_id.clone();
            //save the new data
            self.offers.insert(&contract_and_token_id.clone(),&if_offer);    
            }
        //process the purchase (which will remove the sale, transfer and get the payout from the nft contract, and then distribute royalties) 
        self.process_purchase(
            contract_id.clone(),
            token_id.clone(),
            U128(deposit),
            buyer_id.clone(),
        );

        //go to the ntv ft to pay 
        //let stt ="Tokens a minar".to_string() + &(deposit.clone()*1000000000000000000000000).to_string();
        //
      
        if self.is_mining_ntv_enabled {

                    let tokens_to_mint = deposit.clone() * 3;
                    // NTV for the buyer
                    ext_nft::mint(
                        buyer_id.clone(),
                        tokens_to_mint.to_string(),
                        NTVTOKEN_CONTRACT.to_string().try_into().unwrap(),
                        0000000000000000000000001,
                        10_000_000_000_000.into(),
                    );
                     // NTV for the owner
                    ext_nft::mint(
                        sale.clone().owner_id,
                        tokens_to_mint.to_string(),
                        NTVTOKEN_CONTRACT.to_string().try_into().unwrap(),
                        0000000000000000000000001,
                        10_000_000_000_000.into(),
                    );

                }else{
                    env::log_str("the nvt token minting is disabled");      
                  }
        env::log_str(
            &json!({
            "type": "offer",
            "params": {
                "old_owner_id": sale.clone().owner_id,
                "new_owner_id": buyer_id,
                "nft_contract_id": contract_id,
                "token_id": token_id,
                "price": price.to_string(),
              
            }
        })
                .to_string(),
        );
    }

    //private function used when a sale is purchased. 
    //this will remove the sale, transfer and get the payout from the nft contract, and then distribute royalties
    #[private]
    pub fn process_purchase(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        price: U128,
        buyer_id: AccountId,
    ) -> Promise {
        //get the sale object by removing the sale
        let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());

        //pay the fee comision
        let amount_to_pay  = self.pay_the_market_fee(price);
       
        

        //initiate a cross contract call to the nft contract. This will transfer the token to the buyer and return
        //a payout object used for the market to distribute funds to the appropriate accounts.
        ext_contract::nft_transfer_payout(
            buyer_id.clone(), //purchaser (person to transfer the NFT to)
            token_id, //token ID to transfer
            sale.approval_id, //market contract's approval ID in order to transfer the token on behalf of the owner
            "payout from market".to_string(), //memo (to include some context)
            /*
                the price that the token was purchased for. This will be used in conjunction with the royalty percentages
                for the token in order to determine how much money should go to which account. 
            */
            amount_to_pay,
			10, //the maximum amount of accounts the market can payout at once (this is limited by GAS)
            nft_contract_id, //contract to initiate the cross contract call to
            1, //yoctoNEAR to attach to the call
            GAS_FOR_NFT_TRANSFER, //GAS to attach to the call
        )
        //after the transfer payout has been initiated, we resolve the promise by calling our own resolve_purchase function. 
        //resolve purchase will take the payout object returned from the nft_transfer_payout and actually pay the accounts
        .then(ext_self::resolve_purchase(
            buyer_id, //the buyer and price are passed in incase something goes wrong and we need to refund the buyer
            amount_to_pay,
            env::current_account_id(), //we are invoking this function on the current contract
            NO_DEPOSIT, //don't attach any deposit
            GAS_FOR_ROYALTIES, //GAS attached to the call to payout royalties
        ))
    }

    /*
        private method used to resolve the promise when calling nft_transfer_payout. This will take the payout object and 
        check to see if it's authentic and there's no problems. If everything is fine, it will pay the accounts. If there's a problem,
        it will refund the buyer for the price. 
    */
    #[private]
    pub fn resolve_purchase(
        &mut self,
        buyer_id: AccountId,
        price: U128,
    ) -> U128 {
        // checking for payout information returned from the nft_transfer_payout method
        let payout_option = promise_result_as_success().and_then(|value| {
            //if we set the payout_option to None, that means something went wrong and we should refund the buyer
            near_sdk::serde_json::from_slice::<Payout>(&value)
                //converts the result to an optional value
                .ok()
                //returns None if the none. Otherwise executes the following logic
                .and_then(|payout_object| {
                    //we'll check if length of the payout object is > 10 or it's empty. In either case, we return None
                    if payout_object.payout.len() > 10 || payout_object.payout.is_empty() {
                        env::log_str("Cannot have more than 10 royalties");
                        None
                    
                    //if the payout object is the correct length, we move forward
                    } else {
                        //we'll keep track of how much the nft contract wants us to payout. Starting at the full price payed by the buyer
                        let mut remainder = price.0;
                        
                        //loop through the payout and subtract the values from the remainder. 
                        for &value in payout_object.payout.values() {
                            //checked sub checks for overflow or any errors and returns None if there are problems
                            remainder = remainder.checked_sub(value.0)?;
                        }
                        //Check to see if the NFT contract sent back a faulty payout that requires us to pay more or too little. 
                        //The remainder will be 0 if the payout summed to the total price. The remainder will be 1 if the royalties
                        //we something like 3333 + 3333 + 3333. 
                        if remainder == 0 || remainder == 1 {
                            //set the payout_option to be the payout because nothing went wrong
                            Some(payout_object.payout)
                        } else {
                            //if the remainder was anything but 1 or 0, we return None
                            None
                        }
                    }
                })
        });

        // if the payout option was some payout, we set this payout variable equal to that some payout
        let payout = if let Some(payout_option) = payout_option {
            payout_option
        //if the payout option was None, we refund the buyer for the price they payed and return
        } else {
            Promise::new(buyer_id).transfer(u128::from(price));
            // leave function and return the price that was refunded
            return price;
        };

        // NEAR payouts
        for (receiver_id, amount) in payout {
            Promise::new(receiver_id).transfer(amount.0);
        }

        //return the price payout out
        price
    }

    #[private]
    pub fn pay_the_market_fee(&self,price:U128) -> U128 {
        //send the comision to the treasury
        let newprice: u128  = u128::try_from(price).unwrap();
        let commision = newprice as f64 * self.fee_percent;
        env::log_str("comision");
        env::log_str(&commision.to_string());
        let comisionu128 = commision as u128;
        env::log_str("comision to pay");
        env::log_str(&comisionu128.to_string());

        Promise::new(self.treasure_id.clone()).transfer(commision as u128);

           
       env::log_str("payment without comisiion");

        let comision_payed = newprice   - commision as u128;
        env::log_str(&comision_payed.clone().to_string());
       
 
        let newprice_lesscomision: U128  = U128::try_from(comision_payed).unwrap();
         

 
        return newprice_lesscomision;

       }
 
     /* pub fn add_bid(&self,nft_contract_id: AccountId,token_id: AccountId) {
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let mut actual_sale_info = self.sales.get(&contract_and_token_id.clone()).expect("Nativo: the token id does not exist");
                 //if dont exist throws a expect err
                 
        let bidder_address = env::signer_account_id();
        let bid_amount = env::attached_deposit();
        let price_u128=  u128::try_from(actual_sale_info.clone().price).unwrap();

       let new_bid= Bid {
              bidder_id: bidder_address ,
              price: bid_amount.into()   
        };
        let mut bids = actual_sale_info.bids.unwrap_or(Vec::new());

        if !bids.is_empty() {
            let current_bid = &bids[bids.len() - 1];

            assert!(
                bid_amount > current_bid.price.0,
                "The new bid must more than the current bid price: {:?}",
                current_bid.price
            );

            assert!(
                bid_amount > price_u128,
                "The new bid must to be more o equal to the base bid price: {:?}",
                U128(price_u128)
            );

            // refund
            Promise::new(current_bid.bidder_id.clone()).transfer(current_bid.price.0);

            // always keep 1 bid for now
            bids.remove(bids.len() - 1);
        } else {
            assert!(
                bid_amount > price_u128,
                "Paras: Can't pay less than or equal to starting price: {}",
                price_u128
            );

        }

        bids.push(new_bid);
        actual_sale_info.bids = Some(bids);
        self.sales.insert(&contract_and_token_id, &actual_sale_info);

      
        
       }*/

        // Auction bids
    #[private]
    #[payable]
    pub fn add_bid(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
         
    ) {
        

        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let mut market_data = self
            .sales
            .get(&contract_and_token_id)
            .expect("The token does not exist");

            Some(market_data.clone()).expect("The token does not exist or isnt listed");

         
        let bidder_id = env::predecessor_account_id();
        let bid_amount = env::attached_deposit();
        let price_u128=  u128::try_from(market_data.price).unwrap();

        assert!(
            bidder_id != market_data.owner_id,
            "{} you can not bid your own nft",
            bidder_id
        );
        market_data.is_auction.expect("the token does not accept a bid");
        // the new bid info
        let new_bid = Bid {
            bidder_id: bidder_id.clone(),
            price: bid_amount.into(),
        };
        // get the bid vec void or not
        let mut bids = market_data.bids.unwrap_or(Vec::new());
        //to add a new bid we must to verify if the token was listed as sale or auction
        //if the token was listed as sale we can add a new bid less o equals to the sale price
        if market_data.is_auction== Some(false){ //this is a sale
            env::log_str("was listed as sale");

            //assert that the bid is less than the sale price
            assert!(
                bid_amount < market_data.price.0,
                "You must to pay less than the sale price: {:?}",
                market_data.price.0
            );


            if !bids.is_empty() {
                let current_bid = &bids[bids.len() - 1];
    
                assert!(
                    bid_amount > current_bid.price.0,
                    "You must to pay at more or at least equal to current bid price: {:?}",
                    current_bid.price
                );
    
                /* assert!(
                    bid_amount > price_u128,
                    "you must to pay at least the starting price: {:?}",
                    U128(price_u128)
                ); */
    
                // refund
                Promise::new(current_bid.bidder_id.clone()).transfer(current_bid.price.0);
    
                // always keep 1 bid for now
                bids.remove(bids.len() - 1);
            } else {
                assert!(
                    bid_amount > 1,
                    "you must to pay at least one yocto: {:?}",
                    U128(price_u128)
                );
    
            }

            bids.push(new_bid.clone());
            market_data.bids = Some(bids);
            self.sales.insert(&contract_and_token_id, &market_data.clone());

            env::log_str(
                &json!({
                "type": "add_bid",
                "params": {
                    "owner_id": market_data.owner_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                    "bidder_id": new_bid.bidder_id ,
                    "bid_amount":new_bid.price
                  
                }
            })
                    .to_string(),
            );
    
        }//else the token as auction the bid must to be more than  start price
        else{
            env::log_str("was listed as auction");
            //assert that the bid is less than the sale price
            assert!(
                bid_amount > market_data.price.0,
                "You must to pay more than the auction price: {:?}",
                market_data.price.0
            );


            if !bids.is_empty() {
                let current_bid = &bids[bids.len() - 1];
    
                assert!(
                    bid_amount > current_bid.price.0,
                    "You must to pay at more or at least equal to current bid price: {:?}",
                    current_bid.price
                );
    
                /* assert!(
                    bid_amount > price_u128,
                    "you must to pay at least the starting price: {:?}",
                    U128(price_u128)
                ); */
    
                // refund
                Promise::new(current_bid.bidder_id.clone()).transfer(current_bid.price.0);
    
                // always keep 1 bid for now
                bids.remove(bids.len() - 1);
            } else {
                assert!(
                    bid_amount > market_data.price.0,
                    "2_You must to pay more than the auction price: {:?}",
                    market_data.price.0
                );
    
            }

            bids.push(new_bid.clone());
            market_data.bids = Some(bids);
            self.sales.insert(&contract_and_token_id, &market_data);
            env::log_str(
                &json!({
                "type": "add_bid",
                "params": {
                    "owner_id": market_data.owner_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                    "bidder_id": new_bid.bidder_id ,
                    "bid_amount":new_bid.price
                  
                }
            })
                    .to_string(),
            );
    
        }
        

        

       

       
        
    }
    #[private]
    #[payable]
    pub fn process_bid(&mut self, nft_contract_id: AccountId, token_id: TokenId,response:bool) {
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        //get the actual nft information
        let mut market_data = self.sales.get(&contract_and_token_id).expect("The token does not exist");
     

        /*  Some(market_data.clone()).expect("The token does not exist or isnt listed"); */

        let caller = env::signer_account_id();
        let old_owner=market_data.clone().owner_id;
        // get the bid vec void or not
        let mut bids = market_data.clone().bids.unwrap_or(Vec::new());
        // keep the bid vec 
        let current_bid = &bids[bids.len() - 1];

        //if the signer is the owner he can accept or decline
        if caller == market_data.clone().owner_id {
            // accept
            env::log_str("it's the owner");
            if response==true {
                  //remove from the sales 
                    let selected_bid = bids.clone().remove(&bids.len() - 1);
                    market_data.bids = Some(bids.clone());
                    self.sales.insert(&contract_and_token_id, &market_data.clone());
                  //and process the purchase
                      self.process_purchase(
                        AccountId::new_unchecked(market_data.clone().nft_contract_id),
                            token_id.clone(),
                            selected_bid.price.clone().0.into(),
                            selected_bid.bidder_id.clone(),
                        );  
            
                        
                        env::log_str(
                            &json!({
                            "type": "process_bid",
                            "params": {
                                "old_owner_id": old_owner,
                                "new_owner_id": caller,
                                "nft_contract_id": nft_contract_id,
                                "token_id": token_id,
                                "price": selected_bid.price.clone().0.to_string(),
                              
                            }
                        })
                                .to_string(),
                        );
            }//decline
            else {
                env::log_str("decline the offer");
                //remove from the sales 
                
                    //let selected_bid = bids.clone().remove(&bids.len() - 1);
                    market_data.bids = Some(Vec::new());
                    self.sales.insert(&contract_and_token_id, &market_data.clone());
            
                //refound the bid to the bidder
                Promise::new(current_bid.bidder_id.clone()).transfer(current_bid.price.0);
    
            }
           
           
   
        }else{
            env::log_str("may its the bidder");
            // if the signer is the bidder he just can decline
          if !bids.is_empty(){
            let current_bid = &bids[bids.len() - 1];
            if caller == current_bid.bidder_id{
                env::log_str("decline the offer");

                //accept
                if response==true {
                    assert!(
                        response==true,
                        "You can not accept the offer"
                    );
                      
                }//decline
                else{
                   
                    //remove from the sales 
                         
                        market_data.bids = Some(Vec::new());
                        self.sales.insert(&contract_and_token_id, &market_data.clone());

                    //refound the bid to the bidder
                        Promise::new(current_bid.bidder_id.clone()).transfer(current_bid.price.0);
 
                }
                
            }else{
                 
                assert!(
                    market_data.owner_id ==caller || current_bid.clone().bidder_id ==caller,
                    "you're not the owner or bidder  {}",
                    contract_and_token_id 
                );
                
                 
            }
          }else {
            assert!(
                bids.is_empty()==true,
                "this token haven't an active offer:  {}",
                contract_and_token_id 
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
#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_purchase(
        &mut self,
        buyer_id: AccountId,
        price: U128,
    ) -> Promise;
}

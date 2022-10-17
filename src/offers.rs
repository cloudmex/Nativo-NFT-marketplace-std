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
    pub fn add_offer( 
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        owner_id:AccountId,
     )  {
         // the variables 
         let bidder_id= env::predecessor_account_id();
         let bid_amount = env::attached_deposit();

        //DECLARE A NEW OUTPUT STRUCT FOR EVENT JSON TO ADDA NEW OFFER
       let  mut new_event=  AddOffer {
              current_owner_id:owner_id.clone(),
              old_bidder_account_id:None,
              bidder_account_id: bidder_id.clone(),
              nft_contract_id: nft_contract_id.clone(),
              token_id: token_id.clone(),
              bidded_price: U128::from(bid_amount) ,
              offer_time:env::block_timestamp(),
              _type:None,
        } ;

        // create the index
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);

        assert!( bidder_id.clone()!=owner_id.clone(),"You can not add a bid for your token");
        assert!( bid_amount.clone()> 0,"The bid must be more than 0");

        //1 LOOKD IF THE TOKEN HAS A PREV SALE
        let   market_data:Option<Sale> =  self.sales.get(&contract_and_token_id);
        //IF THE TOKEN IS LISTED AS SALE 
        if !market_data.is_none() {
            //OFERT SYNC WILL BE ADDED

            new_event._type=Some("offer_sync".to_string());
         
            //LOOK IF THE SALE HAS A PREV BID SYNC  
            let   prev_offer:Option<Offers>= self.offers.get(&contract_and_token_id.clone());
            //IF THE SALE HAS A OLD BID
            if !prev_offer.is_none() {
          
                //SAVE THE OLD BIDDER IN THE OUTPUT STRUCTURE
                    new_event.old_bidder_account_id=Some(prev_offer.clone().unwrap().buyer_id);
                //Assert the amount is more than the actual bid
                    assert!(bid_amount.clone()> u128::from(prev_offer.clone().unwrap().price),
                    "The new bid must more than the current bid price: {:?}",u128::from(prev_offer.clone().as_ref().unwrap().price));
               
                //assert that the bidder isnot the previous one    
                   assert!( bidder_id.clone()!=prev_offer.clone().unwrap().buyer_id,"You can not add a new bid having one active" );
                
                    //refound the bid to the old bidder
                        Promise::new(prev_offer.clone().unwrap().buyer_id).transfer(u128::from(prev_offer.as_ref().unwrap().price));
                    //create a new offer structure
                        let newoffer = Offers {
                            token_id: token_id.clone() ,
                            nft_contract_id: nft_contract_id.clone().into() ,
                            owner_id:  owner_id.clone()  ,
                            buyer_id: bidder_id.clone() ,
                            approval_id: market_data.clone().as_ref().unwrap().approval_id ,
                            price:  bid_amount.into(),
                            ft_token_id: None,};

                   //save the offer to the contract_
                        self.add_offer_to_state(owner_id.clone() ,bidder_id.clone(),nft_contract_id.clone(),token_id.clone(),newoffer.clone());

                    // THIS WILL THROW A EVENT TO NOTIFY THAT THE SALE HAS A OLD BID,A NEW HIGHEST BID AND SHOULD BE 
                    //NOTIFIED THAT 
                    // > THE ONWER HAS A NEW OFFER 
                    // > THE OLD BIDDER HAS BEEN SURPASSED AND HE GOT HIS BID
                    // > THE NEW BIDDER IS SETTED AS THE HIGHEST BIDDER
        
                        // THROW THE  OFFER SYNC WITH PREV OFFER

                        Contract::event_add_offer(new_event); 

           } //IF THE SALE DOESNT HAS A OLD BID
           else{
 
                //create a new offer structure
                let newoffer = Offers {
                        token_id: token_id.clone() ,
                        nft_contract_id: nft_contract_id.clone().into() ,
                        owner_id:  owner_id.clone()  ,
                        buyer_id: bidder_id.clone() ,
                        approval_id: market_data.clone().as_ref().unwrap().approval_id  ,
                        price:  bid_amount.into(),
                        ft_token_id: None, };

                //save the offer to the contract_
                     self.add_offer_to_state(owner_id.clone() ,bidder_id.clone(),nft_contract_id.clone(),token_id.clone(),newoffer.clone());
              
                // THIS WILL THROW A EVENT TO NOTIFY THAT THE SALE HAS A OLD BID,A NEW HIGHEST BID AND SHOULD BE 
                //NOTIFIED THAT 
                // > THE ONWER HAS A NEW OFFER 
                // > THE NEW BIDDER IS SETTED AS THE HIGHEST BIDDER
        
                // THROW THE  OFFER SYNC WITHOUT PREV OFFER
                    Contract::event_add_offer(new_event); 

           }
        
                    
        }
        //IF THE TOKEN ISN'T LISTED AS SALE 
        else {

            new_event._type=Some("offer_async".to_string());

            //look if exists a previous offer lower than the actual 
                let   prev_offer:Option<Offers>= self.offers.get(&contract_and_token_id.clone());
            //IF THE NFT NOT LISTED HAS AN ACTIVE OFFER 
                if !prev_offer.is_none() {
                    //WERE FOUNEDED A PREV BID
                    new_event.old_bidder_account_id=Some(prev_offer.clone().unwrap().buyer_id);

                //aassert the amount is more than the actual bid
                assert!(bid_amount.clone()> u128::from(prev_offer.as_ref().unwrap().price),
                    "The new bid must more than the current bid price: {:?}", u128::from(prev_offer.clone().as_ref().unwrap().price) );
                // //assert that the bidder isnot the previous one
                // env::log_str(&bidder_id.clone().to_string());
                // env::log_str(&prev_offer.as_ref().unwrap().buyer_id.to_string());

                assert!(bidder_id.clone()!=prev_offer.as_ref().unwrap().buyer_id,
                    "You can not add a new bid having one active" );
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
                            ft_token_id: None,};

                    //save the offer to the contract_
                    self.add_offer_to_state(owner_id.clone() ,bidder_id.clone(),nft_contract_id.clone(),token_id.clone(),newoffer.clone());
                
                // THIS WILL THROW A EVENT TO NOTIFY THAT THE NFT ISTN LISTED AS SALE BUT HAS A OLD BID,A NEW HIGHEST BID AND SHOULD BE 
                //NOTIFIED THAT 
                // > THE ONWER HAS A NEW OFFER 
                // > THE OLD BIDDER HAS BEEN SURPASSED AND HE GOT HIS BID
                // > THE NEW BIDDER IS SETTED AS THE HIGHEST BIDDER
                // THROW THE  OFFER ASYNC WITH PREV OFFER

                    Contract::event_add_offer(new_event); 

            }else{
             //IF THE NFT NOT LISTED DOESNT HAS AN ACTIVE OFFER 

                 //create a new offer structure
                 let newoffer = Offers {
                        token_id: token_id.clone() ,
                        nft_contract_id: nft_contract_id.clone().into() ,
                        owner_id:  owner_id.clone()  ,
                        buyer_id: bidder_id.clone() ,
                        approval_id: 0 ,
                        price:  bid_amount.into(),
                        ft_token_id: None,};
                //save the offer to the contract_
                    self.add_offer_to_state(owner_id.clone() ,bidder_id.clone(),nft_contract_id.clone(),token_id.clone(),newoffer.clone());

                // THIS WILL THROW A EVENT TO NOTIFY THAT THE NFT ISTN LISTED AS SALE BUT HAS A OLD BID,A NEW HIGHEST BID AND SHOULD BE 
                //NOTIFIED THAT 
                // > THE ONWER HAS A NEW OFFER 
                // > THE NEW BIDDER IS SETTED AS THE HIGHEST BIDDER
                // THROW THE  OFFER ASYNC WITH PREV OFFER
                 Contract::event_add_offer(new_event); 
            }

 
        }
             
    }
    


     //returns paginated sale objects associated with a given nft contract. (result is a vector of sales)
    pub fn get_offer(
        &self,
        nft_contract_id: AccountId,
        token_id:TokenId,
    ) -> Offers {

        let emprs = Offers {
            token_id: "null".to_string(),
            nft_contract_id: "null".to_string(),
            owner_id: "null".to_string().try_into().unwrap(),
            buyer_id: "null".to_string().try_into().unwrap(),
            approval_id: 0 as u64,
            price: 0.into(),
            ft_token_id:Some("null".parse::<AccountId>().unwrap()),
        
        };
      let   res:Option<Offers> =self.offers.get(&format!("{}{}{}", nft_contract_id, DELIMETER, token_id)) ;

        if res.is_none() {
          //  env::log_str("there is not an offer for this token");
            emprs
        }
        else{
            res.unwrap()
        } 

     }

    #[payable]
    pub fn delete_offer(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        
    ) {
       // assert_one_yocto();
        //this is a new method that will recover the owner in the minter and update the sales and offers before anything transaction
     //   self.update_owner_from_minter(nft_contract_id.clone(), token_id.clone());
   

        let caller = env::signer_account_id();
        let index = format!("{}{}{}", &nft_contract_id.clone(), DELIMETER, &token_id.clone());
        let offer= self.offers.get(&index.clone()).expect("there is not an offer for this token");
        

        assert!(
              caller.clone() == offer.clone().buyer_id ,
             "You are not allowed  "
             );
     
          
       
        //refund
        Promise::new(offer.clone().buyer_id.clone()).transfer(offer.clone().price.0);
        //erase bid
        self.internal_remove_offer(nft_contract_id.clone(),token_id.clone());

        let del_offer = DeleteOffer {
              current_owner_id:offer.clone().owner_id.clone(),
              current_bidder_id:offer.clone().buyer_id.clone(),
              nft_contract_id: nft_contract_id.clone(),
              token_id: token_id.clone(),    
              deleted_time:env::block_timestamp(),
        
        } ;
        Contract::event_delete_offer(del_offer);
        // env::log_str(
        //     &json!({
        //         "type": "delete_offer",
        //         "params": {
        //             "nft_contract_id": offer.clone().nft_contract_id,
        //             "buyer_id": offer.clone().buyer_id,
        //             "token_id": offer.clone().token_id,
                     
        //         }
        //     })
        //     .to_string(),
        // );
    }
    #[private]
    #[payable]
    pub fn accept_offer(&mut self, nft_contract_id: AccountId, token_id: TokenId,owner_id:AccountId) {
         
        //this is a new method that will recover the owner in the minter and update the sales and offers before anything transaction
       // self.update_owner_from_minter(nft_contract_id.clone(), token_id.clone());
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        //get the actual offer nft information
        let   market_data = self.offers.get(&contract_and_token_id).expect("The token does not exist");
     
        let caller = env::signer_account_id();
        let old_owner=owner_id.clone();
         
        assert!(
            caller.clone()==old_owner.clone(),
            "You can not accept the offer,only the owner"
        );
        
        //if the signer is the owner he can accept  
        if caller ==old_owner.clone() {
            // accept
           // env::log_str("it's the owner");
             
                  //remove from the sales 
                     
                  self.internal_remove_offer(nft_contract_id.clone(),token_id.clone());
                  
                        
                        // env::log_str(
                        //     &json!({
                        //     "type": "process_bid",
                        //     "params": {
                        //         "old_owner_id": old_owner.clone(),
                        //         "new_owner_id": caller,
                        //         "nft_contract_id": nft_contract_id,
                        //         "token_id": token_id,
                        //         "price": market_data.clone().price,
                              
                        //     }
                        // })
                        //         .to_string(),
                        // );
          
                        if self.is_mining_ntv_enabled {

                               //pay the NTV 
                                    let tokens_to_mint = u128::from(market_data.clone().price) * self.ntv_multiplier ;
                                    // NTV for the buyer
                                    ext_nft::mint(
                                        market_data.clone().buyer_id,
                                        tokens_to_mint.to_string(),
                                        self.ntvtoken_contract.to_string().try_into().unwrap(),
                                        0000000000000000000000001,
                                        10_000_000_000_000.into(),
                                    );
                                    // NTV for the owner
                                    ext_nft::mint(
                                        old_owner.clone(),
                                        tokens_to_mint.to_string(),
                                        self.ntvtoken_contract.to_string().try_into().unwrap(),
                                        0000000000000000000000001,
                                        10_000_000_000_000.into(),
                                    );
   
                                //    env::log_str("the nvt token minting was payed");    
                        }else{
                            env::log_str("the nvt token minting is disabled");      
                          }
         
           //and process the purchase
           self.process_purchase(
            market_data.clone().nft_contract_id,
            //AccountId::new_unchecked(market_data.clone().nft_contract_id),
                token_id.clone(),
                market_data.clone().price,  //   selected_bid.price.clone().0.into(),
                market_data.clone().buyer_id,
                "offer_sync".to_string(),
            );  

   
        }

       
     }



     #[private]
     pub fn add_offer_to_state(&mut self,owner_id: AccountId,bidder_id: AccountId,nft_contract_id:AccountId,token_id:TokenId,newoffer:Offers){

        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let   oldsale:Option<Sale> = self.sales.get(&contract_and_token_id.clone());

        
        if !oldsale.clone().is_none() {

        //    env::log_str("insert sale");

            let newsale=Sale {
                token_id: oldsale.clone().unwrap().token_id,
                nft_contract_id: nft_contract_id.clone().into(),
                owner_id: owner_id.clone(),
                buyer_id:oldsale.clone().unwrap().buyer_id,
                creator_id: oldsale.clone().unwrap().creator_id,
                title: oldsale.clone().unwrap().title,
                description: oldsale.clone().unwrap().description,
                media: oldsale.clone().unwrap().media,
                approval_id: oldsale.clone().unwrap().approval_id,
                price: oldsale.clone().unwrap().price,
                is_auction:oldsale.clone().unwrap().is_auction,
                bids:oldsale.clone().unwrap().bids,
                ft_token_id:oldsale.clone().unwrap().ft_token_id,
            
            };
           
        
        
            self.sales.insert(&contract_and_token_id.clone(),&newsale );

            



                //get the sales by owner ID for the given owner. If there are none, we create a new empty set
            let mut by_owner_id = self.by_owner_id.get(&owner_id.clone()).unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::ByOwnerIdInner {
                        //we get a new unique prefix for the collection by hashing the owner
                        account_id_hash: hash_account_id(&owner_id.clone()),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
           
            //insert the unique sale ID into the set
            by_owner_id.insert(&contract_and_token_id.clone());
            //insert that set back into the collection for the owner
            self.by_owner_id.insert(&owner_id.clone(), &by_owner_id);

            
         //   env::log_str("inserted sale");

           // env::log_str("insert contract_id as sale");
            //get the token IDs for the given nft contract ID. If there are none, we create a new empty set
            let mut by_nft_contract_id = self
                .by_nft_contract_id
                .get(&nft_contract_id)
                .unwrap_or_else(|| {
                    UnorderedSet::new(
                        StorageKey::ByNFTContractIdInner {
                            //we get a new unique prefix for the collection by hashing the owner
                            account_id_hash: hash_account_id(&nft_contract_id),
                        }
                        .try_to_vec()
                        .unwrap(),
                    )
                });
            
            //insert the token ID into the set
            by_nft_contract_id.insert(&token_id);
            //insert the set back into the collection for the given nft contract ID
            self.by_nft_contract_id
                .insert(&nft_contract_id, &by_nft_contract_id);
            /////////////////
           // env::log_str("inserted contract_id as sale");
 
            
        }
      
     //   env::log_str("insert offer");
    //    env::log_str("insert offer");

         self.offers.insert(&contract_and_token_id.clone(),&newoffer);

       //  env::log_str("inserted offer");

       //  env::log_str("insert offer by owner");

            //get the offers by owner ID for the given owner. If there are none, we create a new empty set
            let mut offers_by_owner_id = self.offers_by_owner_id.get(&owner_id.clone()).unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::ByOffersOwnerIdInner {
                        //we get a new unique prefix for the collection by hashing the owner
                        account_id_hash: hash_account_id(&owner_id.clone()),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
           // env::log_str("insert offer by bidder");

            //get the offers by bidder ID for the given owner. If there are none, we create a new empty set
            let mut offers_by_bidder_id = self.offers_by_bidder_id.get(&bidder_id.clone()).unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::ByOffersBidderIdInner {
                        //we get a new unique prefix for the collection by hashing the owner
                        account_id_hash: hash_account_id(&bidder_id.clone()),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });


            //insert the unique sale ID into the set
            offers_by_owner_id.insert(&contract_and_token_id.clone());

 
             //insert the unique sale ID into the set
             offers_by_bidder_id.insert(&contract_and_token_id.clone());

        //     env::log_str("inserted offer by owner");

            //insert that set back into the collection for the owner
            self.offers_by_owner_id.insert(&owner_id, &offers_by_owner_id);
          //  env::log_str("inserted offer by bidder");

             //insert that set back into the collection for the owner
             self.offers_by_bidder_id.insert(&bidder_id, &offers_by_bidder_id);



        //     env::log_str("insert offer by contract");

            //get the token IDs for the given nft contract ID. If there are none, we create a new empty set
            let mut offers_by_nft_contract_id = self
                .offers_by_nft_contract_id
                .get(&nft_contract_id)
                .unwrap_or_else(|| {
                    UnorderedSet::new(
                        StorageKey::ByOffersNFTContractIdInner {
                            //we get a new unique prefix for the collection by hashing the owner
                            account_id_hash: hash_account_id(&nft_contract_id),
                        }
                        .try_to_vec()
                        .unwrap(),
                    )
                });

            //insert the token ID into the set
            offers_by_nft_contract_id.insert(&contract_and_token_id.clone());
            //insert the set back into the collection for the given nft contract ID
            self.offers_by_nft_contract_id
                .insert(&nft_contract_id, &offers_by_nft_contract_id);

                //env::log_str("inserted offer by contract");

    }

    
}

//this is the cross contract call that we call on our own contract. 
/*
    private method used to resolve the promise when calling nft_transfer_payout. This will take the payout object and 
    check to see if it's authentic and there's no problems. If everything is fine, it will pay the accounts. If there's a problem,
    it will refund the buyer for the price. 
*/
 

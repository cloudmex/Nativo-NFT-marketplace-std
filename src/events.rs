
 
use near_sdk::BlockHeight;

use crate::*;
 

//defines the payout type we'll be parsing from the NFT contract as a part of the royalty standard.
#[derive(Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ResolvePurchase {
    pub old_owner_id:AccountId,
    pub new_owner_id: AccountId,
    pub nft_contract_id: AccountId,
    pub token_id: String,
    pub price_sold: U128,
    pub sold_time:BlockHeight,
    pub _type:Option<String>,
    pub _payouts:Option<HashMap<String,HashMap<AccountId,U128>>>
} 
#[derive(Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AddOffer {
    pub current_owner_id:AccountId,
    pub old_bidder_account_id: Option<AccountId>,
    pub bidder_account_id: AccountId,
    pub nft_contract_id: AccountId,
    pub token_id: String,
    pub bidded_price: U128,
    pub offer_time:BlockHeight,
    pub _type:Option<String>,
} 
#[derive(Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DeleteOffer {
    pub current_owner_id:AccountId,
    pub current_bidder_id:AccountId,

    pub nft_contract_id: AccountId,
    pub token_id: String,    
    pub deleted_time:BlockHeight,

} 

#[near_bindgen]
impl Contract {



    // fn event_std(event_type : String, data:String){
    //     //recieve info
    //     //format the info

    //     // emit a log

    //     env::log_str(
    //         &json!({
    //             "EVENT_JSON":{
    //                 "standard": "nep171",
    //                 "version": "1.0.0",
    //                 "event": event_type,
    //                 "data": *data,
    //             }
    //     })
    //             .to_string(),
    //     );
    // }

    // this event notify a new NFT listed as sale
    pub fn event_std ( _event:String, data:String){
        //format the info
        let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": _event ,
                "data":data
        }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),
        );
    }
    // this event notify a new NFT listed as sale
    pub fn event_list_as_sale( data:Sale){
        //format the info
        let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": "List_as_sale",
                "data":[{"timestamp":env::block_timestamp(),"token_info":data }]
        }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),
        );
    }
    // this event notify a new NFT listed as offer
    pub fn event_list_as_offer( data:Sale){
        //format the info
        let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": "List_as_offer",
                "data":[{"timestamp":env::block_timestamp(),"token_info":data }]
        }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),
        );
    }

    // this event notify a NFT receive a new offer async/sync

    pub fn event_add_offer( data:AddOffer){
        //format the info
        let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": data.clone()._type.unwrap(),
                "data":data
        }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),
        );
    }

    // this event notify a if a bidder cancel a NFT bid
    pub fn event_delete_offer( data:DeleteOffer){
        //format the info
        let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": "offer_deleted".to_string(),
                "data":data
        }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),
        );
    }
    
    pub fn event_royalties_purchase( data:HashMap<String,HashMap<AccountId,U128>>){
        //format the info
        let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": "royalties_payment".to_string() ,
                "data":data,
        }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),
        );
    }
    // this event notify a NFT purchase 
    pub fn event_resolve_purchase( data:ResolvePurchase){
        //format the info
        let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": data.clone()._type.unwrap() ,
                "data":data,
        }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),
        );
    }

    

     
}
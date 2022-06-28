use crate::*;
use near_sdk::{Gas};

/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const TGAS: u64 = 10_000_000_000_000;
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(300_000_000_000_000);
pub const GAS_FOR_UPGRADE_REMOTE_DEPLOY: Gas = Gas(300_000_000_000_000);


#[near_bindgen]
impl Contract {
    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        use near_sys as sys;
        // assert!(env::predecessor_account_id() == self.minter_account_id);
        //input is code:<Vec<u8> on REGISTER 0
        //log!("bytes.length {}", code.unwrap().len());
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
       // const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - Gas(GAS_FOR_UPGRADE);
        unsafe {
            // Load input (new contract code) into register 0
            sys::input(0);

            //prepare self-call promise
            let promise_id =
                sys::promise_batch_create(current_id.as_bytes().len() as _, current_id.as_bytes().as_ptr() as _);

            //1st action, deploy/upgrade code (takes code from register 0)
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

            // 2nd action, schedule a call to "migrate()".
            // Will execute on the **new code**
            sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                0 as _,
                0 as _,
                0 as _,
                u64::from(attached_gas),
            );
        }
    }

/////////////////////METODO DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        
        env::log_str("old state readed");
        Self {
            owner_id: old_state.owner_id,
            treasure_id: old_state.treasure_id,
            sales: old_state.sales,
            by_owner_id: old_state.by_owner_id,
            offers_by_owner_id: LookupMap::new(StorageKey::ByOffersOwnerId),
            offers_by_bidder_id: LookupMap::new(StorageKey::ByOffersBidderId),

            by_nft_contract_id: old_state.by_nft_contract_id,
            offers_by_nft_contract_id: LookupMap::new(StorageKey::ByOffersNFTContractId),
            storage_deposits: old_state.storage_deposits,
            fee_percent:old_state.fee_percent,
            whitelist_contracts:old_state.whitelist_contracts,
            offers: UnorderedMap::new(StorageKey::OffersOutMarket),
            is_mining_ntv_enabled:old_state.is_mining_ntv_enabled,
            collection_id:old_state.collectionID,

        }
    }


   
    #[private]
    #[init(ignore_state)]
    pub fn cleanup()  -> Self {
        
        env::log_str("clean up state");
        Self {
            //set the owner_id field equal to the passed in owner_id. 
            owner_id:env::signer_account_id(),
            treasure_id:env::signer_account_id(),
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            sales: UnorderedMap::new(StorageKey::Sales),
            by_owner_id: LookupMap::new(StorageKey::ByOwnerId),
            offers_by_owner_id: LookupMap::new(StorageKey::ByOffersOwnerId),
            offers_by_bidder_id: LookupMap::new(StorageKey::ByOffersBidderId),
            by_nft_contract_id: LookupMap::new(StorageKey::ByNFTContractId),
            offers_by_nft_contract_id: LookupMap::new(StorageKey::ByOffersNFTContractId),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
            fee_percent:0.6,
            whitelist_contracts: LookupMap::new(StorageKey::ContractAllowed),
            offers: UnorderedMap::new(StorageKey::OffersOutMarket),
            is_mining_ntv_enabled:true,
            collection_id:0,

        }
    }

}
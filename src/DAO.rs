use crate::*;
 
#[near_bindgen]
impl Contract {

    //get the actual treasure_id
    pub fn get_treasury(&self) -> AccountId {
        // validate if the contract already exist,dont create a new one
        self.treasure_id.clone()
    }
    //set a new treasure_id
    pub fn set_treasury(&mut self,new_account:AccountId) -> AccountId {
        self.is_the_owner();
            //if the caller is the owner
        self.treasure_id=new_account;
        self.treasure_id.clone()
    }



   


    //get th actual owner address
    pub fn get_owner_account(&self) -> AccountId {
        self.owner_id.clone()
    }
    //modificar cuentas
    pub fn set_owner_account(&mut self,new_account:AccountId) -> AccountId {
        self.is_the_owner();
        //if the caller is the owner
        self.owner_id=new_account;
        self.owner_id.clone()

    }

    //method for change the fee percentages
    pub fn get_mint_fee(&self,) {
        let rest= "the fee comision  is: ".to_string()+&self.fee_percent.to_string();
        env::log_str(&rest);
        
    }
    pub fn set_mint_fee(&mut self,mint_fee:f64) {
        self.is_the_owner();
        assert_eq!(mint_fee<1.0,true,"max fee comision cant be more than 100%");
        self.fee_percent=mint_fee;        
    }


     #[payable]
    pub fn add_new_ext_contract(
        &mut self,
        address_contract: AccountId,
        contract_name: String,
    ) {
         
        //self.is_the_owner();
        // validate that the info sended isnt empty
        assert_eq!(address_contract.to_string().is_empty(),false,"the contract address cannot be empty");
//assert_eq!(address_owner.to_string().is_empty(), false,"the owner address cannot be empty");
        assert_eq!(contract_name.is_empty(), false, "the title cannot be empty");
        // validate that the attached sended is correct
       /* let amount = env::attached_deposit();
        assert_eq!(
            "5000000000000000000000000"
                .to_string()
                .parse::<u128>()
                .unwrap(),
            amount,
            "Wrong amount deposited,please check for the correct amount!"
        );*/
        // validate if the contract already exist,dont create a new one
        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());
        if !contract_exist.is_none() {
            assert_eq!(
                contract_exist.unwrap().contract_name.is_empty(),
                true,
                "the contract already exist"
            );
        }
        // create a new contract structure
        let new_ext_contract = ExternalContract {
            register_address: env::signer_account_id(),
            contract_name: contract_name.clone(),
            contract_balance:0,
        };
        //modify  and save the information
    
        self.whitelist_contracts.insert(&address_contract.clone(), &new_ext_contract);
        let cont = self.whitelist_contracts.get(&address_contract.clone());

        let js =format!("address_contract:{},owner_id:{},contract_name:{}",
        address_contract,
        cont.clone().unwrap().register_address,
        cont.unwrap().contract_name);

        env::log_str(&js.to_string());
         
    }

     //validate if the owner is the caller
     fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_id==env::predecessor_account_id(),
            true,
            "!you are not the contract owner address¡"
        );
    }

     
    pub fn is_white_listed(&self)   {
        // validate if the contract already exist,dont create a new one
        self.whitelist_contracts.get(&env::predecessor_account_id()).expect("the contract isnt approved to list NFT's");
  }
}

 

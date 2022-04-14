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



    //validate if the owner is the caller
    fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_id==env::predecessor_account_id(),
            true,
            "!you are not the contract owner address¡"
        );
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
    
}

 

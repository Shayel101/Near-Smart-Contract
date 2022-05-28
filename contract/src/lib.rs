// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};
use near_sdk::collections::LookupMap;

setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    memo: LookupMap<String, Vec<String>>
}

impl Default for Contract {
  fn default() -> Self {
    Self {
        memo: LookupMap::new(b"memo".to_vec())
    }
  }
}

#[near_bindgen]
impl Contract {
    //change methods
    pub fn add_memo(&mut self, memo_text:String, price:String){
        let account_id = env::signer_account_id();
        let contain_user = self.memo.contains_key(&account_id);
        if contain_user{
            let mut temp_list = match self.memo.get(&account_id){
                Some(x)=>x,//vector/collection of memos
                None=> vec![]
            };
            temp_list.push(memo_text+" || "+ &price+"NEAR");//inserting new memo to temp list
            self.memo.insert(&account_id, &temp_list);//saving newly modified list to blockchain

        }else{
            let fresh_vec = vec![memo_text+" || "+ &price+"NEAR"];//if new user then creating a new memo list
            self.memo.insert(&account_id, &fresh_vec);//saving newly modified list to blockchain
        }
    }

    //Transferring tokens to other user 
    pub fn transfer_money(&mut self, account_id: AccountId, amount: f64){
        Promise::new(account_id).transfer(amount as u128);//transferring tokens
    }

    //View Method
    pub fn get_memos(self, user:String) -> Vec<String>{
        match self.memo.get(&user){
            Some(x)=>x,//if have memos then return list of memos["memo1", "memo2"]
            None=>vec![]//if don't have any memo then return empty list
        }
    }
}




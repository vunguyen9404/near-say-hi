// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Timestamp, BlockHeight, BorshStorageKey, log, Balance};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{UnorderedMap};

setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Comment {
    pub message: String,
    pub author: AccountId,
    pub donation: Balance,
    pub created_at: Timestamp,
    pub created_at_block: BlockHeight
}

impl Comment {
    pub fn is_donate(&self) -> bool {
        self.donation > 0
    }
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    RecordKey
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SayHiContract {
    pub records: UnorderedMap<u128, Comment>,
    pub next_id: u128
}

impl Default for SayHiContract {
    fn default() -> Self {
        Self { 
            records: UnorderedMap::new(StorageKey::RecordKey), 
            next_id: 0 
        }
    }
}

#[near_bindgen]
impl SayHiContract {

    /**
     * User can post new message and donate NEAR for contract
     */
    #[payable]
    pub fn add_new_comment(&mut self, message: String) -> u128 {
        assert!(message.len() > 0, "ERR_INVALID_MESSAGE_CONTENT");
        let account_id = env::predecessor_account_id();

        log!("Account {} post new message {}", account_id.clone(), message.clone());

        let comment = Comment {
            author: account_id,
            message: message,
            donation: env::attached_deposit(),
            created_at: env::block_timestamp(),
            created_at_block: env::block_index()
        };

        self.records.insert(&self.next_id, &comment);
        let old_id = self.next_id;
        self.next_id += 1;

        old_id
    }

    /**
     * Get top 10 message have donation =))
     */
    pub fn get_top_message(&self) -> Vec<Comment> {
        let keys = self.records.keys_as_vector();
        let start = if keys.len() > 10 {
            keys.len() - 10
        } else {
            0
        };

        keys.iter()
        .skip(start as usize)
        .take(10)
        .filter(|id| self.records.get(id).unwrap().is_donate())
        .map(|id| self.records.get(&id).unwrap())
        .collect()
    }

    pub fn get_message_by_id(&self, id: U128) -> Comment {
        self.records.get(&id.0).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool, attached_deposit: u128) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: attached_deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn create_new_message() {
        let context = get_context(vec![], false, 0);
        testing_env!(context);
        let mut contract = SayHiContract::default();
        let id = contract.add_new_comment(String::from("Hello World"));

        // Get message
        let comment = contract.get_message_by_id(U128(id));
        assert_eq!(comment.author, "carol_near".to_string());
        assert_eq!(comment.message, String::from("Hello World"));

        // Get list message
        let comments = contract.get_top_message();
        assert_eq!(comments.len(), 0); // No message with donate
    }

    #[test]
    fn create_new_message_with_donation() {
        let context = get_context(vec![], false, 1000);
        testing_env!(context);
        let mut contract = SayHiContract::default();
        let id = contract.add_new_comment(String::from("Hello World"));

        // Get message
        let comment = contract.get_message_by_id(U128(id));
        assert_eq!(comment.author, "carol_near".to_string());
        assert_eq!(comment.message, String::from("Hello World"));
        assert_eq!(comment.donation, 1000);

        // Get list message
        let comments = contract.get_top_message();
        assert_eq!(comments.len(), 1); // No message with donate
        assert_eq!(comments[0].author, comment.author);
        assert_eq!(comments[0].message, comment.message);
        assert_eq!(comments[0].donation, 1000);
    }
}

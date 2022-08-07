use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::MemData;

#[derive(Debug, Clone)]
pub struct State {
    pub items: Vec<Item>,
    pub input_item: InputItem,
    pub input_secret: String,
    pub secret: MemData,
    pub guild_tips_msg: String,
}

const aes_key_num: usize = 32;


#[derive(Debug, Clone, Default)]
pub struct InputItem {
    pub account_value: String,
    pub secret_value: String,
    pub desc_value: String,
}


impl InputItem {
    pub fn clear(&mut self) {
        self.account_value = "".to_string();
        self.secret_value = "".to_string();
        self.desc_value = "".to_string();
    }
}


#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Item {
    pub(crate) id: String,
    pub(crate) account: String,
    pub(crate) secret: String,
    pub(crate) desc: String,
    pub(crate) status: usize,
    pub nonce: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Export {
    pub version: i32,
    pub desc: String,
    pub time: String,
    pub content: String,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let key = "abcd12347".to_string();
        let new_key = get_valid_key(key).unwrap();
        println!("{}", new_key);
    }

    fn get_valid_key(key: String) -> Result<String> {
        let key_vec = key.into_bytes();
        let mut new_key: Vec<u8> = vec![];
        let time = 32 / key_vec.len();
        let nums = 32 / key_vec.len();
        for _ in 0..time {
            new_key.append(&mut key_vec.clone());
        }
        for i in 0..nums {
            new_key.push(*key_vec.get(i).unwrap());
        }
        Ok(String::from_utf8(new_key).unwrap())
    }
}
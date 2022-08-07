use std::path::Path;

use anyhow::{anyhow, bail, Result};
use rocksdb::{DB, Error, Options};
use uuid::Uuid;

use super::constants::{ITEM_KEYS_KEY, ITEM_LIST_KEY, SECRET_HASH_KEY};
use super::types::Item;

#[derive(Debug)]
pub struct Database {
    db: DB,
}

impl Database {



    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let rocks_db = DB::open_default(path)?;
        let database = Database { db: rocks_db };
        Ok(database)
    }

    pub fn put_secret_hash<V: AsRef<[u8]>>(&self, value: V) -> Result<()> {
        self.db.put(SECRET_HASH_KEY, value).map_err(|e| anyhow!("{}",e))
    }

    pub fn get_secret_hash(&self) -> Result<Option<Vec<u8>>> {
        self.db.get(SECRET_HASH_KEY).map_err(|e| anyhow!("{}",e))
    }

    pub fn put_item(&self, item: &Item) -> Result<()> {
        let uuid = &item.id;
        let data = serde_json::to_string(item)?;
        self.put_item_key(uuid)?;
        self.db.put(uuid, data).map_err(|e| anyhow!("{}",e))
    }

    pub fn get_item<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Item>> {
        let result = self.db.get(key).map_err(|e| anyhow!("{}",e))?;
        if let Some(data) = result {
            let item: Item = serde_json::from_slice(&data)?;
            return Ok(Some(item));
        }
        Ok(None)
    }

    pub fn del_item(&self, key: &str) -> Result<()> {
        self.delete_item_key(key)?;
        self.db.delete(key).map_err(|e| anyhow!("{}",e))?;
        Ok(())
    }


    pub fn get_item_list(&self) -> Result<Option<Vec<Item>>> {
        let mut items: Vec<Item> = Vec::new();
        let result = self.get_item_keys()?;
        if let Some(list) = result {
            for key in list {
                let item = self.get_item(key)?;
                if let Some(it) = item {
                    items.push(it);
                }
            }
            return Ok(Some(items));
        }
        Ok(None)
    }


    fn put_item_key(&self, value: &str) -> Result<()> {
        let result = self.db.get(ITEM_KEYS_KEY).map_err(|e| anyhow!("{}",e))?;
        if let Some(data) = result {
            let mut keys: Vec<&str> = serde_json::from_slice(&data)?;
            keys.push(value);
            let db_keys = serde_json::to_string(&keys)?;
            self.db.put(ITEM_KEYS_KEY, db_keys)?;
        } else {
            let mut keys: Vec<&str> = Vec::new();
            keys.push(value);
            let db_keys = serde_json::to_string(&keys)?;
            self.db.put(ITEM_KEYS_KEY, db_keys).map_err(|e| anyhow!("{}",e))?;
        }
        Ok(())
    }


    fn delete_item_key(&self, key: &str) -> Result<()> {
        let result = self.get_item_keys()?;
        let mut keys: Vec<String> = Vec::new();
        if let Some(list) = result {
            for k in list {
                if k != key {
                    keys.push(k)
                }
            }
            let db_keys = serde_json::to_string(&keys)?;
            self.db.put(ITEM_KEYS_KEY, db_keys).map_err(|e| anyhow!("{}",e))?;
        }
        Ok(())
    }

    fn get_item_keys(&self) -> Result<Option<Vec<String>>> {
        let result = self.db.get(ITEM_KEYS_KEY).map_err(|e| anyhow!("{}",e))?;
        if let Some(data) = result {
            let keys: Vec<String> = serde_json::from_slice(&data)?;
            return Ok(Some(keys));
        }
        Ok(None)
    }

    pub fn put<K, V>(&self, key: K, value: V) -> Result<()> where K: AsRef<[u8]>, V: AsRef<[u8]> {
        self.db.put(key, value).map_err(|e| anyhow!("{}",e))
    }

    pub fn get<K>(&self, key: K) -> Result<Option<Vec<u8>>> where K: AsRef<[u8]> {
        self.db.get(key).map_err(|e| anyhow!("{}",e))
    }

    pub fn delete<K>(&self, key: K) -> Result<()> where K: AsRef<[u8]> {
        self.db.delete(key).map_err(|e| anyhow!("{}",e))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_operation() {
        let db = Database::new(".db").unwrap();
        let items = db.get_item_list().unwrap();
        if let Some(list) = items {
            for i in list {
                println!("item1 list: {:?}", i);
            }
        }

        let uuid = Uuid::new_v4().to_string();
        let item = Item {
            id: uuid.clone(),
            account: "test01".to_string(),
            secret: "abcd1234".to_string(),
            desc: "email".to_string(),
            status: 0,
            nonce: "".to_string()
        };
        db.put_item(&item).unwrap();
        println!("add item: {:?}", uuid);

        let option = db.get_item(&uuid).unwrap();
        if let Some(it) = option {
            println!("get item: {:?}", it);
        }


        let items = db.get_item_list().unwrap();
        if let Some(list) = items {
            for i in list {
                println!("item2 list: {:?}", i);
            }
        }


        // db.del_item(&uuid).unwrap();
        let items = db.get_item_list().unwrap();
        if let Some(list) = items {
            for i in list {
                println!("item3 list: {:?}", i);
            }
        }

        println!("end ...");
    }


    #[test]
    fn secret_hash() {
        let db = Database::new(".db").unwrap();
        db.put_secret_hash("ddddd").unwrap();
        let option = db.get_secret_hash().unwrap().unwrap();
        let result = String::from_utf8(option).unwrap();
        println!("{:?}", result);
    }


    #[test]
    fn db_test() {
        //  let db = DB::open_default(".db").unwrap();
        let database = Database::new(".db").unwrap();
        database.put("account", "password").unwrap();
        let data = database.get("account").unwrap().unwrap();
        println!("{:?}", String::from_utf8(data));
        database.delete("account").unwrap();

        let option = database.get("account").unwrap();
        println!("{:?}", option);
    }
}

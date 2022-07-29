use std::path::Path;

use anyhow::{anyhow, Result};
use rocksdb::{DB, Error, Options};

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
    fn it_works() {
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

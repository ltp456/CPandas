use std::fmt::{Debug, Formatter};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, AeadMut, NewAead};
use anyhow::{anyhow, Result};
use rand::Rng;
use self_encryption::{bytes::Bytes, ChunkInfo, DataMap, decrypt_full_set, encrypt, EncryptedChunk};
use sha2::{Digest, Sha256};

const AES_256_KEY_NUM: usize = 32;
const AES_256_NONCE_NUM: usize = 12;

// todo find best way
const MIN_ENCRYPT_BYTE_NUMS: usize = 3072;



pub fn sha256(msg: &[u8]) -> Result<String> {
    let hash = Sha256::digest(msg);
    let hash_hex = hex::encode(hash);
    Ok(hash_hex)
}

// all parameter is hex
pub fn aes256_hex_decrypt(msg: &str, key: &str, nonce: &str) -> Result<Vec<u8>> {
    let msg_hex = hex::decode(msg.trim_start_matches("0x"))?;
    let key_hex = hex::decode(key.trim_start_matches("0x"))?;
    let nonce_hex = hex::decode(nonce.trim_start_matches("0x"))?;
    let key = Key::from_slice(key_hex.as_slice());
    let nonce = Nonce::from_slice(nonce_hex.as_slice());
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.decrypt(nonce, msg_hex.as_slice()).map_err(|e| anyhow!("{}",e))?;
    Ok(ciphertext)
}

pub fn aes256_hex_encrypt(msg: &str, key: &str) -> Result<(String, String)> {
    let key_hex = hex::decode(key.trim_start_matches("0x"))?;
    let key = Key::from_slice(key_hex.as_slice());
    let cipher = Aes256Gcm::new(key);
    let nonce = gen_rand_key(AES_256_NONCE_NUM)?;
    let nonce = Nonce::from_slice(nonce.as_slice());
    let ciphertext = cipher.encrypt(nonce, msg.as_bytes()).map_err(|e| anyhow!("{}",e))?;
    let cipher_hex = hex::encode(ciphertext);
    let nonce_hex = hex::encode(nonce);
    Ok((cipher_hex, nonce_hex))
}


pub fn aes256_decode(msg: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    let key = Key::from_slice(key);
    // 96-bits; unique per message
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.decrypt(nonce, msg).map_err(|e| anyhow!("{}",e))?;
    Ok(ciphertext)
}


pub fn aes256_encode(msg: &[u8], key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    // 96-bits; unique per message, 12 byte
    let nonce = gen_rand_key(AES_256_NONCE_NUM)?;
    let nonce_array = Nonce::from_slice(nonce.as_slice());
    let ciphertext = cipher.encrypt(nonce_array, msg).map_err(|e| anyhow!("{}",e))?;
    Ok((ciphertext, nonce))
}



pub fn aes256_encode_with_nonce(msg: &[u8], key: &[u8],nonce:&[u8]) -> Result<Vec<u8>> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    // 96-bits; unique per message, 12 byte
    let nonce_array = Nonce::from_slice(nonce);
    let ciphertext = cipher.encrypt(nonce_array, msg).map_err(|e| anyhow!("{}",e))?;
    Ok(ciphertext)
}



pub fn aes256_key() -> Result<Vec<u8>> {
    gen_rand_key(AES_256_KEY_NUM)
}


pub fn gen_rand_key(num: usize) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut key = Vec::<u8>::new();
    for _ in 0..num {
        let rn = rng.gen::<u8>();
        key.push(rn)
    }
    Ok(key)
}


pub fn get_valid_aes_key(key: String) -> Result<String> {
    let key_vec = key.into_bytes();
    let mut new_key: Vec<u8> = vec![];
    let time =  AES_256_KEY_NUM/ key_vec.len();
    let nums = AES_256_KEY_NUM % key_vec.len();
    for _ in 0..time {
        new_key.append(&mut key_vec.clone());
    }
    for i in 0..nums {
        new_key.push(*key_vec.get(i).unwrap());
    }
    let result = String::from_utf8(new_key)?;
    Ok(result)
}




pub struct MemData {
    data_map: DataMap,
    length: usize,
    encrypted_chunks: Vec<EncryptedChunk>,
}

impl Debug for MemData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl Default for MemData {
    fn default() -> Self {
        let data_map = DataMap::new(Vec::<ChunkInfo>::new());
        MemData {
            data_map,
            length: 0,
            encrypted_chunks: vec![],
        }
    }
}

impl Clone for MemData {
    fn clone(&self) -> Self {
        let mut encrypted_chunks = Vec::<EncryptedChunk>::new();
        for item in &self.encrypted_chunks {
            let chunk = item.clone();
            encrypted_chunks.push(chunk);
        }
        Self {
            data_map: self.data_map.clone(),
            length: self.length,
            encrypted_chunks,
        }
    }
}


impl MemData {
    pub fn new(mut data: Vec<u8>) -> Result<Self> {
        let length = data.len();
        let mut rng = rand::thread_rng();
        for _ in length..MIN_ENCRYPT_BYTE_NUMS {
            let x = rng.gen::<u8>();
            data.push(x);
        }
        let bytes = Bytes::from(data);
        let (data_map, encrypted_chunks) = encrypt(bytes)?;
        let mem_data = MemData {
            data_map,
            length,
            encrypted_chunks,
        };
        Ok(mem_data)
    }
    pub fn decrypt(&self) -> Result<Vec<u8>> {
        let content = decrypt_full_set(&self.data_map, self.encrypted_chunks.as_ref())?;
        let vec = content.to_vec();
        let r = vec[0..self.length].to_vec();
        Ok(r)
    }
}


#[cfg(test)]
mod test {
    use std::string::String;
    use super::*;


    #[test]
    fn test_valid_aes_key(){
        let new_key = get_valid_aes_key("abcd1234".to_string()).unwrap();
        println!("{}",new_key);

    }


    #[test]
    fn test() {
        let msg = b"wohenisdfsdfsfslsdfks";
        let key = gen_rand_key(32).unwrap();
        println!("key:{}", hex::encode(&key));
        let (chipper_text, nonce) = aes256_encode(msg, key.as_slice()).unwrap();


        // let msg1 = hex::decode(chipper_text).unwrap();
        // let nonce = hex::decode(nonce).unwrap();

        let plain_text = aes256_decode(chipper_text.as_slice(), key.as_slice(), nonce.as_slice()).unwrap();

        println!("decode: {:?}", String::from_utf8(plain_text));

    }


    #[test]
    fn test_encrypted() {
        let mut res = String::from("woheni");
        println!("{:?}", res.clone().into_bytes().len());
        println!("{:?}", res);
        let mem_data = MemData::new(res.into_bytes()).unwrap();
        let d = mem_data.decrypt().unwrap();

        println!("{:?}", String::from_utf8(d).unwrap());
    }
}
use anyhow::Result;
use sha2::{Digest, Sha256};

pub fn sha256<T: AsRef<[u8]>>(data: T) -> Result<()> {
    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let result = hasher.finalize();
    println!("{:?}", result);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        sha256("nihao").unwrap();
    }
}
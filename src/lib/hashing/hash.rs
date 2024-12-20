use data_encoding::HEXLOWER;
use sha2::{Sha256, Digest};

pub fn hash_string(input: String) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(input);
    
    // read hash digest and consume hasher
    let result = hasher.finalize();
    HEXLOWER.encode(result.as_ref())
}
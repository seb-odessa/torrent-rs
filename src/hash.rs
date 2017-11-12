use crypto;
use crypto::digest::Digest;

pub fn sha1(input: &[u8]) -> Vec<u8> {
    let mut hasher = crypto::sha1::Sha1::new();
    hasher.input(input);
    let mut hash: Vec<u8> = vec![0; hasher.output_bytes()];
    hasher.result(&mut hash);
    return hash;
}

use rand::Rng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Digest, Sha256};
// use ripemd160::Ripemd160;
use base58::ToBase58;


fn generate_private_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut private_key = [0u8; 32];
    rng.fill(&mut private_key);
    SecretKey::from_slice(&private_key).expect("32 bytes, within curve order");
    private_key

} 

fn public_key_from_private_key(private_key: &[u8; 32]) -> [u8; 33] {
    
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
       let publicKey = PublicKey::from_secret_key(&secp, &secret_key);
 

        let mut public_key = [0u8; 33];
    public_key[0] = 0x02; // Pretend to be a compressed public key
    public_key[1..33].copy_from_slice(private_key);
    public_key
    

}

fn main() {
     generate_private_key();
    
    
}

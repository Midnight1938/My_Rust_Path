use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};
use rand::Rng;
use rpassword;
fn main() {
    // Generate a random encryption key
    let key = Aes256Gcm::generate_key(&mut OsRng);

    // Create a new cipher with the generated key
    let cipher = Aes256Gcm::new(&key);

    // Create a unique nonce (96 bits) for this message
    let nonce = Nonce::from_slice(&OsRng.gen::<[u8; 12]>());

    // Encrypt a plaintext message
    let plaintext = b"plaintext message";
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();

    // Decrypt the ciphertext
    let preDecryp = rpassword::prompt_password("Enter Code: ").unwrap();
    let decryp = Nonce::from_slice(&preDecryp.as_bytes());

    let decrypted = cipher.decrypt(decryp, ciphertext.as_ref()).unwrap();

    println!("Encrypted plaintext: {:?}", ciphertext);
    println!("Decrypted plaintext: {:?}", String::from_utf8(decrypted).unwrap());
}

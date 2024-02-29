use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};

fn main() {
    // Generate a random encryption key
    let key = Aes256Gcm::generate_key(&mut OsRng);

    // Create a new cipher with the generated key
    let cipher = Aes256Gcm::new(&key);

    // Create a unique nonce (96 bits) for this message
    let nonce = Nonce::from_slice(b"unique nonce");

    // Encrypt a plaintext message
    let plaintext = b"plaintext message";
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();

    // Decrypt the ciphertext
    let decrypted = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();

    println!("Encrypted plaintext: {:?}", ciphertext);
    println!("Nonce: {:?}", nonce);
    println!("Decrypted plaintext: {:?}", String::from_utf8(decrypted).unwrap());
}

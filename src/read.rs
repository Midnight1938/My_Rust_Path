use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};
use rand::{Rng, RngCore};
use rpassword;

pub fn read_loop(infile: &str, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>, decrypt: bool) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut buffer = [0; CHUNK_SIZE];
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        let _ = stats_tx.send(num_read); // Dont care if it cant see stats

        // TODO Decryption
        if decrypt {
            let pascode = rpassword::prompt_password("Enter Decryption Key").unwrap();
            let nonce_bytes = pascode.as_bytes();
            let nonce = Nonce::from_slice(&nonce_bytes);
            let uncode = cipher.decrypt(nonce, &buffer[..num_read]).unwrap();
            if write_tx.send(Vec::from(uncode)).is_err() {
                break;
            };
        } else {
            let mut nonce_bytes = [0; 12];
            OsRng.fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);
            let ciphertext = cipher.encrypt(nonce, &buffer[..num_read]).unwrap();
            println!("Key: {:?}", nonce);
            if write_tx.send(Vec::from(ciphertext)).is_err() {
                break;
            }
        }

        // if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
        //     break;
        // };
    }
    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new()); // empty vec

    Ok(())
}

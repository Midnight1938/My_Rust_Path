use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};
use rand::{RngCore};
use rpassword;

pub fn read_loop(infile: &str, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>, decrypt: bool) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        let _ = stats_tx.send(num_read); // Dont care if it cant see stats

        // TODO Decryption
        if write_tx.send(Vec::from(scrambler(decrypt, num_read, &buffer))).is_err() {
            break;
        }

        // if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
        //     break;
        // };
    }
    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new()); // empty vec

    Ok(())
}

pub fn scrambler(decrypt: bool, num_read: usize, buffer: &[u8]) -> Vec<u8> {
    let key = Aes256Gcm::generate_key(&mut OsRng);

    if decrypt {
        let pascode = rpassword::prompt_password("Enter Decryption Key").unwrap();
        let nonce = Nonce::from_slice(&pascode.as_bytes());
        let cipher = Aes256Gcm::new(&key);
        return cipher.decrypt(nonce, &buffer[..num_read]).unwrap().to_vec();
    } else {
        let mut nonce_bytes = [0; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let cipher = Aes256Gcm::new(&key);

        // Include the nonce in the encrypted data
        let mut ciphertext = nonce_bytes.to_vec();
        ciphertext.extend_from_slice(&cipher.encrypt(nonce, &buffer[..num_read]).unwrap());
        return ciphertext;
    }
}

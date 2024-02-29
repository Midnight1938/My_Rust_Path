use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};
use rpassword;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use rand::Rng;

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

        // TODO Output the nocne for storage
        if decrypt{
            let predecryp = rpassword::prompt_password("Enter Code").unwrap();
            let decrypt = Nonce::from_slice(&predecryp.as_bytes());
            if write_tx.send(Vec::from(cipher.encrypt(decrypt, &buffer[..num_read]).unwrap())).is_err() {
                break;
            };
        } else {
            let nonce = Nonce::from_slice(&OsRng.gen::<[u8; 12]>());
            if write_tx.send(Vec::from(cipher.encrypt(nonce, &buffer[..num_read]).unwrap())).is_err() {
                break;
            };
        }
    }

    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new()); // empty vec

    Ok(())
}

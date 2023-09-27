use std::io::Result;
use std::sync::mpsc::{Receiver, Sender};

pub fn stats_loop(
    silent: bool,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // DONE Recieve Vector of bytes from read thread
        let buffer = stats_rx.recv().unwrap();
        let num_bytes = buffer.len();
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{}", total_bytes);
        };

        // DONE Send vector of bytes to write loop
        if write_tx.send(buffer).is_err() {
            break;
        };
        if num_bytes == 0 {
            // if run out of data, cuz buffer is empty after sending to write thread
            break;
        };
    }
    if !silent {
        eprintln!();
    };
    Ok(())
}

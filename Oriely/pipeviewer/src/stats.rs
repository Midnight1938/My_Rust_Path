use std::io::Result;
use std::sync::{Arc, Mutex};

pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // TODO Recieve Vector of bytes from read thread
        let buffer: Vec<u8> = Vec::new(); // ! Temporary
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{}", total_bytes);
        };

        // TODO Send vector of bytes to write lop
        let quit = quit.lock().unwrap();
        if *quit {
            break;
        }
    }
    eprintln!();
    Ok(())
}

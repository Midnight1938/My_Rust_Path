use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::{Arc, Mutex};

pub fn write_loop(outfile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout())) // If the outfile is empty, write to stdout
    };

    loop {
        // TODO Recieve Vector of bytes from stats thread
        let buffer: Vec<u8> = Vec::new(); // ! Temporary
        { // ? Personal scope so that lock is dropped before write
            let quit = quit.lock().unwrap();
            if *quit {
                break;
            }
        }
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(()); // stop cleanly
            }
            return Err(e);
        };
    }

    Ok(()) // Keep the loop going
}

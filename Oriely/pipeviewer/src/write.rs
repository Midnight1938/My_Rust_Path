use std::fs::File;
use std::io::{self, BufWriter, Write, ErrorKind, Result};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool>{

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout())) // If the outfile is empty, write to stdout
    };


    if let Err(e) = writer.write_all(&buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(false); // Return false to stop cleanly
        }
        return Err(e);
    };

    Ok(true) // Keep the loop going
}

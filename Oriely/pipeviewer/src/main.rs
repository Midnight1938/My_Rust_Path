use clap::{App, Arg};
use std::env;
use std::fs::File;
/// self makes it import IO. Read, Write temselves are the traits
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write}; // Import the env module that contains the args function
// Buf Read and Write are used to buffer the input and output. ie to read and write in chunks

const CHUNK_SIZE: usize = 16 * 1024; // A pre defined chunk size of 16KB

fn main() -> Result<()> {
    let matches = App::new("pipeviewer")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .help("Write output to file instead of stdout"),
        )
        .arg(
            Arg::with_name("silent")
                .short('s')
                .long("silent")
                .help("Silence the output"),
        )
        .get_matches(); // Get the matches from the command line
    let infile = matches.value_of("infile").unwrap_or_default(); // Get the infile value from the matches
    let outfile = matches.value_of("outfile").unwrap_or_default(); // Get the outfile value. unwrap_or_default sets default value to empty string
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    }; // Get the silent value from the matches
       // dbg!(infile, outfile, silent);

    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        // Dyn is a smart pointer with a fixed size that places data on a heap
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin())) // If the infile is empty, read from stdin
    };
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout())) // If the outfile is empty, write to stdout
    };
    // dbg!(silent); //! Quick and dirty way to print the value of silent. NOT FOR LOGGING BUT DEBUGGING
    let mut total_bytes = 0; // A variable to store the total number of bytes read
    let mut buffer = [0; CHUNK_SIZE]; // Create a buffer of size CHUNK_SIZE
                                      // Read from stdin of the buffer and store in buffer
    loop {
        // Loop to make the read and write operations continous
        let num_read = match reader.read(&mut buffer) {
            // Match is used to handle the Result type
            Ok(0) => break, // ! Press Ctrl+D to break out
            Ok(n) => n,     // Return the num bytes read
            Err(_) => break,
        };
        total_bytes += num_read; // Add the number of bytes read to total_bytes
        if !silent {
            eprint!("\r{}", total_bytes) // eprintln is used to print to stderr, \r is used to print to the same line
        };
        //? &buffer[..num_read] is used to write the number of bytes read
        // io::stdout().write_all(&buffer[..num_read])? does return an error if the write fails
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            // Write to stdout from the buffer.
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e); // Ignore the error and return an empty Result
                           // Custom Error Handling
                           // eprintln!("OH Nien! Error! {}", e.to_string());
                           // std::process::exit(17);
        };
    }
    if !silent {
        eprintln!("\r{}", total_bytes) // eprintln is used to print to stderr
    };

    Ok(()) // Return an empty Result
}

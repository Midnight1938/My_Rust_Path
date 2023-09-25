use clap::{App, Arg};
use std::env;
/// self makes it import IO. Read, Write temselves are the traits
use std::io::{self, ErrorKind, Read, Result, Write}; // Import the env module that contains the args function

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
    let _infile = matches.value_of("infile"); // Get the infile value from the matches
    let _outfile = matches.value_of("outfile"); // Get the outfile value from the matches
    let _silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    }; // Get the silent value from the matches
       // dbg!(infile, outfile, silent);

    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty(); // Check if the PV_SILENT environment variable is set
                                                                        // dbg!(silent); //! Quick and dirty way to print the value of silent. NOT FOR LOGGING BUT DEBUGGING
    let mut total_bytes = 0; // A variable to store the total number of bytes read
    let mut buffer = [0; CHUNK_SIZE]; // Create a buffer of size CHUNK_SIZE
                                      // Read from stdin of the buffer and store in buffer
    loop {
        // Loop to make the read and write operations continous
        let num_read = match io::stdin().read(&mut buffer) {
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
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
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

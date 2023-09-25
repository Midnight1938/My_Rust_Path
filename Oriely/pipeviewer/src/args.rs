use clap::{App, Arg};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
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
        let infile = matches.value_of("infile").unwrap_or_default().to_string(); // Get the infile value from the matches
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string(); // Get the outfile value. unwrap_or_default sets default value to empty string
        let silent = if matches.is_present("silent") {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        }; // Get the silent value from the matches
           // dbg!(infile, outfile, silent);
        Self {
            infile,
            outfile,
            silent,
        }
    }
}

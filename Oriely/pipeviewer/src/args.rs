use clap::{Arg, Command};
use std::{env};

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("pipeviewer")
            .arg(Arg::new("infile").help("Read from a file instead of stdin"))
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    // .takes_value(true)
                    .help("Write output to file instead of stdout"),
            )
            .arg(
                Arg::new("silent")
                    .short('s')
                    .long("silent")
                    .help("Silence the output"),
            )
            .get_matches(); // Get the matches from the command line
        
        // Get the infile value from the matches
        let infile = matches
            .get_many::<String>("infile")
            .unwrap_or_default()
            .into_iter().map(|s| s.to_string())
            .collect();
        // Get the outfile value. unwrap_or_default sets default value to empty string
        let outfile = matches
            .get_many::<String>("outfile")
            .unwrap_or_default()
            .into_iter().map(|s| s.to_string())
            .collect();
        let silent = if matches.contains_id("silent") {
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

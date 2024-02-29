use clap::{Arg, ArgMatches, Command};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub decrypt: bool,
    pub silence: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches: ArgMatches = Command::new("filencryp")
            .arg(Arg::new("infile").help("Read from a file instead of stdin"))
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .help("Write output to a file instead of stdout"),
            )
            .arg(
                Arg::new("decrypt")
                    .short('d')
                    .long("decrypt")
                    .help("Decrypt Encrypted File"),
            )
            .arg(
                Arg::new("silence")
                    .short('s')
                    .long("silent")
                    .help("Disable Verbosity"),
            )
            .get_matches();

        let infile: String = matches
            .get_many::<String>("infile")
            .unwrap_or_default()
            .map(|s| s.to_string()).collect();
        let outfile: String = matches
            .get_many::<String>("outfile")
            .unwrap_or_default()
            .map(|s| s.to_string()).collect();
        let decrypt = std::env::args().any(|arg| arg == "decrypt");
        let silence: bool = if matches.contains_id("silent") { true } else { !env::var("PV_SILENT").unwrap_or_default().is_empty() };

        Self {
            infile,
            outfile,
            decrypt,
            silence,
        }
    }
}

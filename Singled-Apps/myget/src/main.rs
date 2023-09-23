extern crate clap;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

/*
    Broken tutorial from https://mattgathu.github.io/2017/08/29/writing-cli-app-rust.html
 */

fn main() {
    let matches = App::new("myget")
        .version("0.1.0")
        .author("Midnight")
        .about("A simple wget clone")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("The download URL"),
        )
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("Downloading {}", url);
}

fn create_prog_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(msg);
    match length.is_some() {
        true => bar.set_style(ProgressStyle::default_bar()
            .template("{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}

fn download(target: &str, quiet_mode: bool) -> Result<(), Box<::std::error::Error>> {

    // parse url
    let url = parse_url(target)?;
    let client = Client::new().unwrap();
    let mut resp = client.get(url)?
        .send()
        .unwrap();
    print!(format!("HTTP request sent... {}",
                  style(format!("{}", resp.status())).green()),
          quiet_mode);
    if resp.status().is_success() {

        let headers = resp.headers().clone();
        let ct_len = headers.get::<ContentLength>().map(|ct_len| **ct_len);

        let ct_type = headers.get::<ContentType>().unwrap();

        match ct_len {
            Some(len) => {
                print!(format!("Length: {} ({})",
                      style(len).green(),
                      style(format!("{}", HumanBytes(len))).red()),
                    quiet_mode);
            },
            None => {
                print!(format!("Length: {}", style("unknown").red()), quiet_mode); 
            },
        }

        print!(format!("Type: {}", style(ct_type).green()), quiet_mode);

        let fname = target.split("/").last().unwrap();

        print!(format!("Saving to: {}", style(fname).green()), quiet_mode);

        let chunk_size = match ct_len {
            Some(x) => x as usize / 99,
            None => 1024usize, // default chunk size
        };

        let mut buf = Vec::new();

        let bar = create_progress_bar(quiet_mode, fname, ct_len);

        loop {
            let mut buffer = vec![0; chunk_size];
            let bcount = resp.read(&mut buffer[..]).unwrap();
            buffer.truncate(bcount);
            if !buffer.is_empty() {
                buf.extend(buffer.into_boxed_slice()
                               .into_vec()
                               .iter()
                               .cloned());
                bar.inc(bcount as u64);
            } else {
                break;
            }
        }

        bar.finish();

        save_to_file(&mut buf, fname)?;
    }

    Ok(())

}
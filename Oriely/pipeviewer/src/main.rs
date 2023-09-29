use pipeviewer::{args::Args, read, stats, write}; // args::Args cus binary and lib have same name
use std::io::Result;

use std::sync::mpsc; // multi producer single consumer
use std::thread;
fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    // transmitters and receivers
    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx)); // Move is
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx, write_tx)); // {||} closure is a fn that can capture environment around it
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx)); // python equivalent of closure is lambda

    // Crash if any thread panics
    // .join returns a thread::Result<io::Result<()>>.
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // Return error if any thread is an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}

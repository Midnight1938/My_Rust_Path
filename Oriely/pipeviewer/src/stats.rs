mod timer;

use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};

use std::time::Instant;
use std::io::{self, Result, Stderr, Write};

use timer::Timer;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;

    let start = Instant::now();
    let mut timer = Timer::new();

    let mut stderr = io::stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_sec = (num_bytes as f64) / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.ready {
            // Outputs a second at a time
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_sec,
            ); // :.0 removes fraction
        };

        if num_bytes == 0 {
            // if run out of data, cuz buffer is empty after sending to write thread
            break;
        };
    }
    if !silent {
        eprintln!();
    };
    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes)).with(Color::Magenta);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0}b/s]", rate)).with(Color::Blue);
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate),
    );
    let _ = stderr.flush();
}

trait TimeOutput {
    fn as_time(&self) -> String;
}
impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds) // :02 cuts it to 2 chars
    }
}

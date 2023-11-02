use anyhow::{anyhow, Result};
use borat::{BreakoutRoomInfo, BreakoutRoomStatus};
use colored::Colorize;
use console::Term;
use std::env;

const OPEN_SYMBOL: &str = "✓";
const CLOSED_SYMBOL: &str = "✗";

fn main() -> Result<()> {
    let url =
        env::var("BORAT_URL").map_err(|_| anyhow!("Missing environment variable `BORAT_URL`."))?;

    let stdout = Term::stdout();

    loop {
        let response = reqwest::blocking::get(&url)?.json::<Vec<BreakoutRoomInfo>>()?;
        stdout.clear_screen()?;

        stdout.write_line("Breakout Room Statuses:")?;
        response
            .iter()
            .for_each(|BreakoutRoomInfo { name, status }| {
                let s = match status {
                    BreakoutRoomStatus::Open => format!(" {} {name}", OPEN_SYMBOL.green()),
                    BreakoutRoomStatus::Closed => format!(" {} {name}", CLOSED_SYMBOL.red()),
                };

                stdout.write_line(&s).unwrap();
            });

        // wait 30 seconds before reloading
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}

use anyhow::{anyhow, Result};
use borat::{BreakoutRoomInfo, BreakoutRoomStatus};
use console::Term;
use std::env;

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
                let status_mark = match status {
                    BreakoutRoomStatus::Open => "✓",
                    BreakoutRoomStatus::Closed => "✗",
                };

                let s = format!(" {status_mark} {name}");
                stdout.write_line(&s).unwrap();
            });

        // wait 30 seconds before reloading
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}

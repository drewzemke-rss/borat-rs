use anyhow::{anyhow, Result};
use console::Term;
use serde::Deserialize;
use std::{env, fmt::Display};

#[derive(Debug, Deserialize)]
struct BreakoutRoomInfo {
    name: String,
    status: BreakoutRoomStatus,
}

impl Display for BreakoutRoomInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{}: {}", self.name, self.status);
        s.fmt(f)
    }
}

#[derive(Debug, Deserialize)]
enum BreakoutRoomStatus {
    #[serde(rename = "open")]
    Open,

    #[serde(rename = "closed")]
    Closed,
}

impl Display for BreakoutRoomStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BreakoutRoomStatus::Open => "open",
            BreakoutRoomStatus::Closed => "closed",
        };
        s.fmt(f)
    }
}

#[derive(Debug, Deserialize)]
struct BoratResponse(Vec<BreakoutRoomInfo>);

impl Display for BoratResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(|s| format!("  {s}\n")).collect();

        s.fmt(f)
    }
}

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

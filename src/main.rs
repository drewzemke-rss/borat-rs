use anyhow::{anyhow, Result};
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

    let response = reqwest::blocking::get(url)?.json::<BoratResponse>()?;

    println!("Breakout Room Statuses:");
    println!("{}", response);

    Ok(())
}

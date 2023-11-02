#![warn(clippy::all, clippy::pedantic, clippy::unwrap_used)]

use anyhow::{anyhow, Result};
use borat::{BreakoutRoomInfo, BreakoutRoomStatus};
use clap::Parser;
use colored::Colorize;
use console::Term;
use std::env;

const OPEN_SYMBOL: &str = "✓";
const CLOSED_SYMBOL: &str = "✗";

/// BORAT -- the Breakout Room Availability Tracker
///
/// Displays a live* report of which RSS breakout rooms are available and
/// which are occupied. (*Data may be up to one minute out-of-date.)
///
/// NOTE: You must have the `BORAT_URL` environment variable set to the correct URL.
#[derive(Parser, Debug)]
struct CliArgs {
    /// Keep the output on-screen and automatically refresh every 30 seconds.
    #[arg(short, long)]
    persistent: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let url =
        env::var("BORAT_URL").map_err(|_| anyhow!("Missing environment variable `BORAT_URL`."))?;

    let term = Term::stdout();

    if args.persistent {
        loop {
            term.clear_screen()?;
            poll_and_print(&url, &term)?;

            // wait 30 seconds before reloading
            std::thread::sleep(std::time::Duration::from_secs(30));
        }
    } else {
        poll_and_print(&url, &term)?;
    };

    Ok(())
}

fn poll_and_print(url: &str, term: &Term) -> Result<()> {
    let response = reqwest::blocking::get(url)?.json::<Vec<BreakoutRoomInfo>>()?;

    term.write_line("Breakout Room Statuses:")?;
    for BreakoutRoomInfo { name, status } in response {
        let s = match status {
            BreakoutRoomStatus::Open => format!(" {} {name}", OPEN_SYMBOL.green()),
            BreakoutRoomStatus::Closed => format!(" {} {name}", CLOSED_SYMBOL.red()),
        };

        term.write_line(&s)?;
    }

    Ok(())
}

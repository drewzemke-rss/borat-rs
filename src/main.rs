use anyhow::{anyhow, Result};
use std::env;

fn main() -> Result<()> {
    let url =
        env::var("BORAT_URL").map_err(|_| anyhow!("Missing environment variable `BORAT_URL`."))?;

    let response = reqwest::blocking::get(url)?.text()?;
    println!("response: {}", response);

    Ok(())
}

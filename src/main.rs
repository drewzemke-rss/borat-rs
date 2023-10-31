use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("BORAT_URL")?;

    let response = reqwest::blocking::get(url)?.text()?;
    println!("response: {}", response);

    Ok(())
}

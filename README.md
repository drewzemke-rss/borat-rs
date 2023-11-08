# BORAT: the BreakOut Room Availability Tracker

This is a command-line client for BORAT. 
Not only can you check for open breakout rooms without leaving your desk, you can now do that without even leaving your terminal window!
Surely this is the height of luxury.

## Installation 

If you're part of the Rust masterrace and have `cargo` installed, then it's just:
```shell
cargo install risk_and_safety_borat
```

Otherwise you'll have to use one of the pre-built binaries from the [releases](https://github.com/drewzemke-rss/borat-rs/releases) page. 
Right now those only exist for Intel and Apple Silicon Macs, but let me know if you want something else.

Also, because I'm not going to buy an Apple developer account for this, you'll have to jump through a few hoops here. Sorry.
```shell
# Download the binary with `curl`. Make sure you pick the correct architecture:
#  `x86` is Intel, `aarch` is Apple Silicon. 
# For the output location, choose anywhere you like, preferably somewhere on your $PATH 
#   like `/usr/local/bin/borat`.
curl -L https://github.com/drewzemke-rss/borat-rs/releases/download/0.1.1/borat-aarch64 --output /path/to/borat

# Make it executable.
chmod +x /path/to/borat
```

## Usage

You need to supply the URL for the BORAT API as an environment variable. Ask another RSS dev for it!
```shell
BORAT_URL=<borat-url> /path/to/borat

# For optimal experience, set `BORAT_URL` in your shell's environment variables 
# and store the binary on your $PATH. Then it's just
borat

# Pass a flag to persist the output and refresh every 30 seconds.
borat -p
```

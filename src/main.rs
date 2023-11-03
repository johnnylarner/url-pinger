use clap::Parser;
use reqwest::{self, Error};

/// A simple URL pinger that gives you response times and status codes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct UrlPinger {
    /// Comma separated list of URLs to ping
    #[arg(short, long)]
    urls: String
}


fn main() -> Result<(), Error>  {
    let pinger = UrlPinger::parse();

    for url in pinger.urls.split(",") {
        let body = reqwest::blocking::get(url)?
            .text()?;
        println!("body = {:?}", body)
    }
    Ok(())
    
}

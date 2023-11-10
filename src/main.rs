use clap::Parser;
use web_pinger::UrlPinger;

/// A simple URL pinger that gives you response times and status codes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct UrlParser {
    /// Comma separated list of URLs to ping
    #[arg(short, long)]
    urls: String,
}

fn main() {
    let parser = UrlParser::parse();

    let pinger = UrlPinger::from_comma_seperated_string(&parser.urls);

    let ping_results = pinger.ping_urls();
    for res in ping_results.iter() {
        println!("{:?}", res)
    }
}

use clap::Parser;
use reqwest::Client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    url: String
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = Client::new();
    let result = client.head(args.url).send().await;

    match result {
        Ok(r) => println!("URL returned: {}", r.status()),
        Err(e) => eprintln!("Unable to check URL status or URL doesn't exist. {}", e),
    }
}

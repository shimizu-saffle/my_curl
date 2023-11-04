use reqwest::blocking::Client;
use clap::Parser;

/// This program sends an HTTP request based on the provided method (GET or POST) to the specified URL.
/// It then prints the URL accessed, the response headers, the status code, and the response body.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to which the HTTP request will be sent. This argument is required and should be provided on the command line.
    #[arg(short, long)]
    url: String,

    /// The HTTP method to be used for the request. This argument is required and should be provided on the command line.
    /// Valid methods include "GET", "POST", etc.
    #[arg(short, long)]
    method: String,
}

fn main() {    
    let args = Args::parse();
    let client = Client::new();
    let response = match args.method.as_str() {
        "GET" => {
            match client.get(&args.url).send() {
                Ok(response) => response,
                Err(e) => panic!("GET request failed: {}", e),
            }
        },
        "POST" => {
            match client.post(&args.url).send() {
                Ok(response) => response,
                Err(e) => panic!("POST request failed: {}", e),
            }
        },        
        _ => panic!("Unsupported HTTP method"),
    };

    println!("Access {}", response.url());
    println!("Header {:?}", response.headers());
    println!("Status {}", response.status());
    println!("body = {:?}", response.text());
}

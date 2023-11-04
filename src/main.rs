use reqwest::blocking::Client;
use std::env;

fn main() {    
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let url = args[1].clone();
    let client = Client::new();
    let response = match client.get(url)
    .send() {
        Ok(response) => response,
        Err(e) => panic!("request failed:{}", e)
    };

    println!("Access {}", response.url());
    println!("Header {:?}", response.headers());
    println!("body = {:?}", response.text());
}

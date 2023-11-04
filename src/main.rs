use reqwest::blocking::Client;

fn main() {    
    let client = Client::new();
    let response = match client.get("http://localhost:3000")
    .send() {
        Ok(response) => response,
        Err(e) => panic!("Request failed:{}", e)
    };         

    println!("Access {}", response.url());
    println!("Header {:?}", response.headers());
    println!("body = {:?}", response.text());
}

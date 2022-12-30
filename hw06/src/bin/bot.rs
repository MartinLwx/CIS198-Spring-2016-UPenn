extern crate bbs;
extern crate hyper;
extern crate rustc_serialize;

use std::io::{Read, Write};
use std::net::TcpListener;

use bbs::UserClient;
use bbs::BOT_ADDR;
use hyper::net::HttpsConnector;
use hyper::status::StatusCode;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;

fn get_rnd_number(upper_bound: usize) -> hyper::Result<(StatusCode, usize)> {
    println!("[lib] ready to query the random.org");
    let url = format!(
        "https://www.random.org/integers/?num=1&min=1&max={}&col=1&base=10&format=plain&rnd=new",
        upper_bound,
    );
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut response = client.get(&url).send()?;
    println!("[lib] catch response");
    let mut buf = String::new();
    println!("[lib] get the response from random.org: {}", buf);
    response.read_to_string(&mut buf).unwrap();
    Ok((response.status, buf.trim().parse().unwrap()))
}

fn main() {
    // Create a bot user.
    // TODO
    let bot_user = UserClient::new("bot".to_string(), BOT_ADDR.to_string());

    // Start TcpListener.
    // TODO
    let listener = TcpListener::bind(BOT_ADDR).expect("Start TcpListener failed");
    println!("[Bot] listening on {}...", BOT_ADDR);

    // Listen for incoming TCP connections.
    // For each connection, accept it and read all data from the stream.
    // If it's a 'choose' message, post to the BBS with a response (via the above bot user).
    // TODO
    for stream in listener.incoming() {
        match stream {
            Err(_) => println!("Connection failed"),
            Ok(mut stream) => {
                println!("[Bot] find incoming TCP connections");
                // Read all data from the stream
                let mut buf = String::new();
                stream
                    .read_to_string(&mut buf)
                    .expect("Read TcpStream failed");

                println!("[Bot] read the content {}", buf);
                let tokens = buf.split_whitespace().collect::<Vec<_>>();
                if tokens[0] == "choose" {
                    // Get the random number
                    let rnd_number = get_rnd_number(tokens.len() - 1)
                        .expect("No random number available")
                        .1;
                    println!("[Bot] get the rnd number: {}", rnd_number);
                    let msg = tokens[rnd_number];
                    stream
                        .write(msg.as_bytes())
                        .expect("[Bot] Write to TcpStream failed");
                    println!("[Bot] post the choosen msg back: {}", msg);
                } else {
                    stream
                        .write(&buf.as_bytes())
                        .expect("[Bot] Write to TcpStream failed");
                    println!("[Bot] post the origin msg back: {}", buf);
                }
            }
        }
    }
}

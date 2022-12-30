extern crate bbs;
extern crate hyper;
extern crate rustc_serialize;

use bbs::{Message, UserClient, HTML_ADDR};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // we assume the 1st arg is in ["GET", "POST"]
    // we assume the 2nd arg is the username
    // we assume the 3rd arg is the msg of POST
    match args[1].as_str() {
        "GET" => {
            let client = UserClient::new(args[2].clone(), HTML_ADDR.to_string());
            let content = client.get_content().expect("GET failed");
            println!("Response content: {}", content.1)
        }
        "POST" => {
            let client = UserClient::new(args[2].clone(), HTML_ADDR.to_string());
            client
                .send_msg(Message::new(args[2].clone(), args[3].clone()))
                .expect("POST failed");
            println!("POST success :)")
        }
        _ => println!("Please set 1st args to GET or POST"),
    }
}

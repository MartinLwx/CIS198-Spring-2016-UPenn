extern crate bbs;
extern crate hyper;
extern crate rustc_serialize;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use bbs::Message;
use bbs::{BOT_ADDR, HTML_DATA, HTML_FOOTER, HTML_HEADER, SERVER_ADDR};
use hyper::server::{Request, Response, Server};
use hyper::status::StatusCode;
use rustc_serialize::json;

fn req_handler(mut req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            println!("[Server] a GET request");
            // Read the files [HTML_HEADER, HTML_DATA, HTML_FOOTER] into buf.
            // If HTML_DATA doesn't exist, it should be skipped without failure.
            // Use `try_or_server_err!(expression, res)` instead of `try!(expression)` in
            // order to return an internal server error.
            let mut buf = String::new();
            // Read 3 files and concat their content
            File::open(HTML_HEADER)
                .expect("Open `HTML_HEADER` failed")
                .read_to_string(&mut buf)
                .expect("Error happened when reading HTML_HEADER");
            let data_file = File::open(HTML_DATA);
            if !data_file.is_err() {
                data_file
                    .unwrap()
                    .read_to_string(&mut buf)
                    .expect("Error happened when reading HTML_DATA");
            }
            File::open(HTML_FOOTER)
                .expect("Open `HTML_HEADER` failed")
                .read_to_string(&mut buf)
                .expect("Error happened when reading HTML_FOOTER");

            // And return buf as the response.
            *res.status_mut() = StatusCode::Ok;
            res.send(&buf.as_bytes()).unwrap();
            println!("[Server] finishing sending response for GET")
        }
        hyper::Post => {
            println!("[Server] a POST request");
            // Read the message out of the `req` into a buffer, handle it, and respond with Ok.
            // TODO
            let mut buf = String::new();
            req.read_to_string(&mut buf)
                .expect("Read the POST body failed");

            // Relay to the bot
            println!("[Server] Repaly the content of POST to the bot");
            let mut decoded_res: Message = json::decode(&buf).unwrap();
            let mut stream =
                TcpStream::connect(BOT_ADDR).expect("Connect to the bot's address failed");
            stream
                .write_all(&decoded_res.text.as_bytes())
                .expect("Write to TcpStream failed");
            stream.shutdown(Shutdown::Write).unwrap();

            let mut processed_text = String::new();
            stream
                .read_to_string(&mut processed_text)
                .expect("Get the output of bot failed");
            decoded_res.text = processed_text;

            // Write the msg to the BBS
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(HTML_DATA)
                .unwrap();
            f.write(format!("{}\n", json::encode(&decoded_res).unwrap()).as_bytes())
                .expect("Write HTML_DATA failed");

            *res.status_mut() = StatusCode::Ok;
            res.send(b"").unwrap();
            println!("[Server] finishing sending response for POST")
        }
        _ => *res.status_mut() = StatusCode::ImATeapot,
    }
}

fn main() {
    println!("Listening on {}.", SERVER_ADDR);
    match Server::http(SERVER_ADDR) {
        Ok(server) => match server.handle(req_handler) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        },
        Err(e) => println!("{:?}", e),
    }
}

extern crate hyper;
extern crate rustc_serialize;
extern crate websocket;

mod chatserver;
mod webpage;

fn main() {
    chatserver::start();
    webpage::serve();
}

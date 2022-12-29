use rustc_serialize::json;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use websocket::sync::{Client, Server, Writer};
use websocket::OwnedMessage;

const WS_ADDR: &'static str = "0.0.0.0:1981";

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// Represents a single, atomic action taken by a chat member.
///
/// DO NOT MODIFY: the JavaScript relies on this!
enum ChatAction {
    Connect { addr: String },
    Disconnect { addr: String },
    Msg { user: String, text: String },
}

/// Spawn a WebSocket listener thread.
pub fn start() {
    thread::spawn(listen);
}

/// Create the relay MPSC (multi-producer/single-consumer) channel, spawn the
/// relay thread, then listen for WebSocket clients and spawn their threads.
fn listen() {
    let server = Server::bind(WS_ADDR).unwrap();
    println!("Listening on {}...", WS_ADDR);

    // maintain a live clients list
    let live_clients = Arc::new(Mutex::new(HashMap::new()));

    for request in server.filter_map(Result::ok) {
        let client = request.accept().unwrap();

        let live_clients = live_clients.clone();
        thread::spawn(move || relay_thread(client, live_clients));
    }
}

/// The relay thread handles all `ChatAction`s received on its MPSC channel
/// by sending them out to all of the currently connected clients.
fn relay_thread(
    client: Client<TcpStream>,
    live_clients: Arc<Mutex<HashMap<SocketAddr, Writer<TcpStream>>>>,
) {
    let peer_addr = client.peer_addr().unwrap();

    let (tx, rx) = mpsc::channel();
    let cloned_live_clients = Arc::clone(&live_clients);
    thread::spawn(move || client_thread(peer_addr, rx, cloned_live_clients));

    let (mut receiver, sender) = client.split().unwrap();
    live_clients.lock().unwrap().insert(peer_addr, sender);

    // add current user to global map
    println!("Accept a connection from: {}", peer_addr);
    {
        let connect_msg = json::encode(&ChatAction::Connect {
            addr: peer_addr.to_string(),
        })
        .unwrap();
        tx.send(connect_msg).unwrap();
    }
    for message in receiver.incoming_messages() {
        let message = message.unwrap();
        match message {
            OwnedMessage::Text(text) => {
                tx.send(text).unwrap();
            }
            _ => break,
        }
    }
    {
        let disconnect_msg = json::encode(&ChatAction::Disconnect {
            addr: peer_addr.to_string(),
        })
        .unwrap();
        tx.send(disconnect_msg).unwrap();
    }
}

/// Each client thread waits for input (or disconnects) from its respective clients
/// and relays the appropriate messages via the relay MPSC channel.
///
/// The messages received-from and sent-to the client should be JSON objects with the same
/// form as rustc_serialize's serialization of the `ChatAction` type.
///
/// * If the client connects, a `ChatAction::Connect` will be relayed with their IP address.
///
/// * If the client disconnects, a `ChatAction::Disconnect` will be relayed with their IP address.
///
/// * If the client sends any other message (i.e. `ChatAction::Msg`), it will be relayed verbatim.
///   (But you should still deserialize and reserialize the `ChatAction` to make sure it is valid!)
fn client_thread(
    current_peer_addr: SocketAddr,
    receiver: Receiver<String>,
    live_clients: Arc<Mutex<HashMap<SocketAddr, Writer<TcpStream>>>>,
) {
    for msg in receiver {
        let chat_msg: ChatAction = json::decode(&msg).unwrap();
        if let ChatAction::Disconnect { addr } = chat_msg {
            live_clients.lock().unwrap().remove(&current_peer_addr);
        }
        let mut live_clients = live_clients.lock().unwrap();
        for (_, wr) in live_clients.iter_mut() {
            println!("Send msg {}", msg);
            wr.send_message(&OwnedMessage::Text(msg.clone())).unwrap();
        }
    }
}

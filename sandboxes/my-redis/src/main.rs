/*
redis server needs to accept inbound TCP sockets
*/

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is 
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // The 'Connection' lets us read/write redis **frames** instead of 
    // byte streams. The 'Connection' type is defined by mini-redis.

    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();

    }
}

// a tokio task is an sync green thread - theyre made by passing an async blok to tokio spaw nand tokio spawn function returns a JoinHandle, which caller may use to interact with sapwned task
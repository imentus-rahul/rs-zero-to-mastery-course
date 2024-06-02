// use std::thread::JoinHandle;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // let mut tasks = Vec::new();

    let tcp_listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (sender_transmitter, receiver_transmitter) = broadcast::channel(10);
    loop {
        let (mut tcp_stream_socket, socket_addr) = tcp_listener.accept().await.unwrap();
        let sender_transmitter = sender_transmitter.clone();
        let mut receiver_transmitter = sender_transmitter.subscribe();
        tokio::spawn(async move {
            let (reader_half, mut writer_half) = tcp_stream_socket.split();
            let mut reader_half = BufReader::new(reader_half);
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader_half.read_line(&mut line) => {
                        if result.unwrap() == 0{
                            break;
                        }
                        sender_transmitter.send((line.clone(), socket_addr)).unwrap();
                        line.clear();
                    }
                    result = receiver_transmitter.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if socket_addr!= other_addr{
                            writer_half.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
        // for task:JoinHandle<()> in tasks {
        //     task.await.unwrap();
        // }
    }
    // for task in tasks {
    //     task.await.unwrap();
    // }
}

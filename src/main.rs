use utilities::*;

pub mod metrics;
pub mod utilities;

pub const SWARM_SIZE: usize = 15;
pub type SwarmPos = [(f64, f64); SWARM_SIZE];
pub type SwarmMetric = [f64; 9];

fn main() {
    let mut controller_cmd = std::env::args().collect::<Vec<String>>();
    controller_cmd.remove(0);
    let eval: Evaluator = Evaluator::new();
    println!("{}", eval.eval_controller(controller_cmd));
}

// use futures_util::{SinkExt, StreamExt};
// use tokio::io::{AsyncWriteExt, Result};
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
//
// #[tokio::main]
// pub async fn main() {
//     let url = url::Url::parse("ws://localhost:3000").unwrap();
//
//     let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
//     println!("WebSocket handshake has been successfully completed");
//
//     let (mut write, mut read) = ws_stream.split();
//
//     // let read_future = read.for_each(|message| async {
//     //     println!("receiving...");
//     //     let data = message.unwrap().into_data();
//     //
//     //     tokio::io::stdout().write(&data).await.unwrap();
//     //     println!("received...");
//     // });
//     //
//     // read_future.await;
//
//     loop {
//         let _ = write.send(Message::Text("hello".to_string())).await;
//         let message = read.next().await.unwrap();
//         println!("Received message: = {:?}", message);
//     }
// }

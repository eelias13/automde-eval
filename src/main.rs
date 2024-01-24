use utilities::*;

pub mod metrics;
pub mod utilities;

pub const SWARM_SIZE: usize = 15;
pub type SwarmPos = [(f64, f64); SWARM_SIZE];
pub type SwarmMetric = [f64; 9];

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    if args.len() == 0 {
        println!("no arguments passed");
        println!("\nvalid arguments are:");
        println!("\t--eval-controller <FSM>");
        println!("\t--get-metrics -s <seed> <FSM>");
        return;
    }

    let eval: Evaluator = Evaluator::new();

    if args[0] == "--eval-controller" {
        args.remove(0);
        print!("{}", eval.eval_controller(args));
        return;
    }

    if args[0] == "--get-metrics" {
        if args.len() < 4 {
            println!("to vew arguments supplied");
            println!("\nvalid arguments are:");
            println!("\t--eval-controller <FSM>");
            println!("\t--get-metrics -s <seed> <FSM>");
            return;
        }

        if args[1] != "-s" {
            println!("expected -s but got {}", args[1]);
            println!("\nvalid arguments are:");
            println!("\t--eval-controller <FSM>");
            println!("\t--get-metrics -s <seed> <FSM>");
            return;
        }

        if let Ok(seed) = args[2].parse::<u32>() {
            args.remove(0);
            args.remove(0);
            args.remove(0);
            print!("{:?}", eval.eval_all(args, vec![seed as i32]));
            return;
        }

        println!("expected <seed> but got {}", args[2]);
        println!("\nvalid arguments are:");
        println!("\t--eval-controller <FSM>");
        println!("\t--get-metrics -s <seed> <FSM>");
        return;
    }

    println!("unexpected argument {}", args[0]);
    println!("\nvalid arguments are:");
    println!("\t--eval-controller <FSM>");
    println!("\t--get-metrics -s <seed> <FSM>");
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

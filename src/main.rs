use metrics::to_metic;
use rand::Rng;
use utilities::*;

pub mod metrics;
pub mod utilities;

pub const SWARM_SIZE: usize = 15;
pub type SwarmPos = [(f64, f64); SWARM_SIZE];
pub type SwarmMetric = [f64; 9];

fn main() {
    let real = "--nstates 4 --s0 5 --rep0 4.85 --n0 4 --n0x0 1 --c0x0 0 --p0x0 0.96 --n0x1 0 --c0x1 0 --p0x1 0.36 --n0x2 2 --c0x2 2 --p0x2 0.4 --n0x3 2 --c0x3 3 --p0x3 7 --w0x3 5.59 --s1 2 --n1 3 --n1x0 2 --c1x0 4 --p1x0 6 --w1x0 19.16 --n1x1 1 --c1x1 3 --p1x1 10 --w1x1 7.73 --n1x2 1 --c1x2 4 --p1x2 7 --w1x2 6.72 --s2 3 --n2 3 --n2x0 0 --c2x0 0 --p2x0 0.17 --n2x1 1 --c2x1 2 --p2x1 0.65 --n2x2 2 --c2x2 4 --p2x2 9 --w2x2 5.19 --s3 0 --rwm3 61 --n3 2 --n3x0 0 --c3x0 0 --p3x0 0.83 --n3x1 2 --c3x1 4 --p3x1 4 --w3x1 14.54";
    let explore = "--nstates 1 --s0 0 --rwm0 50";
    let random = "--nstates 4 --s0 4 --att0 4.08 --n0 3 --n0x0 2 --c0x0 1 --p0x0 0.95 --n0x1 2 --c0x1 4 --p0x1 1 --w0x1 16.11 --n0x2 1 --c0x2 3 --p0x2 5 --w0x2 1.13 --s1 1 --n1 1 --n1x0 0 --c1x0 3 --p1x0 10 --w1x0 10.7 --s2 0 --rwm2 69 --n2 3 --n2x0 2 --c2x0 1 --p2x0 0.76 --n2x1 2 --c2x1 5 --p2x1 0.25 --n2x2 1 --c2x2 0 --p2x2 0.47 --s3 0 --rwm3 78 --n3 4 --n3x0 0 --c3x0 4 --p3x0 8 --w3x0 1.53 --n3x1 1 --c3x1 0 --p3x1 0.26 --n3x2 2 --c3x2 4 --p3x2 4 --w3x2 5.4 --n3x3 2 --c3x3 4 --p3x3 10 --w3x3 14.41";

    let random = random
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let real = real
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let explore = explore
        .split(" ")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();

    let mut rng = rand::thread_rng();
    let mut seeds: Vec<i32> = Vec::new();
    for _ in 0..3 {
        seeds.push(rng.gen_range(0..0x7FFFFFFF));
    }

    // let eval: Evaluator = Evaluator::new();
    // let random = eval.eval(random, seeds[0]);
    // let exp = eval.eval(exp, seeds[0]);
    // let real = eval.eval(real, seeds[0]);
    // let real_exp = get_real_bot_data(1200)
    //dbg!(real);
    //dbg!(exp);
    //dbg!(random);

    let [real_sim, real_exp, explore, random] =
        eval((real, seeds[0]), (explore, seeds[1]), (random, seeds[2]));

    //  let real_sim = dist(real_sim, real_exp);
    //  let explore = dist(explore, real_exp);
    //  let random = dist(random, real_exp);

    dbg!(real_sim);
    dbg!(real_exp);
    dbg!(explore);
    dbg!(random);
}

fn eval(
    real: (Vec<String>, i32),
    explore: (Vec<String>, i32),
    random: (Vec<String>, i32),
) -> [SwarmMetric; 4] {
    let eval: Evaluator = Evaluator::new();
    let real_sim = eval.run_experiment(real.0, real.1);
    let explore = eval.run_experiment(explore.0, explore.1);
    let random = eval.run_experiment(random.0, random.1);
    let real_exp = get_real_bot_data(1200);

    assert_eq!(real_sim.len(), 1200);
    assert_eq!(explore.len(), 1200);
    assert_eq!(random.len(), 1200);
    assert_eq!(real_exp.len(), 1200);

    let swarm_mode_dist = eval.swarm_mode_dist;
    let density_radius = eval.density_radius;

    let real_sim = to_metic(&real_sim, swarm_mode_dist, density_radius);
    let real_exp = to_metic(&real_exp, swarm_mode_dist, density_radius);
    let explore = to_metic(&explore, swarm_mode_dist, density_radius);
    let random = to_metic(&random, swarm_mode_dist, density_radius);

    let min = eval.metics_norm_min;
    let max = eval.metics_norm_max;

    // let real_sim = real_sim
    //     .iter()
    //     .zip(real_exp.iter())
    //     .map(|(sim, exp)| dist(sim.to_owned(), exp.clone()))
    //     .collect::<Vec<_>>();
    // let explore = explore
    //     .iter()
    //     .zip(real_exp.iter())
    //     .map(|(explore, exp)| dist(explore.to_owned(), exp.clone()))
    //     .collect::<Vec<_>>();
    // let random = random
    //     .iter()
    //     .zip(real_exp.iter())
    //     .map(|(random, exp)| dist(random.to_owned(), exp.clone()))
    //     .collect::<Vec<_>>();

    // let real_sim = real_sim
    //     .iter()
    //     .map(|x| norm(x.to_owned(), min, max))
    //     .collect();
    // let real_exp = real_exp
    //     .iter()
    //     .map(|x| norm(x.to_owned(), min, max))
    //     .collect();
    // let explore = explore
    //     .iter()
    //     .map(|x| norm(x.to_owned(), min, max))
    //     .collect();
    // let random = random
    //     .iter()
    //     .map(|x| norm(x.to_owned(), min, max))
    //     .collect();

    let real_sim = sum(real_sim);
    let real_exp = sum(real_exp);
    let exp = sum(explore);
    let random = sum(random);

    return [real_sim, real_exp, exp, random];
}

fn dist(sm1: SwarmMetric, sm2: SwarmMetric) -> SwarmMetric {
    let mut dist = SwarmMetric::default();
    for i in 0..dist.len() {
        dist[i] = (sm1[i].powi(2) - sm2[i].powi(2)).sqrt();
    }
    return dist;
}

fn sum(metrics: Vec<SwarmMetric>) -> SwarmMetric {
    let mut sum = SwarmMetric::default();
    let len = metrics.len() as f64;

    for m in metrics {
        for i in 0..sum.len() {
            sum[i] += m[i];
        }
    }

    for i in 0..sum.len() {
        sum[i] /= len;
    }
    return sum;
}

fn norm(metrics: SwarmMetric, min: SwarmMetric, max: SwarmMetric) -> SwarmMetric {
    let mut norm = SwarmMetric::default();
    // (x - x_min) / (x_max - x_min)
    for i in 0..norm.len() {
        norm[i] = (metrics[i] - min[i]) / (max[i] - min[i]);
    }
    return norm;
}

// fn main() {
//     let mut args = std::env::args().collect::<Vec<String>>();
//     args.remove(0);
//     if args.len() == 0 {
//         println!("no arguments passed");
//         println!("\nvalid arguments are:");
//         println!("\t--eval-controller <FSM>");
//         println!("\t--get-metrics -s <seed> <FSM>");
//         return;
//     }
//
//     let eval: Evaluator = Evaluator::new();
//
//     if args[0] == "--eval-controller" {
//         args.remove(0);
//         print!("{}", eval.eval_controller(args));
//         return;
//     }
//
//     if args[0] == "--get-metrics" {
//         if args.len() < 4 {
//             println!("to vew arguments supplied");
//             println!("\nvalid arguments are:");
//             println!("\t--eval-controller <FSM>");
//             println!("\t--get-metrics -s <seed> <FSM>");
//             return;
//         }
//
//         if args[1] != "-s" {
//             println!("expected -s but got {}", args[1]);
//             println!("\nvalid arguments are:");
//             println!("\t--eval-controller <FSM>");
//             println!("\t--get-metrics -s <seed> <FSM>");
//             return;
//         }
//
//         if let Ok(seed) = args[2].parse::<u32>() {
//             args.remove(0);
//             args.remove(0);
//             args.remove(0);
//             print!("{:?}", eval.eval_all(args, vec![seed as i32]));
//             return;
//         }
//
//         println!("expected <seed> but got {}", args[2]);
//         println!("\nvalid arguments are:");
//         println!("\t--eval-controller <FSM>");
//         println!("\t--get-metrics -s <seed> <FSM>");
//         return;
//     }
//
//     println!("unexpected argument {}", args[0]);
//     println!("\nvalid arguments are:");
//     println!("\t--eval-controller <FSM>");
//     println!("\t--get-metrics -s <seed> <FSM>");
// }

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

use crate::{metrics::metric_dist, SwarmMetric, SwarmPos, SWARM_SIZE};
use rand::Rng;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tokio::runtime::Builder;

#[derive(Debug, Clone)]
pub struct Evaluator {
    automode_exe: String,
    scenario: String,
    experiment_len: usize,
    num_of_experiments: usize,
    save_probability: f64,
    swarm_mode_dist: f64,
    density_radius: f64,
    real_pos: Vec<SwarmPos>,
    metics_norm_min: SwarmMetric,
    metics_norm_max: SwarmMetric,
    db_path: String,
    counter: Arc<Mutex<usize>>,
}

impl Evaluator {
    pub fn new() -> Self {
        let automode_exe = env!("AUTOMODE_EXE").to_string();
        let scenario = env!("SCENARIO").to_string();
        let experiment_len = env!("EXPERIMENT_LEN").parse::<usize>().unwrap();
        let num_of_experiments = env!("NUM_OF_EXPERIMENT").parse::<usize>().unwrap();
        let db_path = env!("DB_PATH").to_string();
        let save_probability = env!("SAVE_PROBABILITY").parse::<f64>().unwrap();
        let swarm_mode_dist = env!("SWARM_MODE_DIST").parse::<f64>().unwrap();
        let density_radius = env!("DENSITY_RADIUS").parse::<f64>().unwrap();
        let real_pos = get_real_bot_data(experiment_len);
        let [metics_norm_min, metics_norm_max] = get_metics_normalization();
        let counter = Arc::new(Mutex::new(0));

        let db_con: sqlite::Connection = sqlite::open(&db_path).unwrap();

        let query =
            "CREATE TABLE IF NOT EXISTS data (controller_cmd TEXT, seeds TEXT, metric_norm TEXT);";
        db_con.execute(query).unwrap();

        assert!(save_probability <= 1.0);
        return Self {
            counter,
            db_path,
            automode_exe,
            scenario,
            experiment_len,
            num_of_experiments,
            save_probability,
            swarm_mode_dist,
            density_radius,
            real_pos,
            metics_norm_min,
            metics_norm_max,
        };
    }

    fn save_data(&self, controller_cmd: Vec<String>, seeds: Vec<i32>, metric_norm: SwarmMetric) {
        let db_con: sqlite::Connection = sqlite::open(&self.db_path).unwrap();

        let controller_cmd = controller_cmd.join(" ");
        let seeds = seeds
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<String>>()
            .join(", ");
        let metric_norm = metric_norm
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!("INSERT INTO data VALUES ('{controller_cmd}', '{seeds}', '{metric_norm}');",);
        db_con.execute(query).unwrap();
    }

    pub fn get_command(&self, seed: i32, controller_cmd: Vec<String>) -> std::process::Output {
        return Command::new(self.automode_exe.clone())
            .arg("-n")
            .arg("-c")
            .arg(self.scenario.clone())
            .arg("--seed")
            .arg(format!("{}", seed))
            .arg("--fsm-config")
            .args(controller_cmd)
            .output()
            .expect("failed to execute experiment");
    }

    pub fn eval_controller(&self, controller_cmd: Vec<String>) -> f64 {
        //        let mut counter = self.counter.lock().unwrap();
        //        *counter += 1;
        //        let counter = *counter;

        let mut rng = rand::thread_rng();

        let mut seeds: Vec<i32> = Vec::with_capacity(self.num_of_experiments);
        for _ in 0..self.num_of_experiments {
            seeds.push(rng.gen_range(0..0x7FFFFFFF));
        }

        let metics = self.eval_all(controller_cmd.clone(), seeds.clone());
        let mut metric_norm = SwarmMetric::default();

        // (x - x_min) / (x_max - x_min)
        for i in 0..metric_norm.len() {
            metric_norm[i] = (metics[i] - self.metics_norm_min[i])
                / (self.metics_norm_max[i] - self.metics_norm_min[i]);
        }

        if self.save_probability > rng.gen_range(0.0..1.0) {
            self.save_data(controller_cmd, seeds, metric_norm);
        }

        let mut sum = 0.0;
        for val in metric_norm {
            sum += val;
        }

        return sum / SwarmMetric::default().len() as f64;
    }

    pub fn eval_all(&self, controller_cmd: Vec<String>, seeds: Vec<i32>) -> SwarmMetric {
        let num_of_experiments = seeds.len();

        let runtime = Builder::new_multi_thread()
            .worker_threads(num_of_experiments)
            .enable_all()
            .build()
            .unwrap();
        let mut handles = Vec::with_capacity(num_of_experiments);
        for seed in seeds {
            handles.push(runtime.spawn(self.clone().eval(controller_cmd.clone(), seed)));
        }

        let metrics_data: Vec<Result<SwarmMetric, _>> =
            runtime.block_on(futures::future::join_all(handles));

        let mut result = SwarmMetric::default();
        for metrics in metrics_data {
            if let Ok(val) = metrics {
                for i in 0..val.len() {
                    result[i] += val[i];
                }
            }
        }

        for i in 0..result.len() {
            result[i] /= num_of_experiments as f64;
        }

        return result;
    }

    async fn eval(self, controller_cmd: Vec<String>, seed: i32) -> SwarmMetric {
        let sim_pos = self.run_experiment(controller_cmd, seed);

        assert_eq!(sim_pos.len(), self.experiment_len);

        let metrics_dist = metric_dist(
            &self.real_pos,
            &sim_pos,
            self.swarm_mode_dist,
            self.density_radius,
        );

        return metrics_dist;
    }

    pub fn run_experiment(&self, controller_cmd: Vec<String>, seed: i32) -> Vec<SwarmPos> {
        let output = self.get_command(seed, controller_cmd);

        let out_bytes = output.stdout;
        let out_str = String::from_utf8(out_bytes)
            .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
            .unwrap();

        let mut swarm_pos = Vec::new();
        let mut current_pos = SwarmPos::default();

        for line in out_str.split("\n") {
            if line.chars().nth(0) == Some('%') && line.chars().nth(1) == Some('!') {
                let mut line_it = line.split(" ").collect::<Vec<&str>>().into_iter();
                assert_eq!(line_it.next(), Some("%!"));

                let mut i = line_it.next().unwrap().split(":");
                let mut x = line_it.next().unwrap().split(":");
                let mut y = line_it.next().unwrap().split(":");

                assert_eq!(i.next(), Some("i"));
                assert_eq!(x.next(), Some("x"));
                assert_eq!(y.next(), Some("y"));

                let i = i.next().unwrap().parse::<usize>().unwrap();
                let x = x.next().unwrap().parse::<f64>().unwrap();
                let y = y.next().unwrap().parse::<f64>().unwrap();

                current_pos[i] = (x, y);
                if i == 14 {
                    swarm_pos.push(current_pos)
                }
            }
        }

        return swarm_pos;
    }
}

fn get_metics_normalization() -> [SwarmMetric; 2] {
    let mut line_it = include_str!("metics_normalization.csv")
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter();

    let mut max = SwarmMetric::default();
    let mut i = 0;
    for val in line_it.next().unwrap().split(",") {
        assert!(i < SwarmMetric::default().len());
        max[i] = val.trim().parse::<f64>().unwrap();
        i += 1;
    }

    let mut min = SwarmMetric::default();
    let mut i = 0;
    for val in line_it.next().unwrap().split(",") {
        assert!(i < SwarmMetric::default().len());
        min[i] = val.trim().parse::<f64>().unwrap();
        i += 1;
    }
    assert_eq!(line_it.next(), None);

    return [min, max];
}

fn get_real_bot_data(experiment_len: usize) -> Vec<SwarmPos> {
    let mut line_it = include_str!("all_bot_pos.csv")
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter();

    let head = line_it
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    assert_eq!(head.len(), 2 * SWARM_SIZE);

    let mut bot_pos = Vec::new();

    for line in line_it {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let vals = line
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<f64>>();

        assert_eq!(vals.len(), 2 * SWARM_SIZE);

        let mut data = SwarmPos::default();
        for (i, &val) in vals.iter().enumerate() {
            if i % 2 == 0 {
                data[i / 2].0 = val;
            } else {
                data[(i - 1) / 2].1 = val;
            }
        }
        bot_pos.push(data)
    }

    assert!(bot_pos.len() >= experiment_len);
    if bot_pos.len() >= experiment_len {
        let delta = bot_pos.len() - experiment_len;

        let mut temp = bot_pos.iter();
        if delta % 2 == 1 {
            assert_ne!(temp.next(), None);
        }

        for _ in 0..(delta / 2) {
            assert_ne!(temp.next(), None);
        }

        let mut temp = temp.map(|x| x.to_owned()).collect::<Vec<SwarmPos>>();
        for _ in 0..(delta / 2) {
            assert_ne!(temp.pop(), None);
        }
        bot_pos = temp;
    }
    assert_eq!(bot_pos.len(), experiment_len);

    return bot_pos;
}

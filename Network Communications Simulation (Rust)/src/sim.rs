use crate::geom::{euclidean_distance, Point};
use crate::rf::berg_diffraction_distance;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Exp, Poisson};
use rand::distributions::Distribution;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
pub struct ManhattanLayout {
    pub avenues: Vec<f64>,
    pub streets: Vec<f64>,
    pub base_stations: Vec<Point>,
    pub ave_counts: Vec<usize>,
}

pub fn generate_manhattan(
    size: f64,
    lambda_ave: f64,
    lambda_st: f64,
    lambda_base: f64,
    seed: u64,
    create_base_stations: bool,
) -> ManhattanLayout {
    let mut rng = StdRng::seed_from_u64(seed);
    let mean_ave = (size * lambda_ave).max(0.0);
    let mean_st = (size * lambda_st - 1.0).max(0.0);
    let num_avenues = if mean_ave > 0.0 {
        Poisson::new(mean_ave).unwrap().sample(&mut rng) as usize
    } else { 0 };
    let num_streets = if mean_st > 0.0 {
        Poisson::new(mean_st).unwrap().sample(&mut rng) as usize + 1
    } else { 1 }; // include y=0

    let mut avenues = Vec::with_capacity(num_avenues);
    for _ in 0..num_avenues {
        let x = rng.gen::<f64>() * size - size / 2.0;
        avenues.push(x);
    }

    let mut streets = Vec::with_capacity(num_streets);
    streets.push(0.0);
    for _ in 1..num_streets {
        let y = rng.gen::<f64>() * size - size / 2.0;
        streets.push(y);
    }

    let mut base_stations = Vec::new();
    let mut ave_counts = Vec::with_capacity(num_avenues);

    if create_base_stations {
        let mean_bs = (lambda_base * size).max(0.0);
        let poisson_bs = if mean_bs > 0.0 { Some(Poisson::new(mean_bs).unwrap()) } else { None };
        for &ave in &avenues {
            let count = poisson_bs.as_ref().map(|p| p.sample(&mut rng) as usize).unwrap_or(0);
            ave_counts.push(count);
            for _ in 0..count {
                let y = rng.gen::<f64>() * size - size / 2.0;
                base_stations.push(Point { x: ave, y });
            }
        }
        for &st in &streets {
            let count = poisson_bs.as_ref().map(|p| p.sample(&mut rng) as usize).unwrap_or(0);
            for _ in 0..count {
                let x = rng.gen::<f64>() * size - size / 2.0;
                base_stations.push(Point { x, y: st });
            }
        }
    } else {
        ave_counts.resize(num_avenues, 0);
    }

    ManhattanLayout { avenues, streets, base_stations, ave_counts }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationData {
    pub source_power: f64,      // transmitted power (linear units)
    pub receiver: Point,        // will be set each sim iteration
    pub alpha: f64,             // path loss exponent
    pub a: f64,                 // path loss coefficient
    pub fading_mean: f64,       // mean for exponential fading
    pub noise_power: f64,       // linear units
    pub base_stations: Vec<Point>,
    pub penetration_loss: f64,  // per building attenuation (0..1)
    pub avenues: Vec<f64>,
    pub streets: Vec<f64>,
    pub use_nlos: bool,
    pub use_diffraction: bool,
    pub size: f64,
    pub path_loss_nlos: bool,
    pub diffraction_order: i32,
    pub ave_counts: Vec<usize>,
    pub connect_to_nlos: bool,
    pub lambda_ave: f64,
    pub lambda_st: f64,
    pub lambda_base: f64,
    pub create_base_stations: bool,
    // optimization-related
    pub computation_nodes: usize, // number of points per road for fitness
    pub threshold_db: f64,        // threshold in dB for fitness
}

pub fn small_scale_fading_exp(rng: &mut impl Rng, mean_fade: f64) -> f64 {
    let exponential = Exp::new(1.0 / mean_fade).unwrap(); // Exp with mean=mean_fade
    let uniform_sample: f64 = rng.gen();
    exponential.inverse_cdf(uniform_sample)
}

pub fn num_roads_crossed(data: &SimulationData, transmitter: Point) -> i32 {
    let mut num_roads = 0;
    let receiver_x = data.receiver.x;
    let receiver_y = data.receiver.y;
    let transmitter_x = transmitter.x;
    let transmitter_y = transmitter.y;
    for &avenue in &data.avenues {
        if (receiver_x < avenue && avenue < transmitter_x) || (transmitter_x < avenue && avenue < receiver_x) { num_roads += 1; }
    }
    for &street in &data.streets {
        if (receiver_y < street && street < transmitter_y) || (transmitter_y < street && street < receiver_y) { num_roads += 1; }
    }
    num_roads
}

pub fn power_los_linear(rng: &mut impl Rng, data: &SimulationData, transmitter: Point) -> f64 {
    let distance = euclidean_distance(data.receiver, transmitter);
    let path_loss = data.a * distance.powf(-data.alpha);
    let received_power = data.source_power * small_scale_fading_exp(rng, data.fading_mean) * path_loss;
    received_power.min(data.source_power)
}

pub fn power_nlos_linear(rng: &mut impl Rng, data: &SimulationData, transmitter: Point) -> f64 {
    let building_count = 1 + num_roads_crossed(data, transmitter);
    let mut received_power = data.source_power * small_scale_fading_exp(rng, data.fading_mean)
        * data.penetration_loss.powi(building_count);
    if data.path_loss_nlos {
        let distance = euclidean_distance(data.receiver, transmitter);
        received_power *= data.a * distance.powf(-data.alpha);
    }
    received_power.min(data.source_power)
}

pub fn diffraction_power_linear(data: &SimulationData, transmitter: Point) -> f64 {
    if data.diffraction_order <= 0 { return 0.0; }
    let vertical_distance = (data.receiver.y - transmitter.y).abs();
    let horizontal_distance = (data.receiver.x - transmitter.x).abs();
    let effective_distance = berg_diffraction_distance(vertical_distance, horizontal_distance);
    let received_power = data.a * effective_distance.powf(-data.alpha);
    received_power.min(data.source_power)
}

fn sinr_linear(useful_power: f64, noise_power: f64, total_interference: f64) -> f64 {
    if useful_power == 0.0 { return 0.0; }
    let sinr = useful_power / (noise_power + total_interference - useful_power);
    if sinr < 0.0 { 0.0 } else { sinr }
}

pub fn simulate_coverage_ccdf(mut data: SimulationData, simulations: usize, num_bins: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let mut results_db: Vec<f64> = Vec::with_capacity(simulations);

    // Progress bar for simulation iterations
    let pb = ProgressBar::new(simulations as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) - simulating CCDF")
        .unwrap()
        .progress_chars("=>-"));

    for iteration_idx in 0..simulations {
        // Regenerate a fresh Manhattan layout each simulation
        let layout = generate_manhattan(
            data.size,
            data.lambda_ave,
            data.lambda_st,
            data.lambda_base,
            seed + iteration_idx as u64,
            data.create_base_stations,
        );
        data.avenues = layout.avenues;
        data.streets = layout.streets;
        data.base_stations = layout.base_stations;
        data.ave_counts = layout.ave_counts;

        // Place user at (0,0) each time, ensuring y=0 street exists from generator
        data.receiver = Point { x: 0.0, y: 0.0 };

        let mut useful_power = 0.0;
        let mut total_interference = 0.0;
        let num_avenue_bases: usize = if data.diffraction_order > 0 && !data.ave_counts.is_empty() {
            data.ave_counts.iter().sum()
        } else { 0 };

        let mut rng = StdRng::seed_from_u64(seed ^ (iteration_idx as u64 + 1));

        for (base_idx, &base_station) in data.base_stations.iter().enumerate() {
            let same_street = base_station.x == data.receiver.x || base_station.y == data.receiver.y;
            if same_street {
                let power = power_los_linear(&mut rng, &data, base_station);
                total_interference += power;
                if power > useful_power { useful_power = power; }
            } else if data.use_nlos {
                dbg!(&data);
                let power = power_nlos_linear(&mut rng, &data, base_station);
                total_interference += power;
                if power > useful_power && data.connect_to_nlos { useful_power = power; }
            }
            if !same_street && data.diffraction_order > 0 && data.use_diffraction && base_idx < num_avenue_bases {
                dbg!(&data);
                let power = diffraction_power_linear(&data, base_station);
                total_interference += power;
                if power > useful_power && data.connect_to_nlos { useful_power = power; }
            }
        }

        let sinr = sinr_linear(useful_power, data.noise_power, total_interference);
        results_db.push(10.0 * sinr.log10());

        // update progress bar
        pb.inc(1);
    }

    pb.finish_with_message("CCDF simulation complete");

    crate::metrics::ccdf(&results_db, num_bins)
}

pub fn simulate_average_sinr(data: &mut SimulationData, simulations: usize, seed: u64) -> f64 {
    let mut results_linear: Vec<f64> = Vec::with_capacity(simulations);

    for iteration_idx in 0..simulations {
        // Regenerate a fresh Manhattan layout each simulation
        let layout = generate_manhattan(
            data.size,
            data.lambda_ave,
            data.lambda_st,
            data.lambda_base,
            seed + iteration_idx as u64,
            data.create_base_stations,
        );
        data.avenues = layout.avenues;
        data.streets = layout.streets;
        data.base_stations = layout.base_stations;
        data.ave_counts = layout.ave_counts;

        // Place user at (0,0) each time
        data.receiver = Point { x: 0.0, y: 0.0 };

        let mut useful_power = 0.0;
        let mut total_interference = 0.0;
        let num_avenue_bases: usize = if data.diffraction_order > 0 && !data.ave_counts.is_empty() {
            data.ave_counts.iter().sum()
        } else { 0 };

        let mut rng = StdRng::seed_from_u64(seed ^ (iteration_idx as u64 + 1));

        for (base_idx, &base_station) in data.base_stations.iter().enumerate() {
            let same_street = base_station.x == data.receiver.x || base_station.y == data.receiver.y;
            if same_street {
                let power = power_los_linear(&mut rng, &data, base_station);
                total_interference += power;
                if power > useful_power { useful_power = power; }
            } else if data.use_nlos {
                let power = power_nlos_linear(&mut rng, &data, base_station);
                total_interference += power;
                if power > useful_power && data.connect_to_nlos { useful_power = power; }
            }
            if !same_street && data.diffraction_order > 0 && data.use_diffraction && base_idx < num_avenue_bases {
                let power = diffraction_power_linear(&data, base_station);
                total_interference += power;
                if power > useful_power && data.connect_to_nlos { useful_power = power; }
            }
        }

        let sinr = sinr_linear(useful_power, data.noise_power, total_interference);
        results_linear.push(sinr);
    }

    // Return mean SINR in dB
    let mean_linear = results_linear.iter().sum::<f64>() / results_linear.len() as f64;
    10.0 * mean_linear.log10()
}

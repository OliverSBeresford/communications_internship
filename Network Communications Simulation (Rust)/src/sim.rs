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

/// Generates a random Manhattan layout using a Poisson Point Process for avenues and streets, and optionally places base stations along them.
/// - `size`: The size of the grid (e.g., 1000 for a 1km x 1km area).
/// - `lambda_ave`: Density of avenues (average number of avenues per unit length).
/// - `lambda_st`: Density of streets (average number of streets per unit length).
/// - `lambda_base`: Density of base stations (average number of base stations per square kilometer).
/// - `seed`: Random seed for reproducibility.
/// - `create_base_stations`: Whether to generate base stations based on the provided density.
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
        let mean_base_stations_total = (lambda_base * size * size / 1e6).max(0.0);
        let num_roads = avenues.len() + streets.len();
        let mean_per_road = if num_roads > 0 { mean_base_stations_total / num_roads as f64 } else { 0.0 };
        let poisson_bs = if mean_per_road > 0.0 { Some(Poisson::new(mean_per_road).unwrap()) } else { None };
        for &avenue in &avenues {
            let count = poisson_bs.as_ref().map(|p| p.sample(&mut rng) as usize).unwrap_or(0);
            ave_counts.push(count);
            for _ in 0..count {
                let y = rng.gen::<f64>() * size - size / 2.0;
                base_stations.push(Point { x: avenue, y });
            }
        }
        for &street in &streets {
            let count = poisson_bs.as_ref().map(|p| p.sample(&mut rng) as usize).unwrap_or(0);
            for _ in 0..count {
                let x = rng.gen::<f64>() * size - size / 2.0;
                base_stations.push(Point { x, y: street });
            }
        }
    } else {
        ave_counts.resize(num_avenues, 0);
    }

    ManhattanLayout { avenues, streets, base_stations, ave_counts }
}

/// Generates a random Manhattan layout using a Poisson Point Process for avenues and streets, and optionally places base stations along them.
/// - `size`: The size of the grid (e.g., 1000 for a 1km x 1km area).
/// - `lambda_ave`: Density of avenues (average number of avenues per unit length).
/// - `lambda_st`: Density of streets (average number of streets per unit length).
/// - `lambda_base`: Density of base stations (average number of base stations per square kilometer).
/// - `seed`: Random seed for reproducibility.
/// - `create_base_stations`: Whether to generate base stations based on the provided density.
pub fn generate_manhattan_simple(
    size: f64,
    lambda_ave: f64,
    lambda_st: f64,
    lambda_base: f64,
    seed: u64,
    create_base_stations: bool,
) -> ManhattanLayout {
    // Generate number of avenues and streets based on Poisson distribution, but don't actually generate the streets and avenues since we won't use them for NLOS or diffraction calculations
    let mut rng = StdRng::seed_from_u64(seed);

    // Avg number of avenues and streets based on density and size, ensuring non-negative values
    let mean_ave = (size * lambda_ave).max(0.0);
    let mean_st = (size * lambda_st - 1.0).max(0.0);

    // Number of avenues and streets based on Poisson distribution, ensuring at least 1 street (y=0) and non-negative counts
    let num_avenues = if mean_ave > 0.0 {
        Poisson::new(mean_ave).unwrap().sample(&mut rng) as usize
    } else { 0 };
    let num_streets = if mean_st > 0.0 {
        Poisson::new(mean_st - 1.0).unwrap().sample(&mut rng) as usize + 1
    } else { 1 }; // include y=0

    // Don't actually generate the avenues and streets since we won't use them for NLOS or diffraction calculations
    let avenues = Vec::with_capacity(0);
    let mut streets = Vec::with_capacity(1);
    streets.push(0.0);

    let mut base_stations = Vec::new();
    let mut ave_counts = Vec::with_capacity(0);

    if create_base_stations {
        let mean_base_stations_total = (lambda_base * size * size / 1e6).max(0.0);
        let num_roads = num_avenues + num_streets;
        let mean_per_road = if num_roads > 0 { mean_base_stations_total / num_roads as f64 } else { 0.0 };
        let poisson_bs = if mean_per_road > 0.0 { Some(Poisson::new(mean_per_road).unwrap()) } else { None };

        // Place all base stations along the y=0 street for simplicity, since we won't be doing NLOS or diffraction calculations that depend on the actual layout
        for &street in &streets {
            let count = poisson_bs.as_ref().map(|p| p.sample(&mut rng) as usize).unwrap_or(0);
            for _ in 0..count {
                let x = rng.gen::<f64>() * size - size / 2.0;
                base_stations.push(Point { x, y: street });
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
    pub small_scale_fading: bool,    // whether to include small scale fading in the simulation
}

impl SimulationData {
    pub fn generate_layout(&mut self) {
        let seed = (rand::thread_rng().gen::<f64>() * 1e12) as u64; // Generate a random seed for layout generation

        if !self.use_nlos && !self.use_diffraction {
            let layout = generate_manhattan_simple(self.size, self.lambda_ave, self.lambda_st, self.lambda_base, seed, self.create_base_stations);
            self.avenues = layout.avenues;
            self.streets = layout.streets;
            self.base_stations = layout.base_stations;
            self.ave_counts = layout.ave_counts;
            return;
        }

        let layout = generate_manhattan(
            self.size,
            self.lambda_ave,
            self.lambda_st,
            self.lambda_base,
            seed,
            self.create_base_stations,
        );
        self.avenues = layout.avenues;
        self.streets = layout.streets;
        self.base_stations = layout.base_stations;
        self.ave_counts = layout.ave_counts;
    }

    pub fn layout_summary(&self) -> String {
        format!("Avenues: {}, Streets: {}, Base stations: {}", 
            self.avenues.len(), self.streets.len(), self.base_stations.len())
    }

    pub fn display_layout(&self) {
        println!("{}", self.layout_summary());
    }
}

impl Default for SimulationData {
    fn default() -> Self {
        SimulationData {
            source_power: 1.0,
            receiver: Point { x: 0.0, y: 0.0 },
            alpha: 4.0,
            a: 1.0,
            fading_mean: 1.0,
            noise_power: 4e-15, // -114 dBmW noise floor
            base_stations: Vec::new(),
            penetration_loss: 0.1, // 10 dB penetration loss
            avenues: Vec::new(),
            streets: Vec::new(),
            use_nlos: false,
            use_diffraction: false,
            size: 5000.0, // 5km x 5km area
            path_loss_nlos: true,
            diffraction_order: 1,
            ave_counts: Vec::new(),
            connect_to_nlos: false,
            lambda_ave: 7.0 / 1000.0,
            lambda_st: 7.0 / 1000.0,
            lambda_base: 15.0, // per km^2
            create_base_stations: true,
            computation_nodes: 100,
            threshold_db: 10.0,
            small_scale_fading: true,
        }
    }
}

/// Generates a random fading value from an exponential distribution with the specified mean.
/// Rayleigh fading
/// - `rng`: A mutable reference to a random number generator.
/// - `mean_fade`: The mean value of the exponential distribution, which controls the severity of fading (e.g., 1.0 for standard Rayleigh fading).
/// Returns a random value representing the fading effect.
pub fn small_scale_fading_exp(rng: &mut impl Rng, mean_fade: f64) -> f64 {
    let exponential = Exp::new(1.0 / mean_fade).unwrap(); // Exp with mean=mean_fade
    let uniform_sample: f64 = rng.gen();
    exponential.inverse_cdf(uniform_sample)
}

/// Counts the number of roads (avenues and streets) crossed by the line between the receiver and a given transmitter.
/// This is used to determine the number of building penetrations for NLOS path loss calculations.
/// - `data`: The simulation data containing the receiver position and road locations.
/// - `transmitter`: The position of the transmitter (base station).
/// 
/// Returns the total number of roads crossed, which corresponds to the number of building penetrations.
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
    let mut received_power = data.source_power * path_loss;
    if data.small_scale_fading {
        received_power *= small_scale_fading_exp(rng, data.fading_mean);
    }
    received_power.min(data.source_power)
}

pub fn power_nlos_linear(rng: &mut impl Rng, data: &SimulationData, transmitter: Point) -> f64 {
    let building_count = 1 + num_roads_crossed(data, transmitter);
    let mut received_power = data.source_power
        * data.penetration_loss.powi(building_count);
    if data.path_loss_nlos {
        let distance = euclidean_distance(data.receiver, transmitter);
        received_power *= data.a * distance.powf(-data.alpha);
    }
    if data.small_scale_fading {
        received_power *= small_scale_fading_exp(rng, data.fading_mean);
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

pub fn sinr_linear(data: &mut SimulationData) -> f64 {
    // Place user at (0,0) each time, ensuring y=0 street exists from generator
    data.receiver = Point { x: 0.0, y: 0.0 };

    let mut useful_power = 0.0;
    let mut total_interference = 0.0;
    let num_avenue_bases: usize = if data.diffraction_order > 0 && !data.ave_counts.is_empty() {
        data.ave_counts.iter().sum()
    } else { 0 };

    let mut rng = rand::thread_rng();

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

    if useful_power == 0.0 { return 0.0; }
    let sinr = useful_power / (data.noise_power + total_interference - useful_power);
    if sinr < 0.0 { 0.0 } else { sinr }
}


/// Simulate CCDF curve for SINR in dB for the current layout and parameters, placing the user at (0,0) and calculating contributions from all base stations based on LOS, NLOS, and diffraction conditions.
/// Returns a tuple of (coverage_x, coverage_y) where coverage_x is the vector of SINR bin edges in dB and coverage_y is the corresponding CCDF values.
/// - `data`: The simulation data containing the layout and parameters for the simulation.
/// - `simulations`: The number of simulation iterations to run for averaging the CCDF curve
/// - `num_bins`: The number of bins to use for the CCDF curve (e.g., 100).
/// - `progress_bar`: Whether to display a progress bar during the simulation.
pub fn simulate_coverage_ccdf(data: &mut SimulationData, simulations: usize, num_bins: usize, progress_bar: bool) -> (Vec<f64>, Vec<f64>) {
    let mut results_db: Vec<f64> = Vec::with_capacity(simulations);

    // Progress bar for simulation iterations
    let pb = ProgressBar::new(simulations as u64);
    if progress_bar {
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) - simulating CCDF")
            .unwrap()
            .progress_chars("=>-"));
    }

    for _ in 0..simulations {
        // Regenerate a fresh Manhattan layout each simulation
        data.generate_layout();

        // Calculate SINR for this simulation iteration
        let sinr = sinr_linear(data);
        results_db.push(10.0 * sinr.log10());

        // update progress bar
        if progress_bar { pb.inc(1); }
    }

    if progress_bar { pb.finish_with_message("CCDF simulation complete"); }

    crate::metrics::ccdf(&results_db, num_bins)
}

pub fn simulate_average_sinr(data: &mut SimulationData, simulations: usize) -> f64 {
    let mut results_linear: Vec<f64> = Vec::with_capacity(simulations);

    for _ in 0..simulations {
        // Regenerate a fresh Manhattan layout each simulation
        data.generate_layout();

        // Calculate SINR for this simulation iteration
        let sinr = sinr_linear(data);
        results_linear.push(sinr);
    }

    // Return mean SINR in dB
    let mean_linear = results_linear.iter().sum::<f64>() / results_linear.len() as f64;
    10.0 * mean_linear.log10()
}

/// Simulate average SINR as a function of base station density, sweeping over a range of densities and returning the results as a vector of (density, average_sinr_db) pairs.
/// - `data`: A mutable reference to the simulation data, which will be updated with different base station densities.
/// - `simulations`: The number of simulation iterations to run for each density point.
pub fn simulate_sinr_vs_density(data: &mut SimulationData, simulations: usize) -> Vec<(f64, f64)> {
    // Sweep across base station densities on a logarithmic grid (1e-3 to 1)
    let density_range: Vec<f64> = (0..=24)
        .map(|i| 10f64.powf(-0.3 + i as f64 * (4.3 / 24.0)))
        .collect();
    let mut results: Vec<(f64, f64)> = Vec::new();

    println!("Sweeping base station densities...");
    for &base_station_density in &density_range {
        data.lambda_base = base_station_density;

        // Calculate average SINR for this density
        let avg_sinr = simulate_average_sinr(data, simulations);
        results.push((base_station_density, avg_sinr));
        println!("Density {:.2}: avg SINR = {:.3} dB", base_station_density, avg_sinr);
    }
    results
}

use crate::geom::Point;
use crate::sim::{SimulationData, power_los_linear, power_nlos_linear, diffraction_power_linear};
use rayon::prelude::*;

/// Evaluate fitness of current base station deployment by counting points with SINR above threshold
pub fn fitness_value(data: &SimulationData) -> i64 {
    let samples = linspace(-data.size / 2.0, data.size / 2.0, data.computation_nodes);

    let avenue_coverage_points: i64 = data
        .avenues
        .par_iter()
        .map(|&avenue_x| {
            samples
                .iter()
                .filter(|&&y_pos| {
                    let sinr_linear = single_point_sinr_linear(data, Point { x: avenue_x, y: y_pos });
                    let sinr_db = 10.0 * sinr_linear.log10();
                    sinr_db > data.threshold_db
                })
                .count() as i64
        })
        .sum();

    let street_coverage_points: i64 = data
        .streets
        .par_iter()
        .map(|&street_y| {
            samples
                .iter()
                .filter(|&&x_pos| {
                    let sinr_linear = single_point_sinr_linear(data, Point { x: x_pos, y: street_y });
                    let sinr_db = 10.0 * sinr_linear.log10();
                    sinr_db > data.threshold_db
                })
                .count() as i64
        })
        .sum();

    avenue_coverage_points + street_coverage_points
}

/// Generate linearly spaced values between start and end
fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![start]; }
    let step = (end - start) / (n as f64 - 1.0);
    (0..n).map(|i| start + i as f64 * step).collect()
}

/// Calculates the Signal to Interference plus Noise Ratio (SINR) in linear scale for the current layout and parameters
/// - `data`: The simulation data containing the layout and parameters for the simulation.
/// - `receiver`: The point for which to calculate SINR.
/// Returns the SINR value in linear scale for this simulation iteration.
pub fn single_point_sinr_linear(data: &SimulationData, receiver: Point) -> f64 {
    let mut useful_power = 0.0;
    let mut total_interference = 0.0;
    let num_avenue_bases: usize = if data.diffraction_order > 0 && !data.ave_counts.is_empty() {
        data.ave_counts.iter().sum()
    } else { 0 };

    let mut rng = rand::thread_rng();

    for (base_idx, &base_station) in data.base_stations.iter().enumerate() {
        let same_street = base_station.x == receiver.x || base_station.y == receiver.y;
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

/// Identify the best candidate base stations to activate or deactivate for fitness improvement
pub fn best_candidates(
    data: &mut SimulationData,
    candidates: &[Point],
    candidate_select: &mut [bool],
    base_fitness: i64,
) -> (usize, usize) {
    let mut activation_improvement = f64::NEG_INFINITY;
    let mut best_activation_index = 0usize;
    let mut deactivation_improvement = f64::NEG_INFINITY;
    let mut best_deactivation_index = 0usize;

    // Evaluate each candidate for potential activation or deactivation
    for candidate_idx in 0..candidates.len() {
        // Determine if candidate is currently active
        if candidate_select[candidate_idx] {
            // Candidate is active, evaluate deactivation
            candidate_select[candidate_idx] = false;
            data.base_stations = current_bs(candidates, candidate_select);
            let new_fitness = fitness_value(data);
            let improvement = (new_fitness - base_fitness) as f64;
            if improvement > deactivation_improvement {
                deactivation_improvement = improvement;
                best_deactivation_index = candidate_idx;
            }
            candidate_select[candidate_idx] = true;
        } else {
            // Candidate is inactive, evaluate activation
            candidate_select[candidate_idx] = true;
            data.base_stations = current_bs(candidates, candidate_select);
            let new_fitness = fitness_value(data);
            let improvement = (new_fitness - base_fitness) as f64;
            if improvement > activation_improvement {
                activation_improvement = improvement;
                best_activation_index = candidate_idx;
            }
            candidate_select[candidate_idx] = false;
        }
    }
    (best_activation_index, best_deactivation_index)
}

fn current_bs(candidates: &[Point], select: &[bool]) -> Vec<Point> {
    candidates
        .iter()
        .zip(select.iter())
        .filter_map(|(&p, &s)| if s { Some(p) } else { None })
        .collect()
}

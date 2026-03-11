use crate::geom::Point;
use crate::sim::{SimulationData, power_los_linear, power_nlos_linear, diffraction_power_linear};
use rayon::prelude::*;
use rand::seq::IteratorRandom;

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

pub fn randomly_initialize_selection_mask(candidate_count: usize, target_count: usize) -> Vec<bool> {
    let mut rng = rand::thread_rng();

    let mut mask: Vec<bool> = vec![false; candidate_count];
    (0..candidate_count).choose_multiple(&mut rng, target_count).into_iter().for_each(|i| mask[i] = true);
    mask
}

pub fn current_bs(candidates: &[Point], select: &[bool]) -> Vec<Point> {
    candidates
        .iter()
        .zip(select.iter())
        .filter_map(|(&p, &s)| if s { Some(p) } else { None })
        .collect()
}

pub fn steepest_ascent_hill_climb(data: &mut SimulationData, candidate_positions: Vec<Point>, target_deployment_count: usize) -> (Vec<bool>, i64) {
    println!("Initial deployed: {}", data.base_stations.len());

    // Initial selection mask for base stations
    let mut selection_mask = randomly_initialize_selection_mask(candidate_positions.len(), target_deployment_count);

    // Initial fitness evaluation
    let mut base_fitness = fitness_value(data);
    // Store recent fitness values for convergence check (10 iterations, checking relative std dev)
    let mut recent_fitness_values: Vec<f64> = vec![-f64::INFINITY; 10];

    for iteration in 0..50 { // limited iterations for demo
        // Identify best candidates to activate/deactivate
        let (best_activation_idx, best_deactivation_idx) = best_candidates(data, &candidate_positions, &mut selection_mask, base_fitness);
        selection_mask[best_activation_idx] = true;
        selection_mask[best_deactivation_idx] = false;
        data.base_stations = current_bs(&candidate_positions, &selection_mask);

        // Re-evaluate fitness after adjustments
        let new_fitness = fitness_value(data);
        recent_fitness_values.remove(9);
        recent_fitness_values.insert(0, new_fitness as f64);
        println!("iter {} fitness {} -> {}", iteration, base_fitness, new_fitness);

        base_fitness = new_fitness;

        // Check for convergence based on relative standard deviation of recent fitness values
        if recent_fitness_values.iter().all(|v| v.is_finite()) {
            if let Some(relative_std_dev) = rel_std(&recent_fitness_values) {
                if relative_std_dev < 0.05 { break; }
            }
        }
    }
    (selection_mask, base_fitness)
}

// Calculate relative standard deviation of a slice of f64 values
pub fn rel_std(values: &[f64]) -> Option<f64> {
    if values.is_empty() { return None; }
    let average: f64 = values.iter().sum::<f64>() / values.len() as f64;
    if average.abs() < 1e-12 { return None; }
    let variance: f64 = values.iter().map(|value| (value - average).powi(2)).sum::<f64>() / values.len() as f64;
    Some(variance.sqrt() / average.abs())
}


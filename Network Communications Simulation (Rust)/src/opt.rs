use crate::geom::Point;
use crate::sim::{SimulationData};
use rand::rngs::StdRng;
use rand::SeedableRng;

pub fn fitness_value(data: &mut SimulationData) -> i64 {
    let mut total_coverage_points: i64 = 0;
    for &avenue_x in &data.avenues {
        for y_pos in linspace(-data.size / 2.0, data.size / 2.0, data.computation_nodes) {
            data.receiver = Point { x: avenue_x, y: y_pos };
            let sinr_db = single_point_sinr_db(&data);
            if sinr_db > data.threshold_db { total_coverage_points += 1; }
        }
    }
    for &street_y in &data.streets {
        for x_pos in linspace(-data.size / 2.0, data.size / 2.0, data.computation_nodes) {
            data.receiver = Point { x: x_pos, y: street_y };
            let sinr_db = single_point_sinr_db(&data);
            if sinr_db > data.threshold_db { total_coverage_points += 1; }
        }
    }
    total_coverage_points
}

fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![start]; }
    let step = (end - start) / (n as f64 - 1.0);
    (0..n).map(|i| start + i as f64 * step).collect()
}

fn single_point_sinr_db(data: &SimulationData) -> f64 {
    // Compute useful and interference powers similar to MATLAB SINR
    let mut useful_power = 0.0;
    let mut total_interference = 0.0;
    let num_avenue_bases: usize = if data.diffraction_order > 0 && !data.ave_counts.is_empty() {
        data.ave_counts.iter().sum()
    } else { 0 };
    for (candidate_idx, &base_station) in data.base_stations.iter().enumerate() {
        let same_street = base_station.x == data.receiver.x || base_station.y == data.receiver.y;
        if same_street {
            let mut rng = seeded_rng(data, base_station, candidate_idx as u64);
            let power_linear = super::sim::power_los_linear(&mut rng, data, base_station);
            total_interference += power_linear;
            if power_linear > useful_power { useful_power = power_linear; }
        } else if data.use_nlos {
            let mut rng = seeded_rng(data, base_station, candidate_idx as u64);
            let power_linear = super::sim::power_nlos_linear(&mut rng, data, base_station);
            total_interference += power_linear;
            if power_linear > useful_power && data.connect_to_nlos { useful_power = power_linear; }
        }
        if !same_street && data.diffraction_order > 0 && candidate_idx < num_avenue_bases {
            let power_linear = super::sim::diffraction_power_linear(data, base_station);
            total_interference += power_linear;
            if power_linear > useful_power && data.connect_to_nlos { useful_power = power_linear; }
        }
    }
    if useful_power == 0.0 { return f64::NEG_INFINITY; }
    let sinr = useful_power / (data.noise_power + total_interference - useful_power);
    10.0 * sinr.log10()
}

fn seeded_rng(data: &SimulationData, base_station: Point, index: u64) -> StdRng {
    let seed = data.receiver.x.to_bits()
        ^ data.receiver.y.to_bits()
        ^ base_station.x.to_bits()
        ^ base_station.y.to_bits()
        ^ (data.base_stations.len() as u64)
        ^ index;
    StdRng::seed_from_u64(seed)
}

pub fn best_candidates(
    data: &mut SimulationData,
    candidates: &[Point],
    candidates_per_road: usize,
    candidate_select: &mut [bool],
    base_fitness: i64,
) -> (usize, usize) {
    let mut activation_improvement = f64::NEG_INFINITY;
    let mut best_activation_index = 0usize;
    let mut deactivation_improvement = f64::NEG_INFINITY;
    let mut best_deactivation_index = 0usize;

    let num_avenues = data.avenues.len();

    for candidate_idx in 0..candidates.len() {
        let on_avenue = candidate_idx < num_avenues * candidates_per_road;
        if candidate_select[candidate_idx] {
            if on_avenue { /* data.numAveBases -= 1; */ }
            candidate_select[candidate_idx] = false;
            data.base_stations = current_bs(candidates, candidate_select);
            let new_fitness = fitness_value(data);
            let improvement = (new_fitness - base_fitness) as f64;
            if improvement > deactivation_improvement {
                deactivation_improvement = improvement;
                best_deactivation_index = candidate_idx;
            }
            candidate_select[candidate_idx] = true;
            if on_avenue { /* data.numAveBases += 1; */ }
        } else {
            if on_avenue { /* data.numAveBases += 1; */ }
            candidate_select[candidate_idx] = true;
            data.base_stations = current_bs(candidates, candidate_select);
            let new_fitness = fitness_value(data);
            let improvement = (new_fitness - base_fitness) as f64;
            if improvement > activation_improvement {
                activation_improvement = improvement;
                best_activation_index = candidate_idx;
            }
            candidate_select[candidate_idx] = false;
            if on_avenue { /* data.numAveBases -= 1; */ }
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

use network_comms_sim::{geom::Point, opt::{best_candidates, fitness_value}, sim::{generate_manhattan, SimulationData}};

fn main() {
    // Initialize data using random Manhattan grid (MPLP - Manhattan Poisson Line Process)
    let grid_size = 500.0;
    let avenue_density = 7.0 / 1000.0;
    let street_density = 7.0 / 1000.0;
    let base_station_density = 1.5 / 1000.0;
    let seed = 42; // Seed for reproducibility
    
    // Generate random Manhattan layout using Poisson Point Process
    let layout = generate_manhattan(grid_size, avenue_density, street_density, 0.0, seed, false);
    let avenues = layout.avenues;
    let streets = layout.streets;
    
    println!("Generated MPLP grid: {} avenues, {} streets", avenues.len(), streets.len());

    let target_deployment_count = ((grid_size * base_station_density) * (avenues.len() + streets.len()) as f64).round() as usize;

    let base_spacing_distance = 20.0;
    let candidates_per_road = (grid_size / base_spacing_distance).round() as usize;
    let mut candidate_positions: Vec<Point> = Vec::with_capacity((avenues.len() + streets.len()) * candidates_per_road);
    for &avenue_x in &avenues {
        for position_idx in 0..candidates_per_road {
            let y_pos = -grid_size / 2.0 + position_idx as f64 * (grid_size / (candidates_per_road as f64 - 1.0));
            candidate_positions.push(Point { x: avenue_x, y: y_pos });
        }
    }
    for &street_y in &streets {
        for position_idx in 0..candidates_per_road {
            let x_pos = -grid_size / 2.0 + position_idx as f64 * (grid_size / (candidates_per_road as f64 - 1.0));
            candidate_positions.push(Point { x: x_pos, y: street_y });
        }
    }

    let mut selection_mask = vec![false; candidate_positions.len()];
    // deterministic initial selection: first N (mirrors MATLAB random choice if seeded separately)
    for idx in 0..target_deployment_count.min(selection_mask.len()) { selection_mask[idx] = true; }

    let mut data = SimulationData {
        source_power: 1.0,
        receiver: Point { x: 0.0, y: 0.0 },
        alpha: 4.0,
        a: 1.0,
        fading_mean: 1.0,
        noise_power: 0.0,
        base_stations: current_bs(&candidate_positions, &selection_mask),
        penetration_loss: 0.1,
        avenues: avenues.clone(),
        streets: streets.clone(),
        use_nlos: true,
        use_diffraction: true,
        size: grid_size,
        path_loss_nlos: true,
        diffraction_order: 1,
        ave_counts: vec![avenues.len()],
        connect_to_nlos: true,
        lambda_ave: avenue_density,
        lambda_st: street_density,
        lambda_base: base_station_density,
        create_base_stations: false,
        computation_nodes: 100,
        threshold_db: 10.0,
    };

    let mut base_fitness = fitness_value(&mut data);
    let mut recent_fitness_values: Vec<f64> = vec![-f64::INFINITY; 10];

    for iteration in 0..50 { // limited iterations for demo
        let (best_activation_idx, best_deactivation_idx) = best_candidates(&mut data, &candidate_positions, candidates_per_road, &mut selection_mask, base_fitness);
        selection_mask[best_activation_idx] = true;
        selection_mask[best_deactivation_idx] = false;
        data.base_stations = current_bs(&candidate_positions, &selection_mask);
        let new_fitness = fitness_value(&mut data);
        recent_fitness_values.remove(9);
        recent_fitness_values.insert(0, new_fitness as f64);
        println!("iter {} fitness {} -> {}", iteration, base_fitness, new_fitness);
        if recent_fitness_values.iter().all(|v| v.is_finite()) {
            if let Some(relative_std_dev) = rel_std(&recent_fitness_values) {
                if relative_std_dev < 0.05 { break; }
            }
        }
        base_fitness = new_fitness;
    }

    println!("Final deployed: {}", data.base_stations.len());
}

fn current_bs(candidates: &[Point], select: &[bool]) -> Vec<Point> {
    candidates
        .iter()
        .zip(select.iter())
        .filter_map(|(&position, &is_selected)| if is_selected { Some(position) } else { None })
        .collect()
}

fn rel_std(values: &[f64]) -> Option<f64> {
    if values.is_empty() { return None; }
    let average: f64 = values.iter().sum::<f64>() / values.len() as f64;
    if average.abs() < 1e-12 { return None; }
    let variance: f64 = values.iter().map(|value| (value - average).powi(2)).sum::<f64>() / values.len() as f64;
    Some(variance.sqrt() / average.abs())
}

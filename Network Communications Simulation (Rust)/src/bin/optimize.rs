use network_comms_sim::{geom::Point, optimization::{current_bs, steepest_ascent_hill_climb}, sim::{ManhattanLayout, SimulationData, generate_manhattan}, visualization::plot_manhattan_layout_with_zoom};

fn main() {
    // Initialize data using random Manhattan grid (MPLP - Manhattan Poisson Line Process)
    let grid_size = 5000.0;
    let avenue_density = 7.0 / 1000.0;
    let street_density = 7.0 / 1000.0;
    let base_station_density = 13.0;
    let seed = 42; // Seed for reproducibility
    
    // Generate random Manhattan layout using Poisson Point Process
    let layout = generate_manhattan(grid_size, avenue_density, street_density, 0.0, seed, false);
    let avenues = layout.avenues;
    let streets = layout.streets;

    let mut data = SimulationData {
        source_power: 1.0,
        receiver: Point { x: 0.0, y: 0.0 },
        alpha: 4.0,
        a: 1.0,
        fading_mean: 1.0,
        noise_power: 1e-14,
        base_stations: Vec::new(),
        penetration_loss: 0.1,
        avenues: avenues.clone(),
        streets: streets.clone(),
        use_nlos: true,
        use_diffraction: false,
        size: grid_size,
        path_loss_nlos: true,
        diffraction_order: 1,
        ave_counts: vec![avenues.len()],
        connect_to_nlos: false,
        lambda_ave: avenue_density,
        lambda_st: street_density,
        lambda_base: base_station_density,
        create_base_stations: false,
        computation_nodes: 100,
        threshold_db: 15.0,
        small_scale_fading: true,
    };
    
    println!("Generated MPLP grid: {} avenues, {} streets", avenues.len(), streets.len());

    // Determine budget of base stations, in this case based on density per unit length of roads
    let target_deployment_count = (grid_size * grid_size * base_station_density / 1e6).round() as usize;

    let base_spacing_distance = 20.0;
    let candidates_per_road = (grid_size / base_spacing_distance).round() as usize;
    // Generate candidate base station positions along avenues and streets
    let num_candidates: usize = (avenues.len() + streets.len()) * candidates_per_road;
    let mut candidate_positions: Vec<Point> = Vec::with_capacity(num_candidates);
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

    let mut final_mask: Vec<bool> = Vec::with_capacity(num_candidates);
    let mut best_fitness: i64 = -1;

    for i in 0..12 {
        println!("Optimization iteration {}/12", i + 1);
        let (selection_mask, fitness) = steepest_ascent_hill_climb(&mut data, candidate_positions.clone(), target_deployment_count);
        println!("Final fitness: {}", fitness);
        if fitness > best_fitness {
            println!("New best fitness found: {} (previous {})", fitness, best_fitness);
            final_mask = selection_mask;
            best_fitness = fitness;
        }
    }

    // Finalize base station deployment based on best selection mask
    data.base_stations = current_bs(&candidate_positions, &final_mask);

    println!("Final deployed: {}", data.base_stations.len());

    plot_manhattan_layout_with_zoom(
        &ManhattanLayout {
            avenues,
            streets,
            base_stations: data.base_stations.clone(),
            ave_counts: data.ave_counts.clone(),
        },
        data.size,
        "output/manhattan_optimized.svg",
        Some(0.0),
        Some(0.0),
        Some(data.size)
    ).expect("Failed to plot optimized layout");
}
use network_comms_sim::{sim::generate_manhattan, visualization::plot_manhattan_layout};
use std::fs::create_dir_all;
use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let size = if args.len() > 1 {
        args[1].parse::<f64>().unwrap_or_else(|_| {
            eprintln!("Usage: {} [size]", args[0]);
            eprintln!("Using default size of 5000.0");
            5000.0
        })
    } else {
        eprintln!("Usage: {} [size]", args[0]);
        eprintln!("Using default size of 5000.0");
        5000.0
    };

    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");
    let mut data: network_comms_sim::sim::SimulationData = Default::default();
    data.size = size;

    // Generate a random Manhattan layout
    let layout = generate_manhattan(data.size, data.lambda_ave, data.lambda_st, data.lambda_base, 12, true);

    // Plot it
    if let Err(error) = plot_manhattan_layout(&layout, data.size, &format!("output/manhattan_{}.svg", data.size as i32)) {
        eprintln!("Error plotting Manhattan layout: {}", error);
    } else {
        println!("Plotted Manhattan layout to output/manhattan.svg");
        println!("Avenues: {}, Streets: {}, Base stations: {}",
            layout.avenues.len(), layout.streets.len(), layout.base_stations.len());
    }
}

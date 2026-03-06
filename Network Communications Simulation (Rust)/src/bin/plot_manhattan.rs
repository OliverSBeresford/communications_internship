use network_comms_sim::{sim::generate_manhattan, viz::plot_manhattan_layout};
use std::fs::create_dir_all;

fn main() {
    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");
    let data: network_comms_sim::sim::SimulationData = Default::default();

    // Generate a random Manhattan layout
    let layout = generate_manhattan(data.size, data.lambda_ave, data.lambda_st, data.lambda_base, 12, true);

    // Plot it
    if let Err(error) = plot_manhattan_layout(&layout, data.size, "output/manhattan.svg") {
        eprintln!("Error plotting Manhattan layout: {}", error);
    } else {
        println!("Plotted Manhattan layout to output/manhattan.svg");
        println!("Avenues: {}, Streets: {}, Base stations: {}",
            layout.avenues.len(), layout.streets.len(), layout.base_stations.len());
    }
}

use network_comms_sim::{sim::generate_manhattan, viz::plot_manhattan_layout};
use std::fs::create_dir_all;

fn main() {
    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");
    let grid_size = 1000.0;
    let avenue_density = 7.0 / 1000.0;
    let street_density = 7.0 / 1000.0;
    let base_station_density = 1.5 / 1000.0;

    // Generate a random Manhattan layout
    let layout = generate_manhattan(grid_size, avenue_density, street_density, base_station_density, 1, true);

    // Plot it
    if let Err(error) = plot_manhattan_layout(&layout, grid_size, "output/manhattan.svg") {
        eprintln!("Error plotting Manhattan layout: {}", error);
    } else {
        println!("Plotted Manhattan layout to output/manhattan.svg");
        println!("Avenues: {}, Streets: {}, Base stations: {}", 
            layout.avenues.len(), layout.streets.len(), layout.base_stations.len());
    }
}

use network_comms_sim::{sim::generate_manhattan, visualization::plot_manhattan_layout_with_zoom};
use std::fs::create_dir_all;
use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage: {} [size] [center_x center_y zoom_size]", args[0]);
        println!("  size: Grid size in meters (default: 5000)");
        println!("  center_x, center_y: Center of zoom region (default: 0, 0)");
        println!("  zoom_size: Size of zoom region in meters (default: full size)");
        println!("\nExamples:");
        println!("  {} 5000              # Generate 5km grid, plot full view", args[0]);
        println!("  {} 5000 0 0 500      # Generate 5km grid, zoom to 500m region at center", args[0]);
        println!("  {} 5000 1000 500 800 # Generate 5km grid, zoom to 800m region at (1000, 500)", args[0]);
        return;
    }
    
    let size = if args.len() > 1 {
        args[1].parse::<f64>().unwrap_or_else(|_| {
            eprintln!("Using default size of 5000.0");
            5000.0
        })
    } else {
        5000.0
    };
    
    // Parse zoom parameters if provided
    let (center_x, center_y, zoom_size) = if args.len() >= 5 {
        let cx = args[2].parse::<f64>().unwrap_or(0.0);
        let cy = args[3].parse::<f64>().unwrap_or(0.0);
        let zs = args[4].parse::<f64>().unwrap_or(size);
        (Some(cx), Some(cy), Some(zs))
    } else {
        (None, None, None)
    };

    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");
    let mut data: network_comms_sim::sim::SimulationData = Default::default();
    data.size = size;

    // Generate a random Manhattan layout
    let layout = generate_manhattan(data.size, data.lambda_ave, data.lambda_st, data.lambda_base, 12, true);

    // Plot it with optional zoom
    let output_name = if let Some(zs) = zoom_size {
        format!("output/manhattan_{}_zoom_{}.svg", data.size as i32, zs as i32)
    } else {
        format!("output/manhattan_{}.svg", data.size as i32)
    };
    
    if let Err(error) = plot_manhattan_layout_with_zoom(&layout, data.size, &output_name, center_x, center_y, zoom_size) {
        eprintln!("Error plotting Manhattan layout: {}", error);
    } else {
        if let Some(zs) = zoom_size {
            println!("Plotted Manhattan layout to {} (zoomed to {}m region)", output_name, zs);
        } else {
            println!("Plotted Manhattan layout to {}", output_name);
        }
        println!("Avenues: {}, Streets: {}, Base stations: {}",
            layout.avenues.len(), layout.streets.len(), layout.base_stations.len());
    }
}

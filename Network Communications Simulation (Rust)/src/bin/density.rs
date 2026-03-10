use network_comms_sim::{geom::Point, sim::{SimulationData, simulate_sinr_vs_density}};
use std::fs::{File, create_dir_all};
use std::path::Path;
use csv::Writer;
use plotters::prelude::*;
use plotters_svg::SVGBackend;

fn main() {
    // Randomly generate Manhattan layout (Poisson PPP) similar to manhattan.m
    let grid_size = 1000.0;
    let avenue_density = 7.0 / 1000.0;
    let street_density = 7.0 / 1000.0;

    // Right now, we are ignoring NLOS and diffraction for speed and tractability
    let mut data = SimulationData {
        source_power: 1.0,
        receiver: Point { x: 0.0, y: 0.0 },
        alpha: 4.0,
        a: 1.0,
        fading_mean: 1.0,
        noise_power: 0.01,
        base_stations: Vec::new(),
        penetration_loss: 0.01,
        avenues: Vec::new(),
        streets: Vec::new(),
        use_nlos: false,
        use_diffraction: false,
        size: grid_size,
        path_loss_nlos: false,
        diffraction_order: 0,
        ave_counts: Vec::new(),
        connect_to_nlos: false,
        lambda_ave: avenue_density,
        lambda_st: street_density,
        lambda_base: 0.0, // Will vary this in the sweep
        create_base_stations: true,
        computation_nodes: 100,
        threshold_db: 10.0,
        small_scale_fading: true,
    };

    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");

    let simulations = 1e5 as usize;
    let results = simulate_sinr_vs_density(&mut data, simulations);

    // Write results to CSV
    let csv_name = &format!("output/density_vs_sinr_{}.csv", simulations);
    let csv_path = Path::new(csv_name);
    let csv_file = File::create(csv_path).expect("create csv");
    let mut csv_writer = Writer::from_writer(csv_file);
    csv_writer.write_record(["base_station_density", "avg_sinr_db"]).unwrap();
    for (density, sinr) in &results {
        csv_writer.write_record([density.to_string(), sinr.to_string()]).unwrap();
    }
    csv_writer.flush().unwrap();
    println!("Wrote {} points to {}", results.len(), csv_name);

    // Plot results as SVG
    let svg_name = &format!("output/density_vs_sinr_{}.svg", simulations);
    let root = SVGBackend::new(svg_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Determine axis ranges
    let density_min = results.iter().map(|(d, _)| d).cloned().fold(f64::INFINITY, f64::min);
    let density_max = results.iter().map(|(d, _)| d).cloned().fold(f64::NEG_INFINITY, f64::max);
    let sinr_min = results.iter().map(|(_, s)| s).cloned().fold(f64::INFINITY, f64::min);
    let sinr_max = results.iter().map(|(_, s)| s).cloned().fold(f64::NEG_INFINITY, f64::max);

    // Create chart with logarithmic x-axis
    let mut chart = ChartBuilder::on(&root)
        .caption("Average SINR vs Base Station Density (log scale)", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(density_min.log10()..density_max.log10(), sinr_min..sinr_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("log10(Base Station Density per 1000 units²)")
        .y_desc("Average SINR (dB)")
        .draw()
        .unwrap();

    chart
        .draw_series(
            plotters::series::LineSeries::new(
                results.iter().map(|&(x, y)| (x.log10(), y)),
                &BLUE,
            ),
        )
        .unwrap();

    root.present().unwrap();
    println!("Wrote {}", svg_name);
}

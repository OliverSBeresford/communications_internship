use network_comms_sim::{geom::Point, sim::{SimulationData, simulate_coverage_ccdf}};
use std::env;
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
    let base_station_density = 1.5 / 1000.0;

    let data = SimulationData {
        source_power: 1.0,
        receiver: Point { x: 0.0, y: 0.0 },
        alpha: 4.0,
        a: 1.0,
        fading_mean: 1.0,
        noise_power: 0.01,
        base_stations: Vec::new(),
        penetration_loss: 0.9,
        avenues: Vec::new(),
        streets: Vec::new(),
        use_nlos: true,
        use_diffraction: true,
        size: grid_size,
        path_loss_nlos: true,
        diffraction_order: 1,
        ave_counts: Vec::new(),
        connect_to_nlos: true,
        lambda_ave: avenue_density,
        lambda_st: street_density,
        lambda_base: base_station_density,
        create_base_stations: true,
        computation_nodes: 100,
        threshold_db: 10.0,
    };

    let simulations = 1e5 as usize;
    let num_bins = simulations / 200;
    let (ccdf_x, ccdf_y) = simulate_coverage_ccdf(data, simulations, num_bins, 42);

    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--output-csv") {
        let path = Path::new("output/ccdf.csv");
        let file = File::create(path).expect("create csv");
        let mut csv_writer = Writer::from_writer(file);
        csv_writer.write_record(["theta", "probability"]).unwrap();
        for (x_value, y_value) in ccdf_x.iter().zip(ccdf_y.iter()) {
            csv_writer.write_record([x_value.to_string(), y_value.to_string()]).unwrap();
        }
        csv_writer.flush().unwrap();
        println!("Wrote {} points to output/ccdf.csv", ccdf_x.len());
    }

    if args.iter().any(|a| a == "--plot-svg") {
        let root = SVGBackend::new("output/ccdf.svg", (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let y_minimum = ccdf_y.iter().cloned().fold(f64::INFINITY, f64::min);
        let y_maximum = ccdf_y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let x_minimum = *ccdf_x.first().unwrap();
        let x_maximum = *ccdf_x.last().unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("Coverage probability CCDF", ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_minimum..x_maximum, y_minimum..y_maximum).unwrap();
        chart.configure_mesh()
            .x_desc("θ")
            .y_desc("Probability")
            .draw().unwrap();
        chart.draw_series(plotters::series::LineSeries::new(ccdf_x.iter().zip(ccdf_y.iter()).map(|(&x_val,&y_val)|(x_val,y_val)), &RED)).unwrap();
        println!("Wrote output/ccdf.svg plot");
    }

    if !args.iter().any(|a| a == "--output-csv" || a == "--plot-svg") {
        println!("x_len={} y_len={} y_first={:.3} y_last={:.3}", ccdf_x.len(), ccdf_y.len(), ccdf_y.first().unwrap(), ccdf_y.last().unwrap());
        println!("Pass --output-csv to export to output/ccdf.csv or --plot-svg to plot to output/ccdf.svg");
    }
}

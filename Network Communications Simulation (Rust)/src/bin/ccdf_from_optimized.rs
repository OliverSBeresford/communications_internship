use csv::Writer;
use network_comms_sim::{
    geom::Point,
    metrics::ccdf,
    optimization::{single_point_sinr_linear, OptimizationSnapshot},
};
use plotters::prelude::*;
use plotters_svg::SVGBackend;
use rayon::prelude::*;
use std::env;
use std::fs::{create_dir_all, File};
use std::path::Path;

fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![start];
    }
    let step = (end - start) / (n as f64 - 1.0);
    (0..n).map(|i| start + i as f64 * step).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("Usage: {} [snapshot_path] [num_bins] [computation_nodes]", args[0]);
        println!("  snapshot_path: Path to optimization snapshot JSON");
        println!("                 (default: output/optimized_deployment.json)");
        println!("  num_bins: Number of CCDF bins (default: 500)");
        println!("  computation_nodes: Points per road for spatial sampling");
        println!("                     (default: value from snapshot data)");
        println!("\nExample:");
        println!("  {} output/optimized_deployment.json 500 200", args[0]);
        return;
    }

    let snapshot_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("output/optimized_deployment.json");
    let num_bins = args
        .get(2)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(500);

    let snapshot_file = File::open(snapshot_path)
        .unwrap_or_else(|_| panic!("Failed to open snapshot file: {}", snapshot_path));
    let snapshot: OptimizationSnapshot = serde_json::from_reader(snapshot_file)
        .unwrap_or_else(|_| panic!("Failed to parse snapshot JSON: {}", snapshot_path));

    let mut data = snapshot.data;
    if let Some(override_nodes) = args.get(3).and_then(|value| value.parse::<usize>().ok()) {
        data.computation_nodes = override_nodes;
    } else {
        data.computation_nodes = 1e6 as usize; // Default to 100k points per road for smoother CCDF curves
    }

    let mode_name = if data.use_nlos { "nlos" } else { "los" };
    let density_label = data.lambda_base as i32;
    let samples = linspace(-data.size / 2.0, data.size / 2.0, data.computation_nodes);

    println!(
        "Loaded deployment: {} base stations, mode={}, lambda_base={} per km^2, computation_nodes={}",
        data.base_stations.len(),
        mode_name,
        data.lambda_base,
        data.computation_nodes
    );

    let mut receiver_points: Vec<Point> = Vec::with_capacity(
        (data.avenues.len() + data.streets.len()) * samples.len(),
    );
    for &avenue_x in &data.avenues {
        for &y_pos in &samples {
            receiver_points.push(Point { x: avenue_x, y: y_pos });
        }
    }
    for &street_y in &data.streets {
        for &x_pos in &samples {
            receiver_points.push(Point { x: x_pos, y: street_y });
        }
    }

    let results_db: Vec<f64> = receiver_points
        .par_iter()
        .map(|&receiver| {
            let sinr = single_point_sinr_linear(&data, receiver);
            if sinr > 0.0 {
                10.0 * sinr.log10()
            } else {
                -200.0
            }
        })
        .collect();

    if results_db.is_empty() {
        panic!("No sampling points found to compute CCDF");
    }

    let (ccdf_x, ccdf_y) = ccdf(&results_db, num_bins);

    create_dir_all("output").expect("Failed to create output directory");

    let csv_name = format!(
        "output/ccdf_from_optimized_{}_{}_per_km2.csv",
        mode_name, density_label
    );
    let csv_path = Path::new(&csv_name);
    let csv_file = File::create(csv_path).expect("Failed to create CSV output");
    let mut csv_writer = Writer::from_writer(csv_file);
    csv_writer.write_record(["theta", "probability"]).unwrap();
    for (x_value, y_value) in ccdf_x.iter().zip(ccdf_y.iter()) {
        csv_writer
            .write_record([x_value.to_string(), y_value.to_string()])
            .unwrap();
    }
    csv_writer.flush().unwrap();
    println!("Wrote {} points to {}", ccdf_x.len(), csv_name);

    let svg_name = format!(
        "output/ccdf_from_optimized_{}_{}_per_km2.svg",
        mode_name, density_label
    );
    let root = SVGBackend::new(&svg_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let y_minimum = ccdf_y.iter().cloned().fold(f64::INFINITY, f64::min);
    let mut y_maximum = ccdf_y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if (y_maximum - y_minimum).abs() < f64::EPSILON {
        y_maximum = y_minimum + 1e-6;
    }

    let x_minimum = *ccdf_x.first().unwrap();
    let mut x_maximum = *ccdf_x.last().unwrap();
    if (x_maximum - x_minimum).abs() < f64::EPSILON {
        x_maximum = x_minimum + 1e-6;
    }

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "Spatial CCDF from Optimized Deployment ({} mode, {} BS/km^2)",
                mode_name.to_uppercase(),
                density_label
            ),
            ("sans-serif", 20),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(x_minimum..x_maximum, y_minimum..y_maximum)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("theta (dB)")
        .y_desc("Probability")
        .draw()
        .unwrap();

    chart
        .draw_series(plotters::series::LineSeries::new(
            ccdf_x
                .iter()
                .zip(ccdf_y.iter())
                .map(|(&x_value, &y_value)| (x_value, y_value)),
            &RED,
        ))
        .unwrap()
        .label("Optimized Deployment")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
    println!(
        "Wrote {} (sampled {} road points)",
        svg_name,
        results_db.len()
    );
}

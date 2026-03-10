use network_comms_sim::{sim::{SimulationData, simulate_coverage_ccdf}};
use std::env;
use std::fs::{File, create_dir_all};
use std::path::Path;
use csv::Writer;
use plotters::prelude::*;
use plotters_svg::SVGBackend;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Command line args and explanation
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("Usage: {} [--los|--nlos] [density_a] [density_b]", args[0]);
        println!("  --los      Use LOS-only model");
        println!("  --nlos     Use NLOS + diffraction model (default)");
        println!("  density_a  First base station density (default: 20)");
        println!("  density_b  Second base station density (default: 100)");
        println!("\nExamples:");
        println!("  {} --nlos 20 100", args[0]);
        println!("  {} --los 10 50", args[0]);
        return;
    }

    let mut use_nlos = true;
    let mut density_args: Vec<f64> = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--los" => use_nlos = false,
            "--nlos" => use_nlos = true,
            _ => {
                if let Ok(value) = arg.parse::<f64>() {
                    density_args.push(value);
                } else {
                    eprintln!("Ignoring unrecognized argument: {}", arg);
                }
            }
        }
    }

    // Two densities that will be compared as two CCDFs
    let density_a = *density_args.first().unwrap_or(&20.0);
    let density_b = *density_args.get(1).unwrap_or(&100.0);

    if density_a <= 0.0 || density_b <= 0.0 {
        eprintln!("Both base station densities must be positive.");
        return;
    }

    // Randomly generate Manhattan layout (Poisson PPP) similar to manhattan.m
    let grid_size = 5000.0;
    let avenue_density = 7.0 / 1000.0;
    let street_density = 7.0 / 1000.0;

    let mut data: SimulationData = Default::default();
    data.size = grid_size;
    data.lambda_ave = avenue_density;
    data.lambda_st = street_density;
    data.use_nlos = use_nlos;
    data.use_diffraction = use_nlos;
    data.connect_to_nlos = use_nlos;

    let simulations = 1e6 as usize;
    let num_bins = 500;
    
    // Run first density CCDF
    let mut data_a = data.clone();
    data_a.lambda_base = density_a;
    let (ccdf_x_a, ccdf_y_a) = simulate_coverage_ccdf(&mut data_a, simulations, num_bins, true);

    // Run second density CCDF
    let mut data_b = data.clone();
    data_b.lambda_base = density_b;
    let (ccdf_x_b, ccdf_y_b) = simulate_coverage_ccdf(&mut data_b, simulations, num_bins, true);

    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");

    let mode_name = if use_nlos { "nlos" } else { "los" };

    // Output CSV for first density
    let csv_a = format!("output/ccdf_{}_{}_per_km2.csv", mode_name, density_a as i32);
    let path = Path::new(&csv_a);
    let file = File::create(path).expect("create csv");
    let mut csv_writer = Writer::from_writer(file);
    csv_writer.write_record(["theta", "probability"]).unwrap();
    for (x_value, y_value) in ccdf_x_a.iter().zip(ccdf_y_a.iter()) {
        csv_writer.write_record([x_value.to_string(), y_value.to_string()]).unwrap();
    }
    csv_writer.flush().unwrap();
    println!("Wrote {} points to {}", ccdf_x_a.len(), csv_a);

    // Output CSV for second density
    let csv_b = format!("output/ccdf_{}_{}_per_km2.csv", mode_name, density_b as i32);
    let path = Path::new(&csv_b);
    let file = File::create(path).expect("create csv");
    let mut csv_writer = Writer::from_writer(file);
    csv_writer.write_record(["theta", "probability"]).unwrap();
    for (x_value, y_value) in ccdf_x_b.iter().zip(ccdf_y_b.iter()) {
        csv_writer.write_record([x_value.to_string(), y_value.to_string()]).unwrap();
    }
    csv_writer.flush().unwrap();
    println!("Wrote {} points to {}", ccdf_x_b.len(), csv_b);

    // Plot combined SVG
    let name = format!(
        "output/ccdf_compare_{}_{}_vs_{}_per_km2.svg",
        mode_name,
        density_a as i32,
        density_b as i32
    );
    let root = SVGBackend::new(&name, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    
    // Find min/max across both density curves
    let y_minimum = ccdf_y_a
        .iter()
        .chain(ccdf_y_b.iter())
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let y_maximum = ccdf_y_a
        .iter()
        .chain(ccdf_y_b.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let x_minimum = ccdf_x_a.first().unwrap().min(*ccdf_x_b.first().unwrap());
    let x_maximum = ccdf_x_a.last().unwrap().max(*ccdf_x_b.last().unwrap());
    
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "Coverage Probability CCDF ({}) : {} vs {} BS/km^2",
                if use_nlos { "NLOS + Diffraction" } else { "LOS Only" },
                density_a as i32,
                density_b as i32
            ),
            ("sans-serif", 20),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(x_minimum..x_maximum, y_minimum..y_maximum).unwrap();
    chart.configure_mesh()
        .x_desc("θ (dB)")
        .y_desc("Probability")
        .draw().unwrap();
    
    // Plot first density in blue
    chart.draw_series(plotters::series::LineSeries::new(
        ccdf_x_a.iter().zip(ccdf_y_a.iter()).map(|(&x_val,&y_val)|(x_val,y_val)),
        &BLUE
    ))
    .unwrap()
    .label(format!("{} BS/km^2", density_a as i32))
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Plot second density in red
    chart.draw_series(plotters::series::LineSeries::new(
        ccdf_x_b.iter().zip(ccdf_y_b.iter()).map(|(&x_val,&y_val)|(x_val,y_val)),
        &RED
    ))
    .unwrap()
    .label(format!("{} BS/km^2", density_b as i32))
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();
    
    root.present().unwrap();
    println!("Wrote {}", name);
}

use network_comms_sim::{sim::{SimulationData, simulate_coverage_ccdf}};
use std::env;
use std::fs::{File, create_dir_all};
use std::path::Path;
use csv::Writer;
use plotters::prelude::*;
use plotters_svg::SVGBackend;

fn main() {
    // Randomly generate Manhattan layout (Poisson PPP) similar to manhattan.m
    let grid_size = 5000.0;
    let avenue_density = 7.0 / 1000.0;
    let street_density = 7.0 / 1000.0;
    let base_station_density = 20.0;

    let mut data: SimulationData = Default::default();
    data.size = grid_size;
    data.lambda_ave = avenue_density;
    data.lambda_st = street_density;
    data.lambda_base = base_station_density;

    let simulations = 1e4 as usize;
    let num_bins = simulations / 200;
    
    // Run simulation with NLOS and diffraction
    data.use_nlos = true;
    data.use_diffraction = true;
    let (ccdf_x_nlos, ccdf_y_nlos) = simulate_coverage_ccdf(&mut data, simulations, num_bins, true);
    
    // Run simulation with LOS only
    data.use_nlos = false;
    data.use_diffraction = false;
    let (ccdf_x_los, ccdf_y_los) = simulate_coverage_ccdf(&mut data, simulations, num_bins, true);

    // Ensure output directory exists
    create_dir_all("output").expect("Failed to create output directory");

    let args: Vec<String> = env::args().collect();
    // Output CSV if requested (NLOS data)
    if args.iter().any(|a| a == "--output-csv") {
        let name = format!("output/ccdf_{}_per_km2_nlos.csv", base_station_density as i32);
        let path = Path::new(&name);
        let file = File::create(path).expect("create csv");
        let mut csv_writer = Writer::from_writer(file);
        csv_writer.write_record(["theta", "probability"]).unwrap();
        for (x_value, y_value) in ccdf_x_nlos.iter().zip(ccdf_y_nlos.iter()) {
            csv_writer.write_record([x_value.to_string(), y_value.to_string()]).unwrap();
        }
        csv_writer.flush().unwrap();
        println!("Wrote {} points to {}", ccdf_x_nlos.len(), name);
        
        // Output LOS data
        let name = format!("output/ccdf_{}_per_km2_los.csv", base_station_density as i32);
        let path = Path::new(&name);
        let file = File::create(path).expect("create csv");
        let mut csv_writer = Writer::from_writer(file);
        csv_writer.write_record(["theta", "probability"]).unwrap();
        for (x_value, y_value) in ccdf_x_los.iter().zip(ccdf_y_los.iter()) {
            csv_writer.write_record([x_value.to_string(), y_value.to_string()]).unwrap();
        }
        csv_writer.flush().unwrap();
        println!("Wrote {} points to {}", ccdf_x_los.len(), name);
    }

    // Plot SVG if requested
    if args.iter().any(|a| a == "--plot-svg") {
        let name = format!("output/ccdf_{}_per_km2.svg", base_station_density);
        let root = SVGBackend::new(&name, (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        
        // Find min/max for both datasets
        let y_minimum = ccdf_y_nlos.iter().chain(ccdf_y_los.iter()).cloned().fold(f64::INFINITY, f64::min);
        let y_maximum = ccdf_y_nlos.iter().chain(ccdf_y_los.iter()).cloned().fold(f64::NEG_INFINITY, f64::max);
        let x_minimum = ccdf_x_nlos.first().unwrap().min(*ccdf_x_los.first().unwrap());
        let x_maximum = ccdf_x_nlos.last().unwrap().max(*ccdf_x_los.last().unwrap());
        
        let mut chart = ChartBuilder::on(&root)
            .caption("Coverage Probability CCDF: NLOS+Diffraction vs LOS Only", ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(x_minimum..x_maximum, y_minimum..y_maximum).unwrap();
        chart.configure_mesh()
            .x_desc("θ (dB)")
            .y_desc("Probability")
            .draw().unwrap();
        
        // Plot NLOS+Diffraction curve in red
        chart.draw_series(plotters::series::LineSeries::new(
            ccdf_x_nlos.iter().zip(ccdf_y_nlos.iter()).map(|(&x_val,&y_val)|(x_val,y_val)), 
            &RED
        ))
        .unwrap()
        .label("NLOS + Diffraction")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        
        // Plot LOS only curve in blue
        chart.draw_series(plotters::series::LineSeries::new(
            ccdf_x_los.iter().zip(ccdf_y_los.iter()).map(|(&x_val,&y_val)|(x_val,y_val)), 
            &BLUE
        ))
        .unwrap()
        .label("LOS Only")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        
        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();
        
        root.present().unwrap();
        println!("Wrote {}", name);
    }

    // Print summary if no output options specified
    if !args.iter().any(|a| a == "--output-csv" || a == "--plot-svg") {
        println!("NLOS+Diffraction: x_len={} y_len={} y_first={:.3} y_last={:.3}", 
                 ccdf_x_nlos.len(), ccdf_y_nlos.len(), ccdf_y_nlos.first().unwrap(), ccdf_y_nlos.last().unwrap());
        println!("LOS Only: x_len={} y_len={} y_first={:.3} y_last={:.3}", 
                 ccdf_x_los.len(), ccdf_y_los.len(), ccdf_y_los.first().unwrap(), ccdf_y_los.last().unwrap());
        println!("Pass --output-csv to export to output/ccdf.csv or --plot-svg to plot to output/ccdf.svg");
    }
}

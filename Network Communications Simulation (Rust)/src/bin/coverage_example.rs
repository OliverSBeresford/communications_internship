use network_comms_sim::sim::{SimulationData, sinr_linear};

fn main() {
    let mut data: SimulationData = Default::default();
    data.generate_layout();

    println!("Layout summary: {}", data.layout_summary());
    println!("sinr_db={:.2}", 10.0 * sinr_linear(&mut data).log10());
}

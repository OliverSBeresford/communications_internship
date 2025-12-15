use network_comms_sim::{geom::Point, metrics::{BaseStation, LinkType, received_power_dbm, sinr_db}, rf::ChannelParams};
use rand::SeedableRng;
use rand::rngs::StdRng;

fn main() {
    let mut random_generator = StdRng::seed_from_u64(42);
    let channel_params = ChannelParams {
        path_loss_exponent_los: 2.0,
        path_loss_exponent_nlos: 3.5,
        reference_distance_m: 1.0,
        reference_loss_db: 40.0,
        shadowing_std_db: 4.0,
    };

    let base_stations = vec![
        BaseStation { pos: Point { x: 0.0, y: 0.0 }, tx_power_dbm: 30.0 },
        BaseStation { pos: Point { x: 100.0, y: 0.0 }, tx_power_dbm: 30.0 },
    ];

    let user_position = Point { x: 10.0, y: 5.0 };

    let signal_power_dbm = received_power_dbm(&mut random_generator, user_position, &base_stations[0], LinkType::LOS, &channel_params);
    let interference_power_list = vec![received_power_dbm(&mut random_generator, user_position, &base_stations[1], LinkType::NLOS, &channel_params)];
    let noise_power_dbm = -100.0;
    let sinr_value = sinr_db(signal_power_dbm, &interference_power_list, noise_power_dbm);

    println!("signal_dbm={:.2} interference_dbm={:.2} sinr_db={:.2}", signal_power_dbm, interference_power_list[0], sinr_value);
}

use network_comms_sim::{geom::{Point, euclidean_distance, manhattan_distance, nearest_base_station, nearest_los}, metrics::{dbm_to_mw, mw_to_dbm, BaseStation, LinkType, received_power_dbm, sinr_db, ccdf}, rf::ChannelParams};
use network_comms_sim::sim::generate_manhattan;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[test]
fn geom_distances() {
    let point_a = Point { x: 0.0, y: 0.0 };
    let point_b = Point { x: 3.0, y: 4.0 };
    assert!((euclidean_distance(point_a, point_b) - 5.0).abs() < 1e-12);
    assert!((manhattan_distance(point_a, point_b) - 7.0).abs() < 1e-12);
}

#[test]
fn power_and_sinr() {
    let mut random_generator = StdRng::seed_from_u64(1);
    let channel_params = ChannelParams {
        path_loss_exponent_los: 2.0,
        path_loss_exponent_nlos: 3.0,
        reference_distance_m: 1.0,
        reference_loss_db: 40.0,
        shadowing_std_db: 2.0,
    };
    let base_station_0 = BaseStation { pos: Point { x: 0.0, y: 0.0 }, tx_power_dbm: 30.0 };
    let base_station_1 = BaseStation { pos: Point { x: 50.0, y: 0.0 }, tx_power_dbm: 30.0 };
    let user_position = Point { x: 5.0, y: 0.0 };

    let signal_power = received_power_dbm(&mut random_generator, user_position, &base_station_0, LinkType::LOS, &channel_params);
    let interference_power = received_power_dbm(&mut random_generator, user_position, &base_station_1, LinkType::NLOS, &channel_params);
    let sinr = sinr_db(signal_power, &[interference_power], -100.0);
    assert!(sinr.is_finite());
}

#[test]
fn ccdf_basic() {
    let sample_data: Vec<f64> = (0..1000).map(|i| i as f64 / 10.0).collect();
    let (ccdf_x, ccdf_y) = ccdf(&sample_data, 50);
    assert_eq!(ccdf_x.len(), 50);
    assert_eq!(ccdf_y.len(), 50);
    // CCDF should decrease from near 1 to near 0
    assert!(ccdf_y.first().unwrap() > ccdf_y.last().unwrap());
}

#[test]
fn nearest_helpers() {
    let base_stations = vec![Point { x: 0.0, y: 0.0 }, Point { x: 5.0, y: 0.0 }, Point { x: 0.0, y: 5.0 }];
    let receiver = Point { x: 1.0, y: 0.0 };
    let (nearest_idx, _, _) = nearest_base_station(receiver, &base_stations).unwrap();
    assert_eq!(nearest_idx, 0);
    let (nearest_los_idx, _, _) = nearest_los(receiver, &base_stations).unwrap();
    // Both (0,0) and (5,0) share y=0, but (0,0) is closer
    assert_eq!(nearest_los_idx, 0);
}

#[test]
fn manhattan_generator_contains_zero_street() {
    let layout = generate_manhattan(100.0, 0.01, 0.01, 0.005, 123, false);
    assert!(layout.streets.iter().any(|&street_y| (street_y - 0.0).abs() < 1e-9));
}

#[test]
fn dbm_mw_roundtrip() {
    let dbm_value = -30.0;
    let milliwatts = dbm_to_mw(dbm_value);
    let roundtrip_dbm = mw_to_dbm(milliwatts);
    assert!((dbm_value - roundtrip_dbm).abs() < 1e-9);
}

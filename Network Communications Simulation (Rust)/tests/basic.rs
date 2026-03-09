use network_comms_sim::{geom::{Point, euclidean_distance, manhattan_distance, nearest_base_station, nearest_los}, metrics::{ccdf}};
use network_comms_sim::sim::generate_manhattan;

#[test]
fn geom_distances() {
    let point_a = Point { x: 0.0, y: 0.0 };
    let point_b = Point { x: 3.0, y: 4.0 };
    assert!((euclidean_distance(point_a, point_b) - 5.0).abs() < 1e-12);
    assert!((manhattan_distance(point_a, point_b) - 7.0).abs() < 1e-12);
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

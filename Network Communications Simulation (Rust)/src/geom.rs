use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Calculate Euclidean distance between two points
pub fn euclidean_distance(point_a: Point, point_b: Point) -> f64 {
    let delta_x = point_a.x - point_b.x;
    let delta_y = point_a.y - point_b.y;
    (delta_x * delta_x + delta_y * delta_y).sqrt()
}

/// Calculate Manhattan distance between two points (just linear)
pub fn manhattan_distance(point_a: Point, point_b: Point) -> f64 {
    (point_a.x - point_b.x).abs() + (point_a.y - point_b.y).abs()
}

/// Find the nearest point from a target among a list of points
pub fn nearest_point(target: Point, points: &[Point]) -> Option<(usize, Point, f64)> {
    let mut best_match: Option<(usize, Point, f64)> = None;
    for (index, &point) in points.iter().enumerate() {
        let distance = euclidean_distance(target, point);
        if let Some((_, _, best_distance)) = best_match {
            if distance < best_distance {
                best_match = Some((index, point, distance));
            }
        } else {
            best_match = Some((index, point, distance));
        }
    }
    best_match
}

/// Find the nearest point using Manhattan distance
pub fn nearest_point_manhattan(target: Point, points: &[Point]) -> Option<(usize, Point, f64)> {
    let mut best_match: Option<(usize, Point, f64)> = None;
    for (index, &point) in points.iter().enumerate() {
        let distance = manhattan_distance(target, point);
        if let Some((_, _, best_distance)) = best_match {
            if distance < best_distance {
                best_match = Some((index, point, distance));
            }
        } else {
            best_match = Some((index, point, distance));
        }
    }
    best_match
}

/// Find the nearest base station (point) to a receiver
pub fn nearest_base_station(receiver: Point, bases: &[Point]) -> Option<(usize, Point, f64)> {
    nearest_point(receiver, bases)
}

/// Find the nearest line-of-sight base station (point) to a receiver
pub fn nearest_los(receiver: Point, bases: &[Point]) -> Option<(usize, Point, f64)> {
    let mut best_match: Option<(usize, Point, f64)> = None;
    for (index, &point) in bases.iter().enumerate() {
        if receiver.x == point.x || receiver.y == point.y {
            let distance = euclidean_distance(receiver, point);
            if let Some((_, _, best_distance)) = best_match {
                if distance < best_distance {
                    best_match = Some((index, point, distance));
                }
            } else {
                best_match = Some((index, point, distance));
            }
        }
    }
    best_match
}

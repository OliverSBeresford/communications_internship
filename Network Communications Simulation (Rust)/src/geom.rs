use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn euclidean_distance(point_a: Point, point_b: Point) -> f64 {
    let delta_x = point_a.x - point_b.x;
    let delta_y = point_a.y - point_b.y;
    (delta_x * delta_x + delta_y * delta_y).sqrt()
}

pub fn manhattan_distance(point_a: Point, point_b: Point) -> f64 {
    (point_a.x - point_b.x).abs() + (point_a.y - point_b.y).abs()
}

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

pub fn nearest_base_station(receiver: Point, bases: &[Point]) -> Option<(usize, Point, f64)> {
    nearest_point(receiver, bases)
}

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

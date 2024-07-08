#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point {
            x,
            y,
        }
    }
    pub fn manhattan_distance(point_1: &Point, point_2: &Point) -> u32 {
        point_1.x.abs_diff(point_2.x) + point_1.y.abs_diff(point_2.y)
    }
}


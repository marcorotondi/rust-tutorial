
mod model;

fn main() {
    let point_1 = model::Point::new(5, 5);
    let point_2 = model::Point::new(10, 10);
    let manhattan_distance = model::Point::manhattan_distance(&point_1, &point_2);

    println!("Manhattan distance from {point_1:?} and {point_2:?} is: {manhattan_distance}")
}

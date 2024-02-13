struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point {x: 5, y: 10};

    println!("p.x is {}", p.x());

    let float_point = Point{x: 1.0, y: 2.0};

    let dfo = float_point.distance_from_origin();

    println!("The distances from origin of dfo is {}", dfo);
}

use std::f64::consts::PI;

trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}

struct Rectangle {length: f32, width: f32}
struct Circle {length: f32, width: f32}

impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Rectangle {
        return Rectangle{length, width};
    }

    fn area(&self) -> f32 {
        return self.length * self.width
    }
}

impl Shape for Circle {
    fn new(length: f32, width: f32) -> Circle {
        return Circle{length, width};
    }

    fn area(&self) -> f32 {
        return (self.length / 2.0).powf(2.0) * 3.14592;
    }
}

fn main() {
    let rec:Rectangle = Shape::new(10.0, 10.0);
    let circle:Circle = Shape::new(10.0, 10.0);

    println!("Rec area: {}", rec.area());
    println!("Circle area: {}", circle.area());

}

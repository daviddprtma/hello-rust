use std::f64::consts::PI;

enum Shape<T> {
    Circle(T),
    Triangle(T, T),
    Rectangle(T, T),
}

impl<T: Into<f64> + Copy> Shape<T> {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(diameter) => {
                let radius = (*diameter).into() / 2.0;
                PI * radius.powf(2.0)
            }
            Shape::Triangle(base, height) => {
                let base_f64: f64 = (*base).into();
                let height_f64: f64 = (*height).into();
                (base_f64 * height_f64) / 2.0
            }
            Shape::Rectangle(width, length) => {
                let width_f64: f64 = (*width).into();
                let length_f64: f64 = (*length).into();
                width_f64 * length_f64
            }
        }
    }
}

fn main() {
    let base: u8 = 24;
    let height: u8 = 24;
    let triangle = Shape::Triangle(base, height);
    let triangle_area = triangle.area();

    let width: u8 = 12;
    let length: u8 = 24;
    let rectangle = Shape::Rectangle(width, length);
    let rectangular_area = rectangle.area();

    let diameter: u8 = 45;
    let circle = Shape::Circle(diameter);
    let circle_area = circle.area();

    println!(
        "The area of the triangle with a base of {} and a height of {} is {:.5}",
        base, height, triangle_area
    );
    println!(
        "The area of the rectangle with a width of {} and a length of {} is {:.5}",
        width, length, rectangular_area
    );
    println!(
        "The area of the circle with a diameter of {} is {:.5}",
        diameter, circle_area
    );
}
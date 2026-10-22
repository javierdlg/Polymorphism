use std::{f64::consts::PI, process::ExitCode};

// Rust has no class inheritance. This trait supplies behavior, not shared fields.
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn areas() -> [f64; 2] {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 3.0,
    };
    let shapes: [&dyn Shape; 2] = [&circle, &rectangle];
    // TOUR 1: borrowed trait objects request dynamic dispatch, not inheritance.
    shapes.map(|shape| shape.area())
}

// TOUR 2 / TRY: add a required describe method to Shape (without a default).
// Both implementations must change. The next stage separates data and operations.
fn verify() -> bool {
    let [circle, rectangle] = areas();
    (circle - PI * 9.0).abs() < 1e-10 && (rectangle - 12.0).abs() < 1e-10
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !(args.is_empty() || args.len() == 1 && args[0] == "--verify") {
        eprintln!("Usage: rust-problem [--verify]");
        return ExitCode::from(2);
    }

    if !verify() {
        eprintln!("Verification failed: shape areas.");
        return ExitCode::FAILURE;
    }
    if args.is_empty() {
        let [circle, rectangle] = areas();
        println!("Circle area: {circle:.2}");
        println!("Rectangle area: {rectangle:.2}");
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_shapes_preserve_area_formulas() {
        let [circle, rectangle] = areas();
        assert!((circle - PI * 9.0).abs() < 1e-10);
        assert!((rectangle - 12.0).abs() < 1e-10);
    }

    #[test]
    fn area_uses_each_shapes_dimensions() {
        assert!((Circle { radius: 2.0 }.area() - PI * 4.0).abs() < 1e-10);
        assert!(
            (Rectangle {
                width: 2.0,
                height: 5.0
            }
            .area()
                - 10.0)
                .abs()
                < 1e-10
        );
        assert!(verify());
    }
}

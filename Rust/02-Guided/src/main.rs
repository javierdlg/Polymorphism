use std::{f64::consts::PI, process::ExitCode};

// TOUR 1: a sum type with product-type payloads; data has no behavior methods.
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn calculate_area(shape: &Shape) -> f64 {
    // TOUR 2: exhaustive case analysis keeps this operation outside the data.
    match shape {
        Shape::Circle { radius } => PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
    }
}

// TRY: add describe(&Shape) -> String without modifying Shape or calculate_area.
// Handle both variants explicitly; do not hide a future variant with a wildcard.
fn areas() -> [f64; 2] {
    [
        calculate_area(&Shape::Circle { radius: 3.0 }),
        calculate_area(&Shape::Rectangle {
            width: 4.0,
            height: 3.0,
        }),
    ]
}

fn verify() -> bool {
    let [circle, rectangle] = areas();
    (circle - PI * 9.0).abs() < 1e-10 && (rectangle - 12.0).abs() < 1e-10
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !(args.is_empty() || args.len() == 1 && args[0] == "--verify") {
        eprintln!("Usage: rust-guided [--verify]");
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
    fn each_variant_has_its_area_formula() {
        let [circle, rectangle] = areas();
        assert!((circle - PI * 9.0).abs() < 1e-10);
        assert!((rectangle - 12.0).abs() < 1e-10);
        assert!(verify());
    }

    #[test]
    fn operation_reads_payload_dimensions() {
        assert!((calculate_area(&Shape::Circle { radius: 2.0 }) - PI * 4.0).abs() < 1e-10);
        assert!(
            (calculate_area(&Shape::Rectangle {
                width: 2.0,
                height: 5.0
            }) - 10.0)
                .abs()
                < 1e-10
        );
    }
}

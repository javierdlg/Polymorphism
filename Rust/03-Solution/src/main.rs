use std::{f64::consts::PI, process::ExitCode};

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base_length: f64, height: f64 },
}

fn calculate_area(shape: &Shape) -> f64 {
    // TOUR 1 / BREAKPOINT: inspect the variant and payload; no virtual area method.
    match shape {
        Shape::Circle { radius } => PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle {
            base_length,
            height,
        } => base_length * height / 2.0,
    }
}

fn describe(shape: &Shape) -> String {
    // TOUR 2 / BREAKPOINT: first matching arm wins; guards need general arms after them.
    match shape {
        Shape::Circle { radius } if *radius > 100.0 => format!("Large circle: {radius}"),
        Shape::Circle { radius } => format!("Standard circle: {radius}"),
        Shape::Rectangle { width, height } if width == height => {
            format!("Square: {width}x{height}")
        }
        Shape::Rectangle { width, height } => format!("Rectangle: {width}x{height}"),
        Shape::Triangle {
            base_length,
            height,
        } => format!("Triangle: base {base_length}, height {height}"),
    }
}

fn to_svg(shape: &Shape) -> String {
    match shape {
        Shape::Circle { radius } => format!("<circle r=\"{radius}\" />"),
        Shape::Rectangle { width, height } => {
            format!("<rect width=\"{width}\" height=\"{height}\" />")
        }
        Shape::Triangle {
            base_length,
            height,
        } => format!("<polygon points=\"0,0 {base_length},0 0,{height}\" />"),
    }
}

fn example(number: u8) -> String {
    match number {
        1 => format!(
            "Circle area: {:.2}\nRectangle area: {:.2}",
            calculate_area(&Shape::Circle { radius: 3.0 }),
            calculate_area(&Shape::Rectangle {
                width: 4.0,
                height: 3.0
            }),
        ),
        2 => [
            Shape::Circle { radius: 101.0 },
            Shape::Circle { radius: 100.0 },
            Shape::Rectangle {
                width: 4.0,
                height: 4.0,
            },
            Shape::Rectangle {
                width: 4.0,
                height: 3.0,
            },
            Shape::Triangle {
                base_length: 4.0,
                height: 3.0,
            },
        ]
        .iter()
        .map(describe)
        .collect::<Vec<_>>()
        .join("\n"),
        3 => [
            Shape::Circle { radius: 3.0 },
            Shape::Rectangle {
                width: 4.0,
                height: 3.0,
            },
        ]
        .iter()
        .map(to_svg)
        .collect::<Vec<_>>()
        .join("\n"),
        4 => format!(
            "Triangle area: {:.2}",
            calculate_area(&Shape::Triangle {
                base_length: 4.0,
                height: 3.0
            })
        ),
        _ => unreachable!("example number was validated by the caller"),
    }
}

fn verify() -> bool {
    let circle = Shape::Circle { radius: 3.0 };
    let rectangle = Shape::Rectangle {
        width: 4.0,
        height: 3.0,
    };
    let triangle = Shape::Triangle {
        base_length: 4.0,
        height: 3.0,
    };
    (calculate_area(&circle) - PI * 9.0).abs() < 1e-10
        && (calculate_area(&rectangle) - 12.0).abs() < 1e-10
        && (calculate_area(&triangle) - 6.0).abs() < 1e-10
        && describe(&circle) == "Standard circle: 3"
        && example(1) == "Circle area: 28.27\nRectangle area: 12.00"
        && example(2)
            == "Large circle: 101\nStandard circle: 100\nSquare: 4x4\nRectangle: 4x3\nTriangle: base 4, height 3"
        && example(3) == "<circle r=\"3\" />\n<rect width=\"4\" height=\"3\" />"
        && example(4) == "Triangle area: 6.00"
        && to_svg(&triangle) == "<polygon points=\"0,0 4,0 0,3\" />"
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 1 && args[0] == "--verify" {
        if !verify() {
            eprintln!("Verification failed: shape operations.");
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }
    if args.len() == 1 && args[0] == "--all" {
        for number in 1..=4 {
            println!("{}", example(number));
        }
        return ExitCode::SUCCESS;
    }
    if args.is_empty() {
        println!("{}", example(1));
        return ExitCode::SUCCESS;
    }
    if args.len() == 2 && args[0] == "--lesson" {
        let number = match args[1].as_str() {
            "1" => Some(1),
            "2" => Some(2),
            "3" => Some(3),
            "4" => Some(4),
            _ => None,
        };
        if let Some(number) = number {
            println!("{}", example(number));
            return ExitCode::SUCCESS;
        }
    }
    eprintln!("Usage: rust-solution [--lesson 1|2|3|4 | --all | --verify]");
    ExitCode::from(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas_follow_all_variant_formulas() {
        for (shape, expected) in [
            (Shape::Circle { radius: 3.0 }, PI * 9.0),
            (
                Shape::Rectangle {
                    width: 4.0,
                    height: 3.0,
                },
                12.0,
            ),
            (
                Shape::Triangle {
                    base_length: 4.0,
                    height: 3.0,
                },
                6.0,
            ),
            (
                Shape::Triangle {
                    base_length: 5.0,
                    height: 7.0,
                },
                17.5,
            ),
        ] {
            assert!((calculate_area(&shape) - expected).abs() < 1e-10);
        }
    }

    #[test]
    fn descriptions_cover_guards_boundaries_and_all_variants() {
        for (shape, expected) in [
            (Shape::Circle { radius: 101.0 }, "Large circle: 101"),
            (Shape::Circle { radius: 100.0 }, "Standard circle: 100"),
            (Shape::Circle { radius: 3.0 }, "Standard circle: 3"),
            (
                Shape::Rectangle {
                    width: 4.0,
                    height: 4.0,
                },
                "Square: 4x4",
            ),
            (
                Shape::Rectangle {
                    width: 4.0,
                    height: 3.0,
                },
                "Rectangle: 4x3",
            ),
            (
                Shape::Triangle {
                    base_length: 4.0,
                    height: 3.0,
                },
                "Triangle: base 4, height 3",
            ),
        ] {
            assert_eq!(describe(&shape), expected);
        }
    }

    #[test]
    fn svg_serializes_each_variant() {
        for (shape, expected) in [
            (Shape::Circle { radius: 3.0 }, "<circle r=\"3\" />"),
            (
                Shape::Rectangle {
                    width: 4.0,
                    height: 3.0,
                },
                "<rect width=\"4\" height=\"3\" />",
            ),
            (
                Shape::Triangle {
                    base_length: 4.0,
                    height: 3.0,
                },
                "<polygon points=\"0,0 4,0 0,3\" />",
            ),
        ] {
            assert_eq!(to_svg(&shape), expected);
        }
    }

    #[test]
    fn examples_and_verification_preserve_transcripts() {
        assert_eq!(example(1), "Circle area: 28.27\nRectangle area: 12.00");
        assert_eq!(
            example(2),
            "Large circle: 101\nStandard circle: 100\nSquare: 4x4\nRectangle: 4x3\nTriangle: base 4, height 3"
        );
        assert_eq!(
            example(3),
            "<circle r=\"3\" />\n<rect width=\"4\" height=\"3\" />"
        );
        assert_eq!(example(4), "Triangle area: 6.00");
        assert!(verify());
    }
}

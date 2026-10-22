#include <algorithm>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <locale>
#include <numbers>
#include <string_view>
#include <variant>

struct Circle { double radius; };
struct Rectangle { double width; double height; };
using Shape = std::variant<Circle, Rectangle>;

struct AreaVisitor
{
	double operator()(const Circle& circle) const
	{
		return std::numbers::pi * circle.radius * circle.radius;
	}
	double operator()(const Rectangle& rectangle) const
	{
		return rectangle.width * rectangle.height;
	}
};

double area(const Shape& shape)
{
	// STOP 1: data has no methods; visit selects an explicit overload.
	return std::visit(AreaVisitor{}, shape);
}

bool close(double actual, double expected)
{
	return std::isfinite(actual)
		&& std::abs(actual - expected) <= 1e-12 * std::max(1.0, std::abs(expected));
}

int main(int argc, char* argv[])
{
	if (argc > 2 || (argc == 2 && std::string_view(argv[1]) != "--verify"))
	{
		std::cerr << "Usage: Cpp.Guided [--verify]\n";
		return 2;
	}
	const Shape circle = Circle{3};
	const Shape rectangle = Rectangle{4, 3};
	const double circle_area = area(circle);
	const double rectangle_area = area(rectangle);
	if (!close(circle_area, 28.274333882308138) || !close(rectangle_area, 12.0)
		|| !close(area(Circle{0.5}), 0.7853981633974483)
		|| !close(area(Rectangle{2.5, 1.5}), 3.75))
	{
		std::cerr << "Verification failed: visitor shape areas.\n";
		return 1;
	}
	if (argc == 1)
	{
		std::cout.imbue(std::locale::classic());
		std::cout << std::fixed << std::setprecision(2)
			<< "Circle area: " << circle_area << '\n'
			<< "Rectangle area: " << rectangle_area << '\n';
	}
	return 0;
}

// STOP 2 / EXERCISE: add describe(const Shape&) with an explicit visitor.
// Radius > 100: "Large circle: ..."; otherwise "Standard circle: ...".
// Equal sides: "Square: 4x4"; otherwise "Rectangle: 4x3".
// Do not modify Circle, Rectangle, Shape, or area. See the worked answer next.

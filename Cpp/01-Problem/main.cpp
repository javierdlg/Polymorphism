#include <algorithm>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <locale>
#include <numbers>
#include <string_view>

struct Shape
{
	virtual ~Shape() = default;
	virtual double area() const = 0;
};

class Circle final : public Shape
{
public:
	explicit Circle(double radius) : radius_(radius) {}
	double area() const override { return std::numbers::pi * radius_ * radius_; }
private:
	double radius_;
};

class Rectangle final : public Shape
{
public:
	Rectangle(double width, double height) : width_(width), height_(height) {}
	double area() const override { return width_ * height_; }
private:
	double width_;
	double height_;
};

bool close(double actual, double expected)
{
	return std::isfinite(actual)
		&& std::abs(actual - expected) <= 1e-12 * std::max(1.0, std::abs(expected));
}

int main(int argc, char* argv[])
{
	if (argc > 2 || (argc == 2 && std::string_view(argv[1]) != "--verify"))
	{
		std::cerr << "Usage: Cpp.Problem [--verify]\n";
		return 2;
	}
	const Circle circle{3};
	const Rectangle rectangle{4, 3};
	// STOP 1: borrowed base pointers select the concrete virtual area method.
	const Shape* shapes[] = { &circle, &rectangle };
	const double circle_area = shapes[0]->area();
	const double rectangle_area = shapes[1]->area();
	if (!close(circle_area, 28.274333882308138) || !close(rectangle_area, 12.0)
		|| !close(Circle{0.5}.area(), 0.7853981633974483)
		|| !close(Rectangle{2.5, 1.5}.area(), 3.75))
	{
		std::cerr << "Verification failed: virtual shape areas.\n";
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

// STOP 2: adding virtual describe() means editing Shape and every subclass.
// Try a guarded circle description and a square case. Must data own every operation?

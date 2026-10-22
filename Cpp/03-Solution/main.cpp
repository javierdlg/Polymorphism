#include "operations.h"
#include <algorithm>
#include <cmath>
#include <iostream>
#include <stdexcept>
#include <string>
#include <string_view>

std::string run(int lesson)
{
	switch (lesson)
	{
	case 1:
		return "Circle area: " + formatted_area(area(Circle{3})) + "\n"
			+ "Rectangle area: " + formatted_area(area(Rectangle{4, 3})) + "\n";
	case 2:
		return describe(Circle{101}) + "\n" + describe(Circle{100}) + "\n"
			+ describe(Rectangle{4, 4}) + "\n" + describe(Rectangle{4, 3}) + "\n"
			+ describe(Triangle{4, 3}) + "\n";
	case 3:
		return ToSvg(Circle{3}) + "\n" + ToSvg(Rectangle{4, 3}) + "\n";
	case 4:
		return "Triangle area: " + formatted_area(area(Triangle{4, 3})) + "\n";
	default: throw std::out_of_range("Unknown shape example number.");
	}
}

bool close(double actual, double expected)
{
	return std::isfinite(actual)
		&& std::abs(actual - expected) <= 1e-12 * std::max(1.0, std::abs(expected));
}

bool verify()
{
	bool passed = true;
	const auto expect = [&passed](bool condition, std::string_view label)
	{
		if (!condition)
		{
			std::cerr << "Verification failed: " << label << ".\n";
			passed = false;
		}
	};
	expect(close(area(Circle{3}), 28.274333882308138), "circle area");
	expect(close(area(Circle{0.5}), 0.7853981633974483), "fractional circle area");
	expect(close(area(Rectangle{4, 3}), 12.0), "rectangle area");
	expect(close(area(Rectangle{2.5, 1.5}), 3.75), "fractional rectangle area");
	expect(close(area(Triangle{4, 3}), 6.0), "triangle area");
	expect(close(area(Triangle{2.5, 1.5}), 1.875), "fractional triangle area");
	expect(describe(Circle{101}) == "Large circle: 101", "circle above guard");
	expect(describe(Circle{100}) == "Standard circle: 100", "circle at guard");
	expect(describe(Circle{99.5}) == "Standard circle: 99.5", "circle below guard");
	expect(describe(Rectangle{4, 4}) == "Square: 4x4", "square guard");
	expect(describe(Rectangle{4, 3}) == "Rectangle: 4x3", "non-square guard");
	expect(describe(Rectangle{3, 4}) == "Rectangle: 3x4", "reversed rectangle");
	expect(describe(Triangle{4, 3}) == "Triangle: base 4, height 3", "triangle description");
	expect(ToSvg(Circle{3}) == "<circle r=\"3\" />", "circle SVG");
	expect(ToSvg(Rectangle{4, 3}) == "<rect width=\"4\" height=\"3\" />", "rectangle SVG");
	expect(ToSvg(Triangle{4, 3}) == "<polygon points=\"0,0 4,0 0,3\" />", "triangle SVG");
	expect(ToSvg(Rectangle{2.5, 1.5}) == "<rect width=\"2.5\" height=\"1.5\" />", "fractional SVG");
	expect(dimension(3.0) == "3" && dimension(2.5) == "2.5", "concise dimensions");
	expect(formatted_area(12.0) == "12.00" && formatted_area(28.274333882308138) == "28.27",
		"fixed area formatting");
	const std::string_view expected[] = {
		"Circle area: 28.27\nRectangle area: 12.00\n",
		"Large circle: 101\nStandard circle: 100\nSquare: 4x4\nRectangle: 4x3\nTriangle: base 4, height 3\n",
		"<circle r=\"3\" />\n<rect width=\"4\" height=\"3\" />\n",
		"Triangle area: 6.00\n"
	};
	for (int lesson = 1; lesson <= 4; ++lesson)
	{
		expect(run(lesson) == expected[lesson - 1], "lesson " + std::to_string(lesson) + " transcript");
	}
	return passed;
}

int main(int argc, char* argv[])
{
	if (argc == 2 && std::string_view(argv[1]) == "--verify")
	{
		return verify() ? 0 : 1;
	}
	if (argc == 2 && std::string_view(argv[1]) == "--all")
	{
		for (int lesson = 1; lesson <= 4; ++lesson)
		{
			std::cout << run(lesson);
		}
		return 0;
	}
	if (argc == 1)
	{
		std::cout << run(1);
		return 0;
	}
	if (argc == 3 && std::string_view(argv[1]) == "--lesson")
	{
		const std::string_view number(argv[2]);
		if (number.size() == 1 && number[0] >= '1' && number[0] <= '4')
		{
			std::cout << run(number[0] - '0');
			return 0;
		}
	}
	std::cerr << "Usage: Cpp.Solution [--lesson 1|2|3|4 | --all | --verify]\n";
	return 2;
}
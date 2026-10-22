#pragma once
#include "shapes.h"
#include <iomanip>
#include <limits>
#include <locale>
#include <numbers>
#include <sstream>
#include <string>

inline std::string dimension(double value)
{
	std::ostringstream text;
	text.imbue(std::locale::classic());
	text << std::setprecision(std::numeric_limits<double>::digits10) << value;
	return text.str();
}

inline std::string formatted_area(double value)
{
	std::ostringstream text;
	text.imbue(std::locale::classic());
	text << std::fixed << std::setprecision(2) << value;
	return text.str();
}

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
	double operator()(const Triangle& triangle) const
	{
		return triangle.base * triangle.height / 2.0;
	}
};

inline double area(const Shape& shape)
{
	// TOUR 1 / BREAKPOINT: inspect the alternative, then enter its visitor overload.
	return std::visit(AreaVisitor{}, shape);
}

struct DescribeVisitor
{
	std::string operator()(const Circle& circle) const
	{
		return std::string(circle.radius > 100 ? "Large circle: " : "Standard circle: ")
			+ dimension(circle.radius);
	}
	std::string operator()(const Rectangle& rectangle) const
	{
		return std::string(rectangle.width == rectangle.height ? "Square: " : "Rectangle: ")
			+ dimension(rectangle.width) + "x" + dimension(rectangle.height);
	}
	std::string operator()(const Triangle& triangle) const
	{
		return "Triangle: base " + dimension(triangle.base) + ", height " + dimension(triangle.height);
	}
};

inline std::string describe(const Shape& shape)
{
	// TOUR 2 / BREAKPOINT: overload handles the type; its condition refines the data.
	return std::visit(DescribeVisitor{}, shape);
}

struct ToSvgVisitor
{
	std::string operator()(const Circle& circle) const
	{
		return "<circle r=\"" + dimension(circle.radius) + "\" />";
	}
	std::string operator()(const Rectangle& rectangle) const
	{
		return "<rect width=\"" + dimension(rectangle.width)
			+ "\" height=\"" + dimension(rectangle.height) + "\" />";
	}
	std::string operator()(const Triangle& triangle) const
	{
		return "<polygon points=\"0,0 " + dimension(triangle.base) + ",0 0,"
			+ dimension(triangle.height) + "\" />";
	}
};

inline std::string ToSvg(const Shape& shape)
{
	return std::visit(ToSvgVisitor{}, shape);
}

#pragma once
#include <variant>

// Fixtures have finite, positive dimensions. Values are immutable by convention.
struct Circle { double radius; };
struct Rectangle { double width; double height; };
struct Triangle { double base; double height; };
using Shape = std::variant<Circle, Rectangle, Triangle>;

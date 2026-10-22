using System.Globalization;

namespace CSharp.Problem;

public abstract class Shape
{
    public abstract double GetArea();
}

public sealed class Circle(double radius) : Shape
{
    public double Radius { get; } = radius;
    public override double GetArea() => Math.PI * Radius * Radius;
}

public sealed class Rectangle(double width, double height) : Shape
{
    public double Width { get; } = width;
    public double Height { get; } = height;
    public override double GetArea() => Width * Height;
}

internal static class ShapeTour
{
    private static int Main(string[] args)
    {
        if (args.Length > 1 || (args.Length == 1 && args[0] != "--verify"))
        {
            Console.Error.WriteLine("Usage: CSharp.Problem [--verify]");
            return 2;
        }
        Shape[] shapes = [new Circle(3), new Rectangle(4, 3)];
        double[] expected = [28.274333882308138, 12];
        for (int index = 0; index < shapes.Length; index++)
        {
            Shape shape = shapes[index];
            // TOUR 1 / BREAKPOINT: Shape is the static type; F11 selects an override.
            double area = shape.GetArea();
            if (!double.IsFinite(area) || Math.Abs(area - expected[index]) > 1e-10)
            {
                Console.Error.WriteLine("Verification failed: virtual area calculation.");
                return 1;
            }
            if (args.Length == 0)
            {
                Console.WriteLine($"{shape.GetType().Name} area: {area.ToString("F2", CultureInfo.InvariantCulture)}");
            }
        }
        return 0;
    }
}

// TOUR 2 / TRY: add an abstract Describe operation and implement it in both shapes.
// Adding an operation touches every subtype. Adding a subtype leaves callers alone.
// This is a trade-off, not broken code. All fixtures have finite positive dimensions.
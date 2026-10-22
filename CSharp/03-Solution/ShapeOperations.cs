using System.Globalization;

namespace CSharp.Solution;

public static class ShapeOperations
{
    public static double CalculateArea(Shape shape)
    {
        // TOUR 1 / BREAKPOINT: inspect the data, then follow its matching arm.
        return shape switch
        {
            Shape.Circle c => Math.PI * c.Radius * c.Radius,
            Shape.Rectangle(var w, var h) => w * h,
            Shape.Triangle(var b, var h) => 0.5 * b * h,
            _ => throw new ArgumentOutOfRangeException(nameof(shape))
        };
    }

    public static string Describe(Shape shape) => shape switch
    {
        // TOUR 2: a property + relational pattern; first matching arm wins.
        Shape.Circle { Radius: > 100 } c => $"Large circle: {Dimension(c.Radius)}",
        Shape.Circle c => $"Standard circle: {Dimension(c.Radius)}",
        Shape.Rectangle(var w, var h) when w == h => $"Square: {Dimension(w)}x{Dimension(h)}",
        Shape.Rectangle(var w, var h) => $"Rectangle: {Dimension(w)}x{Dimension(h)}",
        Shape.Triangle(var b, var h) => $"Triangle: base {Dimension(b)}, height {Dimension(h)}",
        _ => throw new ArgumentOutOfRangeException(nameof(shape))
    };

    // Another pure operation: no changes to Shape or its variants were required.
    public static string ToSvg(Shape shape) => shape switch
    {
        Shape.Circle(var r) => $"<circle r=\"{Dimension(r)}\" />",
        Shape.Rectangle(var w, var h) => $"<rect width=\"{Dimension(w)}\" height=\"{Dimension(h)}\" />",
        Shape.Triangle(var b, var h) => $"<polygon points=\"0,0 {Dimension(b)},0 0,{Dimension(h)}\" />",
        _ => throw new ArgumentOutOfRangeException(nameof(shape))
    };

    private static string Dimension(double value) => value.ToString("G", CultureInfo.InvariantCulture);
}

// TRY: add a new operation without changing Shape. Then add a variant and count
// affected functions. C# fallbacks report unhandled values at runtime, not build time.
// The exact square comparison illustrates a guard, not approximate geometry equality.
using System.Globalization;

namespace CSharp.Guided;

// A sum (Circle OR Rectangle); Rectangle carries a product (Width AND Height).
// Private construction restricts ordinary external derivation; records provide
// value equality and init-only positional properties, not compiler exhaustiveness.
public abstract record Shape
{
    private Shape() { }
    public sealed record Circle(double Radius) : Shape;
    public sealed record Rectangle(double Width, double Height) : Shape;
}

public static class ShapeOperations
{
    public static double CalculateArea(Shape shape)
    {
        // TOUR 1 / BREAKPOINT: external case analysis replaces the virtual call.
        return shape switch
        {
            Shape.Circle c => Math.PI * c.Radius * c.Radius,
            Shape.Rectangle(var w, var h) => w * h,
            _ => throw new ArgumentOutOfRangeException(nameof(shape))
        };
    }
}

internal static class ShapeTour
{
    private static int Main(string[] args)
    {
        if (args.Length > 1 || (args.Length == 1 && args[0] != "--verify"))
        {
            Console.Error.WriteLine("Usage: CSharp.Guided [--verify]");
            return 2;
        }
        Shape[] shapes = [new Shape.Circle(3), new Shape.Rectangle(4, 3)];
        double[] expected = [28.274333882308138, 12];
        string[] labels = ["Circle", "Rectangle"];
        for (int index = 0; index < shapes.Length; index++)
        {
            double area = ShapeOperations.CalculateArea(shapes[index]);
            if (!double.IsFinite(area) || Math.Abs(area - expected[index]) > 1e-10)
            {
                Console.Error.WriteLine("Verification failed: data-first area calculation.");
                return 1;
            }
            if (args.Length == 0)
            {
                Console.WriteLine($"{labels[index]} area: {area.ToString("F2", CultureInfo.InvariantCulture)}");
            }
        }
        return 0;
    }
}

// TOUR 2 / TRY: add Describe(Shape) as a separate function without editing Shape.
// Put a guarded square case before the general rectangle case. Why that order?
// Then add Triangle: every shape operation must be reviewed. See 03-Solution.
// An optional local Program.cs scratch file is excluded from build and Git.

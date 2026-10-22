namespace CSharp.Solution;

internal static class Verification
{
    public static bool Run()
    {
        bool passed = true;
        void Expect(bool condition, string label)
        {
            if (!condition)
            {
                Console.Error.WriteLine($"Verification failed: {label}.");
                passed = false;
            }
        }

        (Shape Shape, double Area)[] cases = [
            (new Shape.Circle(3), 28.274333882308138),
            (new Shape.Circle(0.5), 0.7853981633974483),
            (new Shape.Rectangle(4, 3), 12), (new Shape.Rectangle(2.5, 1.5), 3.75),
            (new Shape.Triangle(4, 3), 6), (new Shape.Triangle(2.5, 1.5), 1.875)];
        foreach (var item in cases)
        {
            double actual = ShapeOperations.CalculateArea(item.Shape);
            Expect(double.IsFinite(actual) && Math.Abs(actual - item.Area) < 1e-10, $"area of {item.Shape}");
        }

        (Shape Shape, string Description)[] descriptions = [
            (new Shape.Circle(101), "Large circle: 101"),
            (new Shape.Circle(100), "Standard circle: 100"),
            (new Shape.Circle(99.5), "Standard circle: 99.5"),
            (new Shape.Rectangle(4, 4), "Square: 4x4"),
            (new Shape.Rectangle(4, 3), "Rectangle: 4x3"),
            (new Shape.Rectangle(3, 4), "Rectangle: 3x4"),
            (new Shape.Triangle(4, 3), "Triangle: base 4, height 3")];
        foreach (var item in descriptions)
        {
            Expect(ShapeOperations.Describe(item.Shape) == item.Description, $"description of {item.Shape}");
        }
        Expect(ShapeOperations.ToSvg(new Shape.Circle(3)) == "<circle r=\"3\" />", "circle SVG");
        Expect(ShapeOperations.ToSvg(new Shape.Rectangle(4, 3)) == "<rect width=\"4\" height=\"3\" />", "rectangle SVG");
        Expect(ShapeOperations.ToSvg(new Shape.Triangle(4, 3)) == "<polygon points=\"0,0 4,0 0,3\" />", "triangle SVG");
        Expect(ShapeOperations.ToSvg(new Shape.Rectangle(2.5, 1.5)) == "<rect width=\"2.5\" height=\"1.5\" />", "fractional SVG");
        Expect(new Shape.Circle(3) == new Shape.Circle(3), "record value equality");
        return passed;
    }
}
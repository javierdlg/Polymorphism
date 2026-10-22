using System.Globalization;

namespace CSharp.Solution;

internal static class ShapeTour
{
    internal static string Example(int number) => number switch
    {
        1 => $"Circle area: {Area(new Shape.Circle(3))}\nRectangle area: {Area(new Shape.Rectangle(4, 3))}",
        2 => string.Join('\n', new Shape[] { new Shape.Circle(101), new Shape.Circle(100),
            new Shape.Rectangle(4, 4), new Shape.Rectangle(4, 3), new Shape.Triangle(4, 3) }
            .Select(ShapeOperations.Describe)),
        3 => $"{ShapeOperations.ToSvg(new Shape.Circle(3))}\n{ShapeOperations.ToSvg(new Shape.Rectangle(4, 3))}",
        4 => $"Triangle area: {Area(new Shape.Triangle(4, 3))}",
        _ => throw new ArgumentOutOfRangeException(nameof(number))
    };

    private static string Area(Shape shape) => ShapeOperations.CalculateArea(shape)
        .ToString("F2", CultureInfo.InvariantCulture);

    private static int Main(string[] args)
    {
        if (args is ["--verify"])
        {
            return Verification.Run() ? 0 : 1;
        }
        if (args is ["--all"])
        {
            for (int number = 1; number <= 4; number++)
            {
                Console.WriteLine(Example(number));
            }
            return 0;
        }
        if (args.Length == 0)
        {
            Console.WriteLine(Example(1));
            return 0;
        }
        if (args is ["--lesson", var value] && value.Length == 1 && value[0] is >= '1' and <= '4')
        {
            Console.WriteLine(Example(value[0] - '0'));
            return 0;
        }
        Console.Error.WriteLine("Usage: CSharp.Solution [--lesson 1|2|3|4 | --all | --verify]");
        return 2;
    }
}
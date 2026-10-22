namespace CSharp.Solution;

// Pure data: no area, description, or rendering behavior on the variants.
// These examples assume finite positive dimensions; validation is a separate concern.
public abstract record Shape
{
    private Shape() { }

    public sealed record Circle(double Radius) : Shape;
    public sealed record Rectangle(double Width, double Height) : Shape;
    public sealed record Triangle(double BaseLength, double Height) : Shape;
}

// Positional records supply Deconstruct and value equality. Their generated
// properties are init-only; this is not a promise of deep immutability for all records.
// C# still doesn't enforce exhaustive switches over this record hierarchy.
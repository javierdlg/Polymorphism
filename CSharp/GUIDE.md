# C#: virtual objects -> records and pure functions

Use Debug / ARM64 and set each stage as startup in turn. Budget 10-15 minutes per
concept. Fixtures have finite positive dimensions; no input-validation framework.

## 1. Problem: behavior lives on each subtype

Open `01-Problem\ShapeTour.cs`. Shape declares abstract GetArea; Circle and
Rectangle override it. The caller's static type is Shape, but the concrete runtime
type selects the implementation at `shape.GetArea()`.

Predict Circle(3) = pi times 9 and Rectangle(4,3) =12. Break at the virtual call,
F11 into the override, and inspect area after returning. Use Locals/Call Stack.

**Try:** add `public abstract string Describe();` to Shape. Build before adding
overrides and inspect the diagnostics. Then implement descriptions in both shapes.
Adding an operation touches the contract and concrete implementations. This is
not broken OOP: new subtypes can instead implement the contract without editing
callers. Open provider boundaries often suit this organization.

## 2. Guided: represent alternatives, then interpret them

Open `02-Guided\ShapeTour.cs`. A local Program.cs scratch file, if present, is
excluded from compilation and Git and is not part of the shared lesson.

Shape is now an abstract record, private constructor, nested sealed records.
Think **Circle OR Rectangle** (sum type). Rectangle has **Width AND Height**
(product payload). This emulates a discriminated union in C#, but does not give
the compiler exhaustive-switch checking over the hierarchy.

Positional records supply init-only properties, value equality and Deconstruct.
They do not automatically provide deep immutability or create a sum type alone.
The private constructor restricts ordinary outside derivation. No variant owns
the area operation: CalculateArea is an external pure function, reading data
without mutation or I/O; Main prints.

Break at its switch. Inspect the value, predict its arm, then step. In
`Shape.Rectangle(var w, var h) => w * h`, the positional pattern deconstructs the
payload. This is case analysis, not virtual GetArea dispatch.

**Try before the answer:** add static Describe(Shape) without editing Shape or
CalculateArea. Specialized arms must precede general ones:

1. Circle Radius >100 -> large circle.
2. Other Circle -> standard circle.
3. Rectangle with equal sides -> square.
4. Other Rectangle -> rectangle.
5. Explicit failure fallback.

Test 101 versus100 and 4x4 versus4x3. Keep baseline output unchanged; inspect
temporary local calls in Watch. This operation needed a new function, not edits
to every data type. Remove temporary output before transcript verification.

## 3. Solution: patterns and their technical meaning

Open `03-Solution\Shape.cs` and `ShapeOperations.cs`. No arguments run example 1,
the same two areas. Use --lesson 2 for the worked Describe answer:

```text
Large circle: 101
Standard circle: 100
Square: 4x4
Rectangle: 4x3
Triangle: base 4, height 3
```

`Shape.Circle { Radius: > 100 } c` combines type, property and relational patterns,
binding the object to c. The general Circle arm handles the rest. First match wins.
`Shape.Rectangle(var w, var h) when w == h` is deconstruction plus a guard. Exact
equality illustrates classification of dimensions, not approximate geometry.

Fallbacks throw rather than return a success-shaped default. They also reject
null. Static annotations are helpful but not runtime enforcement. C# does not
diagnose every omitted variant here: compare Rust exhaustive matching and C++
explicit visitor coverage. Coverage alone never proves a formula correct.

## 4. Add an operation versus add a variant

Run --lesson 3. ToSvg is another external function; Shape data did not change:

```text
<circle r="3" />
<rect width="4" height="3" />
```

Run --lesson 4. Triangle(4,3) has area6.00. Find its handling in area, description
and SVG (a right triangle). A new alternative required reviewing every operation.

**Try:** add a dimension-summary function without editing Shape. Then sketch a
new variant and list affected functions. Optionally remove Triangle's area arm
and run example4: the fallback fails at runtime. Restore it before verification.

| Extension | Behavior-owning hierarchy | Data-first functions |
|---|---|---|
| New shape | New subtype implements contract | Extend data and affected operations |
| New required operation | Update contract and implementations | Add function without editing data |

This is the **expression-problem trade-off**. Neither makes both extensions easy.
Known messages/states often suit closed data; open providers often suit interfaces.
This established approach is not automatically best because its syntax is newer.

## Verify and explain

Use --verify per stage. Solution checks formulas, fractional inputs, 100/101,
square/non-square cases, SVG and record equality. Shared Verify.ps1 also checks
transcripts, selectors, invalid arguments and both build configurations.

Explain static type, runtime type, sum/product, positional pattern, guard, pure
function and exhaustiveness at actual statements. Recommend designs for an open
provider API and a fixed protocol model, naming the more expensive extension.
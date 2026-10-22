# C++: virtual shapes to data-first operations

Use Debug / ARM64. These three projects use C++20, v145 MSVC, and only the standard
library. Start with the problem, attempt the guided exercise, then read the answer.

## 1. Problem: behavior lives in a hierarchy

Open `01-Problem\main.cpp`. `Shape::area() = 0` makes Shape abstract; Circle and
Rectangle override it. At STOP 1, the static type is `const Shape*`, but the concrete
object determines the virtual method called. The pointers borrow stack objects
that outlive the calls; virtual dispatch does not require allocation or ownership.

Run without arguments:

```text
Circle area: 28.27
Rectangle area: 12.00
```

At STOP 2, sketch adding `virtual std::string describe() const = 0`. You must edit
the base contract and every concrete class. Try describing circles differently
above radius 100, and distinguishing squares from other rectangles. Adding SVG
would spread another operation through the hierarchy. This change pressure, not a
runtime bug, motivates separating stable data from growing operations.

## 2. Guided: data plus one pure operation

Open `02-Guided\main.cpp`. Circle and Rectangle are aggregates containing only
their relevant dimensions. Shape is `std::variant<Circle, Rectangle>`: a value
holding one of a fixed list of alternatives, not a base class.

**STOP 1:** follow `area(const Shape&)` into `std::visit`. Each explicit
`AreaVisitor::operator()` handles one data type. There is no generic catch-all.
The formulas are still pi times radius squared and width times height. The
no-argument output stays identical to the problem stage.

**STOP 2 / exercise:** add a pure `describe(const Shape&)` returning `std::string`
and an explicit DescribeVisitor. Do not change the aggregates, variant, or area.
Use radius `> 100` for a large circle (100 itself is standard), and exact equality
of width and height for a square. Test 101, 100, 4x4, and 4x3. Expect the first four
lines of lesson 2 below. Keep the baseline main output unchanged; add checks to
verification for your exercise. Compare with the worked answer only afterward.

## 3. Worked answer: new operations versus new alternatives

Open `03-Solution\shapes.h` and `operations.h`.

1. `--lesson 1`: the same area calculation, now a free function over data.
2. `--lesson 2`: DescribeVisitor completes the exercise. Its conditional guards
   correspond to guarded cases in a C# switch expression over nested records.
3. `--lesson 3`: ToSvgVisitor adds another pure operation without changing data.
4. `--lesson 4`: Triangle adds a new alternative. This time the variant and every
   visitor must change: area, describe, and ToSvg. Its SVG uses a right triangle
   with vertices (0,0), (base,0), and (0,height).

Lesson 2:

```text
Large circle: 101
Standard circle: 100
Square: 4x4
Rectangle: 4x3
Triangle: base 4, height 3
```

Lesson 3:

```text
<circle r="3" />
<rect width="4" height="3" />
```

Lesson 4:

```text
Triangle area: 6.00
```

**Try:** temporarily remove the Triangle overload from one visitor and build.
`std::visit` requires an invocable visitor for every alternative, so this fails
at compile time. Restore the overload. Explicit overloads keep missing cases
visible; a generic catch-all could silently hide the work needed for new data.
This checks completeness, not whether formulas or guards are correct.

Unlike these C# records' init-only positional properties, C++ aggregates have writable fields. Treat
fixtures as immutable by convention: use `const Shape` values and pass
`const Shape&`. Operations neither mutate their input nor perform I/O; main owns
printing. `const` access is not a universal deep-immutability guarantee.

All fixtures require finite, positive dimensions with representable calculated
areas. The CLI does not accept dimensions; it only selects examples. There is no
input-validation framework. Dimension formatting is concise and locale invariant;
area formatting is locale invariant with exactly two fractional digits. Square
classification compares the supplied dimensions exactly, while computed-area
verification uses combined relative/absolute tolerance.

This is a trade-off, not a universal modern replacement for virtual methods.
Virtual interfaces fit independently extensible implementations. A fixed variant
makes adding operations easy, but adding an alternative requires reviewing all
visitors. Neither design removes behavioral contracts or guarantees performance.

## Run and verify

Problem and Guided support no arguments and `--verify`. Solution supports:

```text
Cpp.Solution
Cpp.Solution --lesson 1
Cpp.Solution --lesson 2
Cpp.Solution --lesson 3
Cpp.Solution --lesson 4
Cpp.Solution --all
Cpp.Solution --verify
```

No arguments selects lesson 1. `--all` concatenates lessons 1 through 4 without
headings or blank separators. `--verify` is silent on success (exit 0); failures
are reported on stderr (exit 1). It checks numeric areas, fractional fixtures,
guard boundaries, square/non-square cases, triangle behavior, SVG, formatting,
and exact lesson transcripts. Problem and Guided verify baseline and fractional
areas. Invalid arguments print `Usage:` on stderr and return 2.

Build each affected `.vcxproj` separately with MSBuild, configuration Debug and
platform ARM64; do not build the whole repository. Run each built executable's
`--verify`, then compare default/lesson outputs above.

**Self-check:** Which files change when adding describe? When adding Triangle?
Why does const usage differ from init-only record properties? When would an open virtual
interface be a better fit than a fixed variant?

# Polymorphism: objects versus data and operations

## Purpose

A compact college-level comparison of inheritance-based OOP and data-first design
in C#, C++, and Rust. Teach how organizing code changes extension costs, not a
universal replacement for OOP. Use Circle(radius), Rectangle(width,height), and
Triangle(base,height). Fixtures assume finite positive dimensions and representable
areas. The CLI selects examples, not arbitrary geometry inputs.

## Three languages, three independent stages each

| Stage | What runs | Exercise |
|---|---|---|
| 01-Problem | Shapes own area behavior through a virtual contract | Add a required description operation; inspect contract/implementation changes |
| 02-Guided | Two data alternatives plus an external area function | Add description without changing data |
| 03-Solution | Three alternatives; external area, description, SVG | Compare the answer; add an operation versus a variant |

All nine projects remain in the existing solution. C#/C++ begin with abstract
classes and overrides; Rust has no inheritance and uses borrowed trait objects.
The initial design works: distributed edits for new operations are the pressure.
Stages share build settings, not teaching implementations. Solution as a stage
means worked answer; .slnx is the Visual Studio solution container.

ShapeTour.cs is the C# entry point. A local Guided Program.cs scratch file, if
present, is excluded from compilation and Git; it is not part of the shared lab.

## Concepts and professional meaning

1. Runtime dispatch: static versus concrete runtime type, override versus overload.
2. Algebraic modeling: Circle OR Rectangle is a sum; Width AND Height is a product.
3. Pure operations read values without mutation or I/O; Main prints results.
4. Patterns: deconstruction, radius >100, square before general rectangle; first
   matching arm wins. Exact equality illustrates classification, not approximate geometry.
5. Extension trade-off: add ToSvg without changing data; add Triangle and review
   every operation. SVG uses a right triangle with (0,0),(base,0),(0,height).

| Language | Data | Operations |
|---|---|---|
| C# | Private base record constructor, nested sealed positional records | Switch expressions, property/relational/positional patterns, when guards |
| C++ | Aggregates in std::variant | std::visit with explicit overloads |
| Rust | Enum variants with payloads | Exhaustive match over borrowed data, guards |

C# restricts ordinary external derivation but does not enforce exhaustive switches
over this record hierarchy. Fallbacks fail explicitly at runtime. Records provide
value equality, init-only positional properties and Deconstruct, not universal
deep immutability. C++ aggregates remain mutable; const provides read-only usage.
Rust borrows input. Guards do not prove exhaustiveness; unguarded arms cover
remaining values. Explicit C++ visitors expose omitted alternatives. Coverage
does not prove formulas/guards correct. Dispatch tables do not imply a guaranteed
portable layout or a measured performance cost.

| Extension | Behavior-owning hierarchy | Data + external functions |
|---|---|---|
| New shape | New subtype implements existing operations | Extend data and affected operations |
| New required operation | Usually edit contract and implementations | Add function without editing data |

Teach this expression-problem trade-off. Open provider boundaries often suit
interfaces/traits; known message/state alternatives often suit closed data.
Neither design makes both extensions effortless.

## Guidance and execution

Budget 10-15 minutes per concept per language. Keep data/operations compact;
CLI/verification can be longer. No packages, frameworks, unsafe code, or template
tricks. Each stage has two tour stops, definition, prediction, executable breakpoint,
small edit, expected observation, professional use and limitation. Intentional
failure experiments are opt-in; default projects build. Guides lead to a separate
worked answer rather than silently applying exercise solutions.

Use F9/F5/F10/F11/Shift+F11, Locals, Watch, Call Stack and navigation purposefully.
Predict the virtual target, then inspect data/payload and selected external arm.
Add a variant and compare compiler Output/Error List coverage across languages.

Problem/Guided: no arguments or --verify. Solution: no arguments (example 1),
--lesson 1 through 4, --all, --verify. Silent verification success exits 0; failure
reports stderr and exits 1; invalid arguments print Usage: and exit 2. No keypress.
--all concatenates examples with no headings or blank separators.

Example 1 and earlier stages:

```text
Circle area: 28.27
Rectangle area: 12.00
```

Example 2:

```text
Large circle: 101
Standard circle: 100
Square: 4x4
Rectangle: 4x3
Triangle: base 4, height 3
```

Example 3:

```text
<circle r="3" />
<rect width="4" height="3" />
```

Example 4: Triangle area: 6.00. Areas use two fractional digits, dimensions concise
locale-invariant formatting. Numeric checks use tolerance; transcripts are exact.

## Build and acceptance

C#: .NET 10, global.json stable SDK, nullable/warnings-as-errors. C++: C++20,
VS 2026 v145 and Windows SDK, high warnings, no CMake. Rust: stable edition 2024,
dependency-free Cargo workspace, NMake adapters targeting executables and PDBs.
Native ARM64 is verified. x64 mappings require that target's Rust library/tools
and are unverified. Run Cargo here, not under another repo's toolchain override.
LSP, compiler, and native debugger are separate layers.

Verify.ps1 (PowerShell 7) builds all nine in Debug/Release, checks exact output,
selectors, verification, repeated runs and invalid arguments, then Rust tests,
rustfmt and Clippy. It installs nothing or changes source. Check all formulas,
fractional values, guard boundary 100/101, square/non-square, Triangle, and SVG.
A process start or presence-only check is insufficient.

Guides must match actual paths. Verify source breakpoints separately: previous
robot debugger/navigation evidence is historical, not proof for shapes. Do not
infer optimized cost or editor/debug success from CLI tests. Preserve user scratch.
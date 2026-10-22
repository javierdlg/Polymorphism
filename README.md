# Polymorphism: behavior-owning objects versus structured data

The main comparison is now your Shape example: virtual area methods versus
data-only alternatives and external area, description and SVG functions.

## Start

Open Polymorphism.slnx in Visual Studio. Use **Debug / ARM64** on this machine.
Read [the C# guide](CSharp/GUIDE.md), set CSharp.Problem as startup, and follow
Problem -> Guided -> Solution. Repeat in [C++](Cpp/GUIDE.md) and [Rust](Rust/GUIDE.md).
There are still nine independently buildable projects, three in each language.

| Stage | What you learn |
|---|---|
| 01-Problem | Virtual shapes work; adding a required operation changes every implementation |
| 02-Guided | Shape data plus an external area function; add Describe without changing data |
| 03-Solution | Worked answer: Triangle, descriptions with guards, another SVG operation |

Open ShapeTour.cs for the C# tours. A local Guided Program.cs scratch file, if
present, is excluded from compilation and Git; it is not part of the shared lab.

## Worked-solution examples

| Arguments | Focus |
|---|---|
| none or --lesson 1 | Area: Circle(3), Rectangle(4,3) |
| --lesson 2 | Radius 101 versus 100, square/non-square, Triangle descriptions |
| --lesson 3 | Add SVG without editing data |
| --lesson 4 | Add Triangle and update affected operations |
| --all | All examples, no headings |
| --verify | Silent checks; success 0, failure diagnostic and exit 1 |

Problem/Guided accept no arguments or --verify. Invalid arguments print usage to
stderr and exit 2. No keypress. Set C# arguments through Debug launch settings;
C++/Rust through Properties > Debugging > Command Arguments.

Area baseline in all languages/stages:

```text
Circle area: 28.27
Rectangle area: 12.00
```

Fixtures assume finite positive dimensions; this is not a public geometry library.
See [the spec](SPEC.md) for full transcripts and acceptance criteria.

## Debug to answer a question

Use **Toggle Breakpoint (F9)** at an executable tour statement. **Start Debugging
(F5)**; predict before **Step Into (F11)**. Inspect Locals, Watch and Call Stack.
**Step Over (F10)** and **Step Out (Shift+F11)** avoid library internals; break
directly in tiny methods when needed. Keyboard mappings can differ.

At the virtual call distinguish static and concrete runtime type. At the external
operation inspect the alternative, payload, and selected switch/match/visitor.
Use Go To Definition and Go To Implementation where supported. When adding a
variant, inspect Error List/Build Output. C# record hierarchy switches are not
compiler-exhaustive; explicit fallbacks remain runtime safeguards. Rust match and
C++ visitor coverage do not prove correct formulas or guards.

## Tools and verification

Stable .NET 10 (global.json), VS 2026 .NET desktop and C++ v145/Windows SDK, and
stable Rust MSVC are already available. Native ARM64 is verified; x64 requires
its Rust target and matching native tools. Cargo adapters remain in the solution.
LSP support is separate from compilation/debugging; diagnose before installing.

From PowerShell 7 in this directory, run `.\Verify.ps1`. It builds all nine in
Debug/Release and checks transcripts, numeric formulas, guards, SVG, argument
handling, Rust tests, rustfmt, and Clippy without installing tools or changing source.

Fresh validation on October 19, 2026 passed all 18 native ARM64 stage/configuration
combinations, all eight Rust tests, formatting and Clippy. x64 is configured but
unverified.

Fresh Visual Studio source breakpoints also verified the shape examples:

| Target | Observed execution | Inspected results |
|---|---|---|
| CSharp.Problem | Concrete virtual Circle/Rectangle GetArea bodies | 28.274333882308138 and 12 |
| CSharp.Guided | External CalculateArea over record alternatives | 28.274333882308138 and 12 |
| Cpp.Guided | External area and explicit visitor overloads over variant | 28.274333882308138 and 12 |
| Rust.Guided | External calculate_area over enum alternatives | 28.274333882308138 and 12 |

These checks used implementation and post-call breakpoints, not automated F11
single-stepping. Rust match bindings required native-pointer inspection. New Rust
editor-navigation behavior was not tested. The IDE was left in design mode with
CSharp.Problem startup and Debug/ARM64 selected; user breakpoints and unsaved
documents were preserved.

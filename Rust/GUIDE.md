# Rust guided route: behavior contracts to data-first shapes

## 1. Problem: operations belong to implementations

Open `01-Problem\src\main.rs`. Rust has no class inheritance: `Shape` is a
trait implemented by Circle and Rectangle. At TOUR 1, follow the borrowed
`&dyn Shape` calls for radius 3 and dimensions 4 by 3: areas are `PI * 9` and
12. Dynamic dispatch borrows local values without heap allocation.

At TOUR 2, add a required `fn describe(&self) -> String;` to the trait without
a default. Both implementations must change. This is a maintenance trade-off,
not a claim that traits are unsuitable for open implementation boundaries.

## 2. Guided: separate data from operations

Open `02-Guided\src\main.rs`.

1. **TOUR 1 — data:** Shape is a sum type: Circle or Rectangle. Each variant's
   named fields form a product-type payload. This corresponds to the closed
   alternatives of the C# record example, not record inheritance in Rust.
2. **TOUR 2 — operation:** `calculate_area(&Shape)` is an external pure function.
   Its exhaustive match handles both variants and borrows the data unchanged.

**Exercise:** add `describe(&Shape) -> String` without modifying Shape or
calculate_area. Return `Standard circle: 3` for radius 3 and `Rectangle: 4x3`
for the rectangle. Handle both variants explicitly and add description tests.
Only then read the solution.

## 3. Solution: independent operations over a closed sum type

Open `03-Solution\src\main.rs`. The data-only enum adds
`Triangle { base_length, height }`. External pure functions `calculate_area`,
`describe`, and `to_svg` each perform case analysis.

| Example | Focus | Result |
| --- | --- | --- |
| 1 (default) | Area operation | Circle 28.27; rectangle 12.00 |
| 2 | Match guards | Radius 101 is large; 100 is standard; equal sides form a square |
| 3 | New operation, unchanged data | Circle and rectangle SVG |
| 4 | New variant | Triangle area 6.00 |

Guards refine cases (`radius > 100`, `width == height`); they do **not** prove
exhaustiveness. Following unguarded Circle and Rectangle arms handle remaining
values, and Triangle has its own arm. No Shape match uses a wildcard.

Adding an operation leaves the enum unchanged. Adding a variant requires
updating every exhaustive operation: try a new variant and observe compiler
diagnostics before completing each match. Compare the trait stage, where a
new implementation leaves callers alone but a required operation changes
every implementation.

Fixtures assume positive finite dimensions, not a validated public geometry
API. Square detection uses exact equality of fixture dimensions; area checks
use numeric tolerance. No validation framework is needed for this lesson.

## Run and verify

Run from this Rust directory to avoid another repository's toolchain override.
Explicit stable selection matches the existing NMake adapters:

```powershell
Set-Location .\Rust
cargo +stable fmt --all -- --check
cargo +stable test --workspace --target aarch64-pc-windows-msvc
cargo +stable clippy --workspace --all-targets --target aarch64-pc-windows-msvc -- -D warnings
cargo +stable run -p rust-problem --target aarch64-pc-windows-msvc -- --verify
cargo +stable run -p rust-guided --target aarch64-pc-windows-msvc -- --verify
cargo +stable run -p rust-solution --target aarch64-pc-windows-msvc -- --all
cargo +stable run -p rust-solution --target aarch64-pc-windows-msvc -- --verify
```

Problem and Guided accept no arguments or `--verify`. Solution accepts no
arguments (example 1), `--lesson 1` through `--lesson 4`, `--all`, or `--verify`.
Successful executable verification is silent and returns 0; Cargo may print
its own messages. Invalid arguments print `Usage:` to stderr and return 2.
`--all` concatenates examples 1 through 4 without headings.

## Tooling facts and evidence limits

The Cargo workspace and Visual Studio NMake adapters remain unchanged.
`Build.props` maps ARM64 to `aarch64-pc-windows-msvc`, invokes `cargo +stable`,
and launches `target\<target>\<profile>` executables. Existing project entries
include the stage sources. Use Debug / ARM64 for this machine's native route.

Fresh Visual Studio source-breakpoint checks on October 19, 2026 reached the
Guided calculate_area match arms for Circle radius 3 and Rectangle dimensions
4 by 3. Post-call inspection in verify showed 28.274333882308138 and 12.
Match bindings required native-pointer inspection. These checks used source and
post-call breakpoints, not automated single-stepping. New editor navigation was
not tested; earlier robot navigation evidence is historical. Cargo tests and a
rust-analyzer installation alone prove neither debugger binding nor navigation.

## Self-check

Explain trait implementation versus inheritance, borrowed dynamic dispatch
versus enum case analysis, sum versus product types, and guards versus
exhaustiveness. Identify exactly what changes for a new operation or variant.

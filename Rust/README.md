# Data-first shapes in Rust

Three runnable stages compare a behavior-centered trait contract with a
data-only enum and external pure functions. Rust has no class inheritance.

| Stage | Model | Learning goal |
| --- | --- | --- |
| `01-Problem` | Circle/Rectangle implement `Shape::area` | Adding a required trait operation changes implementations |
| `02-Guided` | Shape enum plus `calculate_area` | Add describe without changing the data model |
| `03-Solution` | Circle/Rectangle/Triangle and three operations | Exhaustive case analysis, guards, and extension trade-offs |

An enum is a **sum type**; each variant's fields form a **product type**.
Match guards refine cases but do not establish exhaustiveness. Unguarded
variant arms complete coverage; no Shape match uses a wildcard.

Fixtures assume positive finite dimensions. This is not an input-validation
framework. See [GUIDE.md](GUIDE.md) for the two-stop guided exercise, expected
results, and tooling evidence limits.

From this directory:

```powershell
cargo +stable run -p rust-solution --target aarch64-pc-windows-msvc
cargo +stable run -p rust-solution --target aarch64-pc-windows-msvc -- --lesson 2
cargo +stable run -p rust-solution --target aarch64-pc-windows-msvc -- --all
cargo +stable run -p rust-solution --target aarch64-pc-windows-msvc -- --verify
cargo +stable fmt --all -- --check
cargo +stable test --workspace --target aarch64-pc-windows-msvc
cargo +stable clippy --workspace --all-targets --target aarch64-pc-windows-msvc -- -D warnings
```

Default output (all three stages):

```text
Circle area: 28.27
Rectangle area: 12.00
```

Solution supports `--lesson 1|2|3|4`, `--all`, and `--verify`; Problem and
Guided support `--verify`. Successful executable verification prints nothing
and returns 0. Invalid arguments print `Usage:` to stderr and return 2.
Cargo itself may print build/run messages. Existing workspace and solution
adapter build paths are unchanged.

# Non-functional Requirements

| ID       | Requirement                   | Description                                                                                                                      |
| -------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| **NF1**  | **Performance**               | All arithmetic and conversions must run in constant time **O(1)**. Compile-time currency types eliminate runtime checks.         |
| **NF2**  | **Memory safety**             | The code must be **100% safe Rust**. Use `#![forbid(unsafe_code)]` to guarantee no `unsafe` blocks.                              |
| **NF3**  | **Precision and determinism** | All operations must yield deterministic results across all platforms. Floating-point types (`f32`, `f64`) are prohibited.        |
| **NF4**  | **Portability**               | The crate must compile and operate under `no_std` with `alloc`. Targets include Linux, macOS, Windows, and WebAssembly.          |
| **NF5**  | **Extensibility**             | Adding a new currency or unit must require no modification of the library — only a new `impl Currency` definition.               |
| **NF6**  | **Documentation quality**     | 100% of public APIs must be documented with Rust doc comments. Include high-quality examples and crate-level documentation.      |
| **NF7**  | **Testing coverage**          | Achieve at least 90% code coverage with unit and property-based tests. Include serialization and arithmetic tests.               |
| **NF8**  | **Continuous integration**    | CI must enforce `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo audit`. All warnings must fail builds.                     |
| **NF9**  | **Security**                  | No network dependencies for exchange rates. All parsing must reject ambiguous formats and check for overflow.                    |
| **NF10** | **Usability**                 | API naming must be intuitive (`from_major`, `from_minor`, `convert`, `to_string`). Compiler errors must be explicit and helpful. |
| **NF11** | **Licensing**                 | Dual-licensed under MIT OR Apache-2.0. Dependencies must use compatible, permissive licenses.                                    |
| **NF12** | **Semantic versioning**       | The crate must follow SemVer. Backward compatibility must be preserved across minor releases.                                    |
| **NF13** | **Build validation**          | The crate must compile successfully with and without default features enabled.                                                   |
| **NF14** | **Error transparency**        | All errors must implement `std::error::Error` and provide human-readable messages.                                               |
| **NF15** | **Community readiness**       | The project must include a `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, and issue templates for contributors.                 |

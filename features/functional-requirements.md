# Functional Requirements

| ID      | Requirement                           | Description                                                                                                                                                |
| ------- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **F1**  | **Type-safe monetary representation** | The system must represent monetary values using strongly typed structs (`Amount<Currency>`), ensuring currency safety at compile time.                     |
| **F2**  | **Currency abstraction**              | Users must be able to define their own currency types via a `Currency` trait. The library should provide built-in common currencies (USD, EUR, BTC, etc.). |
| **F3**  | **Explicit conversions**              | Currency conversion must only occur through explicit `Rate<From,To>` objects. Implicit or automatic conversions are disallowed.                            |
| **F4**  | **Arithmetic operations**             | Support arithmetic between amounts of the same currency (`+`, `-`, `*`, `/`), and comparisons (`>`, `<`, `==`). Disallow cross-currency operations.        |
| **F5**  | **Decimal precision**                 | Internal representation must use deterministic decimal arithmetic (e.g., `rust_decimal`). Precision and rounding follow `Currency::DECIMALS`.              |
| **F6**  | **Rounding and scaling**              | Provide configurable rounding modes: half-up, down, up, banker's rounding, etc. Rounding must be explicit and predictable.                                 |
| **F7**  | **Serialization and formatting**      | Support optional `serde` (de)serialization and formatted display strings like `"€12.34 EUR"`. Implement `Display`, `Debug`, and `FromStr`.                 |
| **F8**  | **Exchange rate safety**              | Conversion rates must be immutable and explicitly defined. Each conversion must reference a `Rate` instance for auditability.                              |
| **F9**  | **Error handling**                    | Define errors such as `CurrencyMismatch`, `ConversionRateMissing`, and `PrecisionError`. Use `Result<T, MoneyError>` for all fallible operations.          |
| **F10** | **Custom units (optional)**           | Allow extension to other units (e.g., `Amount<KILOGRAM>`) using the same type-safety model.                                                                |
| **F11** | **Feature flags**                     | Provide feature flags: `serde`, `rust_decimal`, `bigdecimal`, `std`, and `no_std` for flexible builds.                                                     |
| **F12** | **Doctests and examples**             | All public APIs must include runnable doctests. The crate must include example files demonstrating usage (e.g., conversions, serialization).               |

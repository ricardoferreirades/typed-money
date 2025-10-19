# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of typed-money library
- Type-safe monetary representation with compile-time currency safety
- Support for 6 built-in currencies (USD, EUR, GBP, JPY, BTC, ETH)
- O(1) arithmetic operations (Add, Sub, Mul, Div)
- O(1) comparison operations (Eq, PartialEq, Ord, PartialOrd)
- Explicit currency conversion system with Rate<From, To> types
- 7 rounding modes (HalfUp, HalfDown, HalfEven, Up, Down, Ceiling, Floor)
- Precision control and validation
- Comprehensive error handling with MoneyError enum
- Display formatting with locale support (en_US, de_DE, fr_FR)
- String parsing with security validation
- Serde serialization/deserialization support
- Feature flags for flexible builds (std/no_std, rust_decimal/bigdecimal)
- Conversion tracking for auditing (optional feature)
- 210+ unit tests with >90% code coverage
- 67 doctests with comprehensive examples
- 6 example files demonstrating all features
- CI/CD pipeline with GitHub Actions
- Complete API documentation

### Technical Details
- Built with Rust 1.70+
- Uses rust_decimal for deterministic arithmetic (default)
- Optional bigdecimal support via feature flag
- No-std support for embedded systems
- Zero unsafe code (`#![forbid(unsafe_code)]`)
- Dual MIT/Apache-2.0 licensing

## [0.1.0] - 2025-10-19

### Added
- Initial release
- Core type system with Amount<C: Currency>
- Currency trait with DECIMALS, CODE, SYMBOL constants
- Built-in currency implementations
- Arithmetic and comparison operations
- Currency conversion with exchange rates
- Rounding and precision control
- Error handling and validation
- Display formatting and string parsing
- Serialization support
- Comprehensive testing suite
- Documentation and examples

---

## Version History

- **0.1.0** - 2025-10-19 - Initial release with core functionality
- **Unreleased** - Future features and improvements

## Migration Guide

### From 0.0.x (if applicable)
This is the initial release, so no migration is needed.

### Breaking Changes
None in this initial release.

### Deprecations
None in this initial release.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

This project is dual-licensed under MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) for details.

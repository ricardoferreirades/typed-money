# Contributing to Typed Money

Thank you for your interest in contributing to Typed Money! This document provides guidelines and information for contributors.

## Code of Conduct

This project and everyone participating in it is governed by our commitment to creating a welcoming and inclusive environment. By participating, you agree to uphold these values.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- [Make](https://www.gnu.org/software/make/) (for development commands)
- Git

### Setup

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/your-username/typed-money.git
   cd typed-money
   ```

2. **Complete setup:**
   ```bash
   make setup
   ```
   This will install development dependencies and configure git hooks.

3. **Verify everything works:**
   ```bash
   make quality
   ```

## Development Workflow

### Available Commands

```bash
# Development
make setup        # Complete setup (install deps + git hooks)
make install-deps # Install development dependencies only
make run          # Run the application
make test         # Run all tests (437 unit + doctests)
make bench        # Run performance benchmarks
make bench-open   # Open benchmark HTML reports
make doc          # Build documentation
make doc-open     # Build and open docs in browser

# Quality
make quality      # Run all quality checks
make fmt          # Format code
make lint         # Run clippy linter
make check        # Type check
make spell        # Check spelling

# Examples
make run          # Run examples
```

### Running Examples

```bash
# Basic usage
cargo run --example basic_usage

# Currency conversions
cargo run --example conversions

# Rounding modes
cargo run --example rounding

# Error handling
cargo run --example error_handling

# Serialization
cargo run --example serialization

# Custom currencies
cargo run --example custom_currency

# Internationalization features
cargo run --example internationalization

# Currency metadata
cargo run --example currency_metadata

# Precious metals
cargo run --example precious_metals

# Global currencies
cargo run --example global_currencies
```

### Branch Strategy

- **main**: Production-ready code
- **feature/**: New features
- **fix/**: Bug fixes
- **docs/**: Documentation updates

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `build`: Build system changes
- `ci`: CI/CD changes
- `perf`: Performance improvements
- `chore`: Maintenance tasks

**Examples:**
```
feat(currency): add support for Canadian Dollar
fix(conversion): handle edge case in rate calculation
docs(api): update Amount documentation with examples
test(rounding): add tests for negative number rounding
```

## Testing

### Running Tests

```bash
# Run all tests
make test

# Run specific test categories
cargo test --lib                    # Unit tests only
cargo test --doc                    # Doc tests only
cargo test --examples               # Example tests only
cargo test --features serde_support # With specific features
```

### Test Requirements

- **Unit Tests**: All public APIs must have unit tests
- **Doc Tests**: All public APIs must have working examples in documentation
- **Integration Tests**: Cross-module functionality must be tested
- **Edge Cases**: Test boundary conditions and error cases
- **Coverage**: Maintain >90% code coverage

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{USD, EUR};

    #[test]
    fn test_amount_creation() {
        let amount = Amount::<USD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_currency_conversion() {
        let usd_amount = Amount::<USD>::from_major(100);
        let rate = Rate::<USD, EUR>::new(Decimal::new(85, 2)).unwrap();
        let eur_amount = usd_amount.convert(&rate);
        assert_eq!(eur_amount.to_major_floor(), 85);
    }
}
```

## Code Style

### Rust Standards

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Prefer explicit over implicit
- Use meaningful variable and function names

### Code Organization

```
src/
├── lib.rs                 # Library root
├── amount/               # Amount implementation
│   ├── mod.rs
│   ├── type_def.rs      # Core type definition
│   ├── constructors.rs  # Creation methods
│   ├── arithmetic.rs    # Math operations
│   ├── conversions.rs   # Unit conversions
│   ├── display.rs       # Formatting
│   ├── parsing.rs       # String parsing
│   ├── rounding.rs      # Rounding logic
│   ├── precision.rs     # Precision control
│   └── serialization.rs # Serde support
├── currency/            # Currency implementations
│   ├── mod.rs
│   ├── trait_def.rs    # Currency trait
│   └── *.rs            # Individual currencies
├── rate.rs             # Exchange rates
├── error.rs            # Error types
└── rounding.rs         # Rounding modes
```

### Documentation

- **All public APIs** must have rustdoc comments
- **Examples** should be in doc comments
- **Module-level** documentation for each module
- **README** updates for user-facing changes

```rust
/// Creates a new amount from major units (e.g., dollars).
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let amount = Amount::<USD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// ```
pub fn from_major(amount: i64) -> Self {
    // implementation
}
```

## Feature Development

### Adding New Currencies

1. **Create currency file** in `src/currency/`
2. **Implement Currency trait**
3. **Add to currency module**
4. **Add tests**
5. **Update documentation**

```rust
// src/currency/cad.rs
use crate::Currency;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CAD;

impl Currency for CAD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CAD";
    const SYMBOL: &'static str = "C$";
}
```

### Adding New Features

1. **Update implementation plan** if needed
2. **Implement feature** with tests
3. **Update documentation**
4. **Add examples**
5. **Update changelog**

## Bug Reports

### Before Reporting

1. **Check existing issues** for duplicates
2. **Test with latest version**
3. **Verify with minimal reproduction**

### Bug Report Template

```markdown
## Bug Description
Brief description of the bug.

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: [e.g., macOS 14.0]
- Rust version: [e.g., 1.75.0]
- typed-money version: [e.g., 0.1.0]

## Additional Context
Any other relevant information.
```

## Feature Requests

### Before Requesting

1. **Check existing issues** for similar requests
2. **Consider if it fits the project scope**
3. **Think about implementation complexity**

### Feature Request Template

```markdown
## Feature Description
Brief description of the feature.

## Use Case
Why is this feature needed?

## Proposed Solution
How should it work?

## Alternatives Considered
What other approaches were considered?

## Additional Context
Any other relevant information.
```

## Pull Request Process

### Before Submitting

1. **Run quality checks:**
   ```bash
   make quality
   ```

2. **Ensure tests pass:**
   ```bash
   make test
   ```

3. **Update documentation** if needed

4. **Update changelog** for user-facing changes

### PR Template

```markdown
## Description
Brief description of changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] New tests added for new functionality
- [ ] Documentation updated

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Changelog updated (if applicable)
```

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Testing** on multiple platforms
4. **Documentation** review
5. **Approval** and merge

## Architecture Guidelines

### Core Principles

1. **Type Safety**: Compile-time guarantees over runtime checks
2. **Performance**: O(1) operations where possible
3. **Determinism**: Consistent results across platforms
4. **Explicitness**: Clear, unambiguous APIs
5. **Extensibility**: Easy to add new currencies/features

### Design Patterns

- **Zero-cost abstractions**: Type safety without runtime overhead
- **Explicit conversions**: No implicit currency mixing
- **Immutable by default**: Prevent accidental mutations
- **Error handling**: Comprehensive error types with context

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Keep a Changelog](https://keepachangelog.com/)

## 🤝 Getting Help

- **Issues**: Use GitHub issues for bugs and feature requests
- **Discussions**: Use GitHub discussions for questions
- **Documentation**: Check the [API docs](https://docs.rs/typed-money)

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to Typed Money!

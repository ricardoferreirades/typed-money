# Rust Project Setup Guide - Quality & Standards Configuration

This guide provides step-by-step instructions to set up a Rust project with professional quality standards, automated checks, and development workflows.

## 🎯 Overview

This setup ensures:
- ✅ **Zero warnings policy** - All code must compile without warnings
- ✅ **Zero errors policy** - Comprehensive error handling
- ✅ **Automated quality checks** - Pre-push hooks prevent bad code
- ✅ **Conventional commits** - Standardized commit messages
- ✅ **Code formatting** - Consistent style across the project
- ✅ **Spell checking** - Catch typos in code and docs
- ✅ **Modular architecture** - Clean, maintainable code structure

---

## 📋 Prerequisites

Install the following tools:

```bash
# Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Taplo (TOML formatter)
cargo install taplo-cli

# Typos (spell checker)
cargo install typos-cli

# GitHub CLI (for git operations)
brew install gh  # macOS
# or: sudo apt install gh  # Linux
# or: winget install --id GitHub.cli  # Windows
```

---

## 🔧 Step 1: Create Configuration Files

### 1.1 Clippy Configuration (`clippy.toml`)

Create `clippy.toml` in project root:

```toml
# Clippy configuration
# Similar to .eslintrc for JavaScript

# Allow some lints that might be too strict for this project
# Most configuration is done via attributes in the code

# Allow single component path imports (like `use seahash;`)
# This is useful for re-exports and macro dependencies
# Note: This is configured via #[allow(clippy::single_component_path_imports)] in code

# Performance and style lints are generally good to keep enabled
# Disable specific ones in code if needed using #[allow(clippy::lint_name)]
```

### 1.2 Rustfmt Configuration (`rustfmt.toml`)

Create `rustfmt.toml` in project root:

```toml
# Rustfmt configuration
# Similar to .prettierrc for JavaScript

# Maximum line length
max_width = 100

# Indentation
tab_spaces = 4

# Function formatting
fn_params_layout = "Tall"

# Note: Many advanced features require nightly Rust
# For stable Rust, we use the default formatting with these basic settings
```

### 1.3 Typos Configuration (`_typos.toml`)

Create `_typos.toml` in project root:

```toml
[default]
extend-ignore-re = [
  # Technical terms and identifiers
  "([A-Z][a-z]*){2,}", # CamelCase
  "[a-z]+_[a-z]+",     # snake_case variables
]

[default.extend-words]
# Add your project-specific terms here
utoipa = "utoipa"
axum = "axum"
sqlx = "sqlx"
chrono = "chrono"
tokio = "tokio"
rustls = "rustls"

# Technical terms
anonymization = "anonymization"
anonymize = "anonymize"
anonymized = "anonymized"

[files]
extend-exclude = ["target/", "Cargo.lock", "*.log", ".git/"]
```

---

## 🛠️ Step 2: Create Makefile

Create `Makefile` in project root:

```makefile
.PHONY: fmt lint lint-fix check spell spell-fix quality setup-hooks test run

# Run the application
run:
	cargo run

# Run tests
test:
	cargo test

# Format code (like Prettier)
fmt:
	cargo fmt
	taplo format

# Lint code (like ESLint)
lint:
	cargo clippy

# Fix linting issues automatically
lint-fix:
	cargo clippy --fix --allow-dirty --allow-staged

# Check code without building (faster)
check:
	cargo check

# Check spelling
spell:
	typos

# Fix spelling issues automatically
spell-fix:
	typos --write-changes

# Setup git hooks
setup-hooks:
	./.githooks/setup.sh

# Run all code quality checks
quality: fmt lint check spell

# Build release version
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean
```

---

## 🪝 Step 3: Setup Git Hooks

### 3.1 Create `.githooks` Directory

```bash
mkdir -p .githooks
```

### 3.2 Create Pre-Push Hook (`.githooks/pre-push`)

Create `.githooks/pre-push` with execute permissions:

```bash
#!/bin/sh
# Git pre-push hook
# Runs code quality checks before allowing push

# Initialize status tracking
OVERALL_STATUS=0
FORMAT_STATUS="⏳"
LINT_STATUS="⏳"
SPELL_STATUS="⏳"
TYPE_STATUS="⏳"
TEST_STATUS="⏳"

echo "🔍 Running pre-push checks..."
echo ""

# 1. Format check
echo "📝 Checking code formatting..."
FORMAT_OUTPUT=$(mktemp)
cargo fmt --check > "$FORMAT_OUTPUT" 2>&1
FMT_EXIT=$?
taplo format --check >> "$FORMAT_OUTPUT" 2>&1
TAPLO_EXIT=$?
if [ $FMT_EXIT -eq 0 ] && [ $TAPLO_EXIT -eq 0 ]; then
    FORMAT_STATUS="✅ PASS"
    echo "✅ Formatting check passed"
else
    FORMAT_STATUS="❌ FAIL"
    echo "❌ Code formatting failed. Run 'make fmt' to fix."
    OVERALL_STATUS=1
fi
rm -f "$FORMAT_OUTPUT"
echo ""

# 2. Linting
echo "🔎 Running linter..."
CLIPPY_OUTPUT=$(mktemp)
cargo clippy --all-targets --all-features -- -D warnings > "$CLIPPY_OUTPUT" 2>&1
CLIPPY_EXIT=$?
if [ $CLIPPY_EXIT -eq 0 ]; then
    LINT_STATUS="✅ PASS"
    echo "✅ Linting passed"
else
    LINT_STATUS="❌ FAIL"
    echo "❌ Clippy found issues. Fix them before pushing."
    OVERALL_STATUS=1
fi
rm -f "$CLIPPY_OUTPUT"
echo ""

# 3. Spell check
echo "📖 Checking spelling..."
TYPOS_OUTPUT=$(mktemp)
typos > "$TYPOS_OUTPUT" 2>&1
TYPOS_EXIT=$?
if [ $TYPOS_EXIT -eq 0 ]; then
    SPELL_STATUS="✅ PASS"
    echo "✅ Spell check passed"
else
    SPELL_STATUS="❌ FAIL"
    echo "❌ Typos found. Run 'make spell-fix' to fix."
    OVERALL_STATUS=1
fi
rm -f "$TYPOS_OUTPUT"
echo ""

# 4. Type check
echo "🔧 Type checking..."
CHECK_OUTPUT=$(mktemp)
cargo check --all-targets > "$CHECK_OUTPUT" 2>&1
CHECK_EXIT=$?
if [ $CHECK_EXIT -eq 0 ]; then
    TYPE_STATUS="✅ PASS"
    echo "✅ Type check passed"
else
    TYPE_STATUS="❌ FAIL"
    echo "❌ Type check failed. Fix errors before pushing."
    OVERALL_STATUS=1
fi
rm -f "$CHECK_OUTPUT"
echo ""

# 5. Tests
echo "🧪 Running tests..."
TEST_OUTPUT=$(mktemp)
cargo test --all-targets > "$TEST_OUTPUT" 2>&1
TEST_EXIT=$?
if [ $TEST_EXIT -eq 0 ]; then
    TEST_STATUS="✅ PASS"
    echo "✅ Tests passed"
else
    TEST_STATUS="❌ FAIL"
    echo "❌ Tests failed. Fix failing tests before pushing."
    OVERALL_STATUS=1
fi
rm -f "$TEST_OUTPUT"
echo ""

# Display summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 PRE-PUSH CHECKS SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "%-25s %s\n" "Code Formatting (fmt)" "$FORMAT_STATUS"
printf "%-25s %s\n" "Linting (clippy)" "$LINT_STATUS"
printf "%-25s %s\n" "Spell Check (typos)" "$SPELL_STATUS"
printf "%-25s %s\n" "Type Check (cargo check)" "$TYPE_STATUS"
printf "%-25s %s\n" "Tests (cargo test)" "$TEST_STATUS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $OVERALL_STATUS -eq 0 ]; then
    echo "🎉 All checks passed! Proceeding with push..."
    exit 0
else
    echo "❌ Some checks failed. Please fix the issues before pushing."
    exit 1
fi
```

Make it executable:
```bash
chmod +x .githooks/pre-push
```

### 3.3 Create Setup Script (`.githooks/setup.sh`)

Create `.githooks/setup.sh`:

```bash
#!/bin/bash
# Setup script to install git hooks
# Run this after cloning the repository

echo "🔧 Setting up git hooks..."

# Configure git to use .githooks directory
git config core.hooksPath .githooks

echo "✅ Git hooks configured successfully!"
echo ""
echo "Installed hooks:"
echo "  - pre-push: Runs formatting, linting, spell check, and tests"
echo ""
echo "To bypass hooks (not recommended):"
echo "  git push --no-verify"
```

Make it executable:
```bash
chmod +x .githooks/setup.sh
```

### 3.4 Create README (`.githooks/README.md`)

Create `.githooks/README.md`:

```markdown
# Git Hooks

This directory contains git hooks for maintaining code quality.

## Setup

Run the setup script after cloning:

\`\`\`bash
make setup-hooks
# or
./.githooks/setup.sh
\`\`\`

## Hooks

### pre-push

Runs before every `git push` to ensure:
- ✅ Code is formatted (`cargo fmt`)
- ✅ No clippy warnings
- ✅ No spelling errors
- ✅ Code compiles
- ✅ All tests pass

### Bypassing Hooks (Not Recommended)

Only in emergencies:
\`\`\`bash
git push --no-verify
\`\`\`
```

---

## 📐 Step 4: Project Structure Best Practices

### Recommended Directory Structure

```
project-root/
├── src/
│   ├── main.rs                    # Entry point
│   ├── lib.rs                     # Library root
│   ├── domain/                    # Business logic
│   │   ├── entities/             # Domain entities
│   │   ├── repositories/         # Repository traits
│   │   └── services/             # Domain services
│   ├── application/              # Use cases
│   │   ├── dto/                  # Data transfer objects
│   │   └── use_cases/            # Application use cases
│   ├── infrastructure/           # External concerns
│   │   ├── database/            # Database implementations
│   │   ├── config/              # Configuration
│   │   └── http/                # HTTP infrastructure
│   └── presentation/            # API layer
│       └── handlers/            # HTTP handlers
│           ├── mod.rs
│           ├── app_state.rs
│           └── <feature>_handlers/
│               ├── mod.rs
│               └── <feature>/
│                   ├── mod.rs
│                   ├── handler1.rs
│                   └── handler2.rs
├── tests/                        # Integration tests
│   ├── integration_test.rs
│   └── common/
├── Cargo.toml
├── Makefile
├── clippy.toml
├── rustfmt.toml
├── _typos.toml
└── .githooks/
    ├── setup.sh
    ├── pre-push
    └── README.md
```

### Handler Modularization Pattern

**BAD** (Monolithic):
```
handlers/
└── auth_handlers.rs (500+ lines)
```

**GOOD** (Modular):
```
handlers/
└── auth_handlers/
    ├── mod.rs (re-exports)
    └── auth/
        ├── mod.rs (module declarations)
        ├── dtos.rs (data types)
        ├── login_handler.rs (~60 lines)
        └── register_handler.rs (~80 lines)
```

**Benefits:**
- Each file ~40-150 lines
- Easy to navigate and modify
- Clear separation of concerns
- Consistent pattern across all modules

---

## 🔄 Step 5: Development Workflow

### Initial Setup

```bash
# 1. Clone the repository
git clone <your-repo>
cd <your-project>

# 2. Setup git hooks
make setup-hooks

# 3. Install dependencies
cargo build

# 4. Run quality checks
make quality
```

### Daily Development Workflow

```bash
# 1. Create a feature branch
git checkout -b feat/my-feature

# 2. Make changes...

# 3. Run quality checks frequently
make quality

# 4. Fix any issues
make fmt        # Auto-format code
make lint-fix   # Auto-fix linting issues
make spell-fix  # Auto-fix spelling

# 5. Run tests
cargo test

# 6. Commit changes (see Conventional Commits below)
git add .
git commit -m "feat(scope): add feature description"

# 7. Push (pre-push hook will run automatically)
git push
```

### Quick Reference Commands

```bash
make run          # Run the application
make test         # Run tests
make fmt          # Format code
make lint         # Check linting
make lint-fix     # Auto-fix linting
make check        # Type check
make spell        # Check spelling
make spell-fix    # Auto-fix spelling
make quality      # Run all checks
make build        # Build release version
make clean        # Clean artifacts
```

---

## 📝 Step 6: Conventional Commits Standard

All commits MUST follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code refactoring (no functional changes)
- `style`: Code style changes (formatting, etc.)
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `chore`: Build process, dependencies, tooling
- `perf`: Performance improvements
- `ci`: CI/CD changes

### Examples

```bash
# Feature
git commit -m "feat(auth): add password reset functionality"

# Bug fix
git commit -m "fix(database): resolve connection pool timeout issue"

# Refactor
git commit -m "refactor(handlers): modularize auth_handlers following consistent pattern"

# Style
git commit -m "style(handlers): apply cargo fmt to remove trailing spaces"

# Documentation
git commit -m "docs(readme): add setup instructions"

# Chore
git commit -m "chore(deps): update axum to 0.7.5"
```

### Multi-line Commit

```bash
git commit -m "feat(api): add user profile endpoints

Add three new endpoints:
- GET /profile - Get current user profile
- PUT /profile - Update profile
- DELETE /profile - Delete account

Includes validation, error handling, and OpenAPI docs"
```

### Atomic Commits

✅ **DO:** Make small, focused commits
```bash
git commit -m "feat(database): add users table migration"
git commit -m "feat(domain): create User entity"
git commit -m "feat(repository): implement PostgresUserRepository"
```

❌ **DON'T:** Make large, unfocused commits
```bash
git commit -m "add user stuff"  # Too vague
git commit -m "feat: add users and auth and profiles"  # Too broad
```

---

## 🚦 Step 7: Quality Standards

### Zero Warnings Policy

**All code must compile with zero warnings:**

```bash
# These must ALL pass with no warnings:
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
cargo test
```

### Pre-Push Checklist

Before every push, the pre-push hook verifies:

1. ✅ **Code formatting** - `cargo fmt --check`
2. ✅ **TOML formatting** - `taplo format --check`
3. ✅ **Linting** - `cargo clippy -- -D warnings`
4. ✅ **Spell check** - `typos`
5. ✅ **Type check** - `cargo check`
6. ✅ **Tests** - `cargo test`

**If any check fails, the push is blocked.**

### Fixing Issues

```bash
# Format issues
make fmt

# Linting issues
make lint-fix     # Auto-fix what's possible
cargo clippy      # Check remaining issues

# Spelling issues
make spell-fix    # Auto-fix typos
# or manually add terms to _typos.toml

# Test failures
cargo test        # Run and fix failing tests

# Then verify everything passes
make quality
```

---

## 🏗️ Step 8: Architecture Best Practices

### Clean Architecture Layers

```
┌─────────────────────────────────────┐
│      Presentation Layer             │  ← HTTP handlers, DTOs
│      (src/presentation/)            │
├─────────────────────────────────────┤
│      Application Layer              │  ← Use cases, orchestration
│      (src/application/)             │
├─────────────────────────────────────┤
│      Domain Layer                   │  ← Entities, business logic
│      (src/domain/)                  │     (No external dependencies)
├─────────────────────────────────────┤
│      Infrastructure Layer           │  ← Database, external services
│      (src/infrastructure/)          │
└─────────────────────────────────────┘
```

### Dependency Rule

- **Presentation** → Application → Domain ← Infrastructure
- **Domain layer** should have NO dependencies on outer layers
- **All dependencies point inward**

### Code Organization Rules

1. **Each module in its own directory**
2. **Each handler in its own file** (~40-150 lines max)
3. **Shared utilities in separate files**
4. **DTOs grouped logically**
5. **Consistent naming conventions**

---

## 🧪 Step 9: Testing Standards

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

### Testing Levels

1. **Unit Tests** - Test individual functions
   - Location: Same file as code (`#[cfg(test)]` modules)
   - Focus: Pure functions, utilities, converters

2. **Integration Tests** - Test full workflows
   - Location: `tests/` directory
   - Focus: End-to-end scenarios

3. **Domain Tests** - Test business logic
   - Location: `src/domain/` modules
   - Focus: Entities, validation, business rules

### Test Commands

```bash
cargo test                    # Run all tests
cargo test --lib             # Unit tests only
cargo test --test <name>     # Specific integration test
cargo test <pattern>         # Tests matching pattern
```

---

## 📦 Step 10: Cargo.toml Best Practices

### Essential Dependencies for Quality

```toml
[dependencies]
# Error handling
thiserror = "1.0"
anyhow = "1.0"        # For application errors

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
# Testing utilities
mockall = "0.12"      # For mocking
proptest = "1.4"      # Property-based testing (optional)
```

### Project Metadata

```toml
[package]
name = "your-project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
description = "Your project description"
repository = "https://github.com/username/project"
```

---

## 🎨 Step 11: Code Style Guidelines

### Naming Conventions

```rust
// Files and modules: snake_case
mod user_repository;
mod password_reset_service;

// Types (structs, enums): PascalCase
struct UserProfile {}
enum UserStatus {}

// Functions and variables: snake_case
fn create_user() {}
let user_id = 1;

// Constants: SCREAMING_SNAKE_CASE
const MAX_RETRIES: u32 = 3;

// Traits: PascalCase (often with -able, -er suffix)
trait Serializable {}
trait UserRepository {}
```

### Documentation

```rust
/// Brief description of function
///
/// # Arguments
///
/// * `user_id` - The ID of the user
/// * `email` - The user's email address
///
/// # Returns
///
/// Returns `Ok(User)` on success, `Err(...)` on failure
///
/// # Example
///
/// \`\`\`
/// let user = get_user(1, "test@example.com").await?;
/// \`\`\`
pub async fn get_user(user_id: i32, email: &str) -> Result<User, Error> {
    // Implementation
}
```

---

## ✅ Step 12: Verification Checklist

After setup, verify everything works:

```bash
# ✓ Git hooks installed
git config core.hooksPath
# Should output: .githooks

# ✓ Formatting works
make fmt
git status  # Should show no changes if already formatted

# ✓ Linting works
make lint
# Should output: Finished successfully

# ✓ Spell check works
make spell
# Should output: nothing (no typos)

# ✓ Quality suite works
make quality
# All checks should pass

# ✓ Tests work
cargo test
# All tests should pass

# ✓ Build works
cargo build --release
# Should complete successfully
```

---

## 🚀 Step 13: AI Agent Instructions

**When providing this guide to an AI agent, use this prompt:**

```
Please set up this Rust project following the PROJECT_SETUP_GUIDE.md.

Requirements:
1. Create all configuration files (clippy.toml, rustfmt.toml, _typos.toml)
2. Create Makefile with quality commands
3. Setup git hooks in .githooks/ directory
4. Make all scripts executable
5. Run initial quality checks to verify setup
6. Create a brief README.md documenting the setup

Follow these standards:
- Zero warnings policy
- Conventional commits
- Atomic commits
- Modular architecture (handlers in separate files)
- Pre-push quality gates

After setup, run 'make quality' and 'cargo test' to verify everything works.
```

---

## 📚 Additional Resources

### Documentation

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

### Tools

- [Clippy](https://github.com/rust-lang/rust-clippy) - Rust linter
- [Rustfmt](https://github.com/rust-lang/rustfmt) - Code formatter
- [Taplo](https://taplo.tamasfe.dev/) - TOML formatter
- [Typos](https://github.com/crate-ci/typos) - Spell checker

---

## 🎯 Summary

This setup provides:

✅ **Automated quality enforcement** via pre-push hooks  
✅ **Consistent code style** via rustfmt  
✅ **Zero warnings** via clippy strict mode  
✅ **Spell checking** via typos  
✅ **Conventional commits** for clean history  
✅ **Modular architecture** for maintainability  
✅ **Fast feedback loop** via Makefile commands  

**Result:** Professional-grade Rust project with enforced quality standards! 🚀


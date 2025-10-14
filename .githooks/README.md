# Git Hooks

This directory contains git hooks for maintaining code quality.

## Setup

Run the setup script after cloning:

```bash
make setup-hooks
# or
./.githooks/setup.sh
```

## Available Hooks

### pre-push

Runs before every `git push` and performs the following checks:

1. **Code Formatting** - Ensures code is properly formatted
2. **Linting** - Runs clippy with strict warnings
3. **Spell Check** - Checks for typos in code and docs
4. **Type Check** - Verifies code compiles
5. **Tests** - Runs all tests

If any check fails, the push is blocked.

## Bypassing Hooks

In rare cases, you may need to bypass hooks:

```bash
git push --no-verify
```

⚠️ **Not recommended** - Only use this when absolutely necessary.

## Manual Checks

You can run the checks manually:

```bash
make quality  # Run all quality checks
make test     # Run tests only
make fmt      # Format code
make lint     # Run linter
make spell    # Check spelling
```


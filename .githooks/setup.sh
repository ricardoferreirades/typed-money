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


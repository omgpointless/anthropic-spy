#!/bin/bash
# Generate changelog using conventional-changelog-cli
# Workaround for JetBrains commit hook issues

set -e

cd "$(dirname "$0")/.."

echo "Generating changelog..."
npx conventional-changelog-cli -p conventionalcommits -i ./CHANGELOG.md -s

echo "Done! CHANGELOG.md updated."

#!/bin/bash
# Generate changelog using conventional-changelog-cli
# Workaround for JetBrains commit hook issues
#
# USAGE:
#   ./scripts/changelog.sh          # Incremental: prepend new commits since last tag
#   ./scripts/changelog.sh --fresh  # Full regeneration from all commits (clean slate)
#
# HOW IT WORKS:
#   - Parses commits following Conventional Commits format (feat:, fix:, etc.)
#   - Groups them by type into sections (Features, Bug Fixes, etc.)
#   - Uses git tags as version markers
#
# TYPICAL WORKFLOW:
#   1. Make commits with conventional format
#   2. When ready to release: git tag v0.x.x
#   3. Run this script (incremental mode)
#   4. Commit the updated CHANGELOG.md
#
# WHY INCREMENTAL vs FRESH:
#   - Incremental (-r 1, default): Only adds commits since last tag. Run once per release.
#   - Fresh (-r 0): Rewrites entire file. Use when changelog got corrupted or you want
#     to start over. Running incremental multiple times without a new tag = duplicates.

set -e

cd "$(dirname "$0")/.."

if [[ "$1" == "--fresh" ]]; then
    echo "Regenerating FULL changelog from all commits..."
    npx conventional-changelog-cli -p conventionalcommits -i ./CHANGELOG.md -s -r 0
    echo "Done! CHANGELOG.md fully regenerated."
else
    echo "Generating incremental changelog (since last tag)..."
    npx conventional-changelog-cli -p conventionalcommits -i ./CHANGELOG.md -s
    echo "Done! CHANGELOG.md updated with new entries."
fi

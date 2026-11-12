#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Fills in the Homebrew cask from the zip produced by scripts/release.sh.
#
# Usage:  scripts/update-cask.sh <github-user/repo>
#   e.g.  scripts/update-cask.sh emirbugra/alt-tab-mac
# Output: dist/alt-tab-mac.rb
set -euo pipefail

cd "$(dirname "$0")/.."

REPO="${1:?usage: scripts/update-cask.sh <github-user/repo>}"
VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)"
BUNDLE_ID="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' packaging/Info.plist)"
ZIP="dist/alt-tab-mac-${VERSION}-universal.zip"
OUT="dist/alt-tab-mac.rb"

[[ -f "${ZIP}" ]] || { echo "${ZIP} not found, run scripts/release.sh first" >&2; exit 1; }

SHA256="$(shasum -a 256 "${ZIP}" | cut -d' ' -f1)"

sed -e "s|__VERSION__|${VERSION}|g" \
    -e "s|__SHA256__|${SHA256}|g" \
    -e "s|__REPO__|${REPO}|g" \
    -e "s|__BUNDLE_ID__|${BUNDLE_ID}|g" \
    packaging/homebrew/alt-tab-mac.rb.template > "${OUT}"

echo "==> ${OUT} (version ${VERSION}, sha256 ${SHA256})"

#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Builds the distribution packages (Intel + Apple Silicon in one .app):
#   dist/alt-tab-mac-<version>-universal.zip   (used by the Homebrew cask)
#   dist/alt-tab-mac-<version>.dmg            (for manual installs)
#   dist/SHA256SUMS
#
# Without an Apple Developer ID the app is ad-hoc signed; users bypass the
# "unidentified developer" warning with right-click → Open (see README).
#
# Usage: scripts/release.sh
set -euo pipefail

cd "$(dirname "$0")/.."

APP_NAME="alt-tab-mac"
VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)"
BUNDLE="target/${APP_NAME}.app"
DIST="dist"
ZIP="${DIST}/${APP_NAME}-${VERSION}-universal.zip"
DMG="${DIST}/${APP_NAME}-${VERSION}.dmg"

build_bundle() {
    SIGN_IDENTITY="-" scripts/bundle.sh
}

verify_bundle() {
    echo "==> verifying"
    codesign --verify --strict --verbose=1 "${BUNDLE}"
    local archs
    archs="$(lipo -archs "${BUNDLE}/Contents/MacOS/${APP_NAME}")"
    [[ "${archs}" == *x86_64* && "${archs}" == *arm64* ]] || { echo "not universal: ${archs}" >&2; exit 1; }
}

make_zip() {
    echo "==> ${ZIP}"
    ditto -c -k --sequesterRsrc --keepParent "${BUNDLE}" "${ZIP}"
}

make_dmg() {
    echo "==> ${DMG}"
    local staging
    staging="$(mktemp -d)"
    cp -R "${BUNDLE}" "${staging}/"
    ln -s /Applications "${staging}/Applications"
    hdiutil create -quiet -volname "${APP_NAME}" -srcfolder "${staging}" -ov -format UDZO "${DMG}"
    rm -rf "${staging}"
}

write_checksums() {
    (cd "${DIST}" && shasum -a 256 "$(basename "${ZIP}")" "$(basename "${DMG}")" > SHA256SUMS)
    cat "${DIST}/SHA256SUMS"
}

rm -rf "${DIST}"
mkdir -p "${DIST}"
build_bundle
verify_bundle
make_zip
make_dmg
write_checksums
echo "==> ready: ${DIST}/"

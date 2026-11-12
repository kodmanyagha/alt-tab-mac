#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Builds a universal (Intel + Apple Silicon) release and creates a signed .app
# bundle: target/alt-tab-mac.app
#
# Usage:
#   scripts/bundle.sh            # universal
#   scripts/bundle.sh --native   # this machine's architecture only (faster)
#
# The signing identity can be set with SIGN_IDENTITY. By default the local
# certificate from scripts/create-dev-cert.sh is used, otherwise an ad-hoc
# signature (permissions are then reset on every build).
set -euo pipefail

cd "$(dirname "$0")/.."

APP_NAME="alt-tab-mac"
BUNDLE="target/${APP_NAME}.app"
VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)"
BUNDLE_ID="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' packaging/Info.plist)"
DEV_CERT_NAME="alt-tab-mac dev"
ARCHS=(aarch64-apple-darwin x86_64-apple-darwin)

if [[ "${1:-}" == "--native" ]]; then
    ARCHS=("$(rustc -vV | sed -n 's/^host: //p')")
fi

build_binaries() {
    for target in "${ARCHS[@]}"; do
        echo "==> cargo build --release --target ${target}"
        cargo build --release --target "${target}"
    done
}

make_universal_binary() {
    local inputs=()
    for target in "${ARCHS[@]}"; do
        inputs+=("target/${target}/release/${APP_NAME}")
    done
    lipo -create "${inputs[@]}" -output "${BUNDLE}/Contents/MacOS/${APP_NAME}"
}

assemble_bundle() {
    rm -rf "${BUNDLE}"
    mkdir -p "${BUNDLE}/Contents/MacOS" "${BUNDLE}/Contents/Resources"
    sed "s/__VERSION__/${VERSION}/g" packaging/Info.plist > "${BUNDLE}/Contents/Info.plist"
    make_universal_binary
}

pick_identity() {
    if [[ -n "${SIGN_IDENTITY:-}" ]]; then
        echo "${SIGN_IDENTITY}"
    elif security find-certificate -c "${DEV_CERT_NAME}" >/dev/null 2>&1; then
        echo "${DEV_CERT_NAME}"
    else
        echo "-"
    fi
}

sign_bundle() {
    local identity
    identity="$(pick_identity)"
    echo "==> signing: ${identity}"
    codesign --force --sign "${identity}" --identifier "${BUNDLE_ID}" "${BUNDLE}"
}

build_binaries
assemble_bundle
sign_bundle
echo "==> ready: ${BUNDLE} ($(lipo -archs "${BUNDLE}/Contents/MacOS/${APP_NAME}"))"

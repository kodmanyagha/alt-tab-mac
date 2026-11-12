#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Creates a self-signed code signing certificate for development and adds it to
# the login keychain, so the signing identity stays the same across builds and
# macOS does not reset the Accessibility permission. The certificate stays on
# this machine and is not used for distribution.
set -euo pipefail

NAME="alt-tab-mac dev"
KEYCHAIN="${HOME}/Library/Keychains/login.keychain-db"

if security find-certificate -c "${NAME}" "${KEYCHAIN}" >/dev/null 2>&1; then
    echo "Certificate already exists: ${NAME}"
    exit 0
fi

WORK="$(mktemp -d)"
trap 'rm -rf "${WORK}"' EXIT

cat > "${WORK}/cert.cnf" <<EOF
[req]
distinguished_name = dn
x509_extensions = ext
prompt = no
[dn]
CN = ${NAME}
[ext]
basicConstraints = critical, CA:false
keyUsage = critical, digitalSignature
extendedKeyUsage = critical, codeSigning
EOF

openssl req -x509 -newkey rsa:2048 -nodes -days 3650 \
    -config "${WORK}/cert.cnf" -keyout "${WORK}/key.pem" -out "${WORK}/cert.pem" 2>/dev/null

PASS="alt-tab-mac"
openssl pkcs12 -export -legacy -inkey "${WORK}/key.pem" -in "${WORK}/cert.pem" \
    -name "${NAME}" -out "${WORK}/cert.p12" -passout "pass:${PASS}"

security import "${WORK}/cert.p12" -k "${KEYCHAIN}" -P "${PASS}" -T /usr/bin/codesign
echo "Created: ${NAME}"

#!/usr/bin/env bash

set -e

source local.env

DOCKERFILE_TEMPLATE="Dockerfile.template"
DOCKERFILE="Dockerfile"

echo "Resin machine name: ${RESIN_MACHINE_NAME}"

if [ ! -f ${DOCKERFILE_TEMPLATE} ]; then
    echo "Unable to locate ${DOCKERFILE_TEMPLATE} file"
    exit 1
fi

# We do not want to push target and --ignore doesn't work
cargo clean

sed "s/%%RESIN_MACHINE_NAME%%/${RESIN_MACHINE_NAME}/g" "${DOCKERFILE_TEMPLATE}" > "${DOCKERFILE}"
sudo resin local push -s . \
    --force-build \
    --app-name local-rust-thermometer \
    -b "cargo build --target=${RESIN_MACHINE_TARGET} && cp target/${RESIN_MACHINE_TARGET}/debug/thermometer target/debug/thermometer && rm -r target/${RESIN_MACHINE_TARGET}/" \
    "$@"

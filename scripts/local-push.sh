#!/usr/bin/env bash

set -e

source local-push.env

DOCKERFILE_TEMPLATE="Dockerfile.template"
DOCKERFILE="Dockerfile"

echo "Resin machine name: ${RESIN_MACHINE_NAME}"

if [ ! -f ${DOCKERFILE_TEMPLATE} ]; then
    echo "Unable to locate ${DOCKERFILE_TEMPLATE} file"
    exit 1
fi

# We do not want to push target and --ignore doesn't work, .dockerignore is ignored
#
# TODO Move target to `mktemp -d` and then put it back
cargo clean

sed "s/%%RESIN_MACHINE_NAME%%/${RESIN_MACHINE_NAME}/g" "${DOCKERFILE_TEMPLATE}" > "${DOCKERFILE}"
sudo resin local push -s . \
    --force-build \
    --ignore target \
    --env "INSIDE_THERMOMETER=28-000008e948b7" \
    --env "OUTSIDE_THERMOMETER=28-000009e9b786" \
    "$@"

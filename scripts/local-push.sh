#!/usr/bin/env bash

set -e

RESIN_MACHINE_NAME="raspberrypi3"
DEVICE="cae15d3.local"

DOCKERFILE_TEMPLATE="Dockerfile.template"
DOCKERFILE="Dockerfile"

echo "Resin machine name: ${RESIN_MACHINE_NAME}"
echo "Device: ${DEVICE}"

if [ ! -f ${DOCKERFILE_TEMPLATE} ]; then
    echo "Unable to locate ${DOCKERFILE_TEMPLATE} file"
    exit 1
fi

sed "s/%%RESIN_MACHINE_NAME%%/${RESIN_MACHINE_NAME}/g" "${DOCKERFILE_TEMPLATE}" > "${DOCKERFILE}"
sudo resin local push ${DEVICE} -s . --force-build

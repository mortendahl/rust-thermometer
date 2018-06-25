# Local push

`resin local push` does not support [Dockerfile.template](../Dockerfile.template). Proper `Dockerfile`
must be created before any local push attempt is made.

It's all handled via [local-push.sh](../scripts/local-push.sh) script. This script is driven by
environment variables set in the [local-push.env](../local-push.env) file.

Two variables are supported now:

* `RESIN_MACHINE_NAME`
* `RESIN_MACHINE_TARGET`

This repository contains values for Raspberry Pi 3 B+. 

## Sample values for Raspberry Pi 3 B+

```bash
export RESIN_MACHINE_NAME=raspberrypi3
export RESIN_MACHINE_TARGET=armv7-unknown-linux-gnueabihf
```

## Script usage

Push to specific local device:

```bash
scripts/local-push.sh cae15d3.local
```

Let Resin CLI find local device for us:

```bash
$ scripts/local-push.sh
Resin machine name: raspberrypi3
Reporting discovered devices
? select a device (Use arrow keys)
❯ cae15d3.local (10.11.12.133)
```

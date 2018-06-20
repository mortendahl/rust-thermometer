# Rust Thermometer

Thermometer prototype written in the Rust language.

* Raspberry Pi 3 Model B+
* Thermometer DS18B20
* Deployment via [resin.io](https://resin.io/)

## Prerequisites

* [resinOS](https://resinos.io) is installed on your device.
* Device is visible in your resin.io application.
* W1 is working on your device (Fleet configuration - `RESIN_HOST_CONFIG_dtoverlay=w1-gpio`).
* [Resin CLI](https://github.com/resin-io/resin-cli) is installed on your machine.

## Configuration

N/A for now. You have to add your thermometers to `src/config.rs`.

## Local deployment

* Open `scripts/local-push.sh`
    * Modify `DEVICE` variable
* Run `scripts/local-push.sh`

### Local deployment to another device type

List of [supported devices](https://docs.resin.io/reference/hardware/devices/). You have to modify:

* `resin-sync.yml` (`cargo build --target=...`)
* `scripts/local-push.sh` (`RESIN_MACHINE_NAME`)

## Related Articles

* [Rust Thermometer - Part 1](https://www.robertvojta.com/rust-thermometer-part-1/)

# Rust Thermometer

Thermometer prototype written in the Rust language.

* Raspberry Pi 3 Model B+
* Thermometer DS18B20
* Deployment via [resin.io](https://resin.io/)

Application contains third party [assets](assets). See [license](LICENSE.md) for more
details about ownership and trademarks.

## Design

Designer wanted! [Details](https://gist.github.com/zrzka/022f560d651ad259ba6945c67d7462b7).

## Prerequisites

* [resinOS](https://resinos.io) is installed on your device.
* Device is visible in your resin.io application.
* W1 is working on your device (Fleet configuration - `RESIN_HOST_CONFIG_dtoverlay=w1-gpio,pullup=1`).
* [Resin CLI](https://github.com/resin-io/resin-cli) is installed on your machine.

## Documentation

* [Local development](docs/local-development.md)
* [Local push](docs/local-push.md)

## Blog posts

* [Rust Thermometer - Part 1](https://www.robertvojta.com/rust-thermometer-part-1/)
* [Rust Thermometer - Part 2](https://www.robertvojta.com/rust-thermometer-part-2/)

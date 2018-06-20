# Change Log

## master branch

* Add third DS18B20 (Outside)
    * Two of them have three wires
    * One of them has two wires
    * Had to rewire everything to use parasitic mode
    * `RESIN_HOST_CONFIG_dtoverlay=w1-gpio,pullup=1`
* Temperature
    * Report `t=85000` as error (sensor reset, error, ...)    

## v0.0.1 (2018-06-20)

* Initial release

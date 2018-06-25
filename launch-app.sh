#!/usr/bin/env bash

# Default to UTC if no TIMEZONE env variable is set
echo "Setting time zone to ${TIMEZONE=UTC}"
echo "${TIMEZONE}" > /etc/timezone
dpkg-reconfigure tzdata

# Disable DPMS / Screen blanking
xset -dpms
xset s off
xset s noblank

mkdir /root/.config
sudo matchbox-window-manager -use_cursor no -use_titlebar no &
/usr/src/app/target/debug/thermometer
sleep 2s

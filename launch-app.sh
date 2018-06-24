#!/usr/bin/env bash

# Disable DPMS / Screen blanking
xset -dpms
xset s off
xset s noblank

mkdir /root/.config
sudo matchbox-window-manager -use_cursor no -use_titlebar no &
/usr/src/app/target/debug/thermometer
sleep 2s

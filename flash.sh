#! /usr/bin/env bash
avrdude -v -p atmega328p -P /dev/ttyACM0 -c arduino -b 115200 -D -U flash:w:target/avr-atmega328p/release/dot_games.elf:e
#! /bin/sh

set -e

echo "Cleaning elf..."

rm -rf ../target/avr-atmega328p/debug/arduino-test.elf

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <path-to-binary.elf>" >&2
    exit 1
fi

if [ "$#" -lt 1 ]; then
    echo "$0: Expecting a .elf file" >&2
    exit 1
fi

avrdude -q -C/etc/avrdude.conf -patmega328p -carduino -P$2 -D "-Uflash:w:$1:e"
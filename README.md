RustEmbedded
==================
## Definitions

[avrdude](https://github.com/avrdudes/avrdude) a toolset cli for flashing to hardware.

## Requirements
You will need the avrdude toolset.

For Linux it is as simple as:

```bash
sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
```

For MacOS:
```apple
xcode-select --install
brew tap osx-cross/avr
brew install avr-gcc avrdude
```

For Windows:
```ps
scoop install avr-gcc
scoop install avrdude
```

## Usage

Install the packages listed above and run the arguments based on your machine:

```bash
OS_DRIVER=Linux SERIAL=/dev/ttyACM0 ELF_PATH=target/avr-atmega328p/debug/arduino-test.elf cargo build -vv
```

The flag -vv is for very verbose. It prints the output within the build file without having to initialize panicking.

```bash
OS_DRIVER=Linux SERIAL=/dev/ttyACM0 ELF_PATH=target/avr-atmega328p/debug/arduino-test.elf cargo build -vv
```

The build.rs file should compile based on changes to the main.rs file. After that it runs a script to flash to the arduino. For now the program is using arduino_hal.
Reason behind including env variables in the beginning as that is the only way to pass arguments to the build script to be handled.

```bash
OS_DRIVER=Linux SERIAL=/dev/ttyACM0 ELF_PATH=target/avr-atmega328p/debug/arduino-test.elf 
```

If in error pops up saying it cannot find the elf file try, with linux, this:

```bash
file target/avr-atmega328p/debug/arduino-test.elf
```

This will link the file and it will now be ready to flash correctly and should work. Cargo is able to build a release for this program. For that replace debug with release:
```
target/avr-atmega328p/release/arduino-test.elf
```

## Issues to be fixed

The flashing after build currently takes two flash processes in order to recognize any changes. Although faster than zephyr, it is still annoying as the first flash should take.

Crude building scripts should be fixed and expound upon for better logic and reasoning.

There should be custom libs as arduino's hal is still, for unknown reasons, quite verbose.

Better logic handling.

Better building for release and debugging.
## License
Licensed under either of
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Contributions are welcomed. You can add some example libs and also some other devices you may be working with. I am implementing a system that reduces the need for example directory and can automate changing between board types without a device tree.


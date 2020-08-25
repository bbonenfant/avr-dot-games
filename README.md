# AVR Dot Games

This is a toy project of implementing various games that run on an 
Arduino Uno board with a MAX7129 8x8 LED Dot Display and PS2 JoyStick,
written in Rust.

This project is meant to be exploration into writing Rust for embedded
systems, and the code quality can be described as "polished prototype".

Games currently implemented:
* Snake (S)

The "game-play" consists of the following:
1. Interaction with a "selection screen" where you can navigate with Left and
   Right on the JoyStick, and select the game using the JoyStick press.
2. Play the game.
3. Game over screen. The player can restart the game with a JoyStick press.

The GPIO pins are hardcoded as the following:
* MAX7129 chip-select: D10
* MAX7129 clock: D13
* MAX7129 data io: D11
* JoyStick x-axis: A0
* JoyStick y-axis: A1
* JoyStick z-axis: A2

## Development
Building:
```bash
cargo build -Z build-std=core --target avr-atmega328p.json --release
```

Flashing to arduino: 
```bash
./flash.sh
```

Debugging using serial connection:
```bash
screen /dev/ttyACM0
```
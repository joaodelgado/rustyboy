# rustyboy

![status](https://travis-ci.org/joaodelgado/rustyboy.svg?branch=master)

Gameboy emulator written in rust.

This project it's still in the very early stages and does basically nothing.

## How to run

To run a rom:

    $ cargo run <path_to_rom>

To run the test suite:

    $ cargo test

To compile with clippy:

    $ cargo +nightly build --features clippy

To compile with the debugger enabled:

    $ cargo +nightly build --features debugger

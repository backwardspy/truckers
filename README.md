# trucke.rs

an _extremely_ work-in-progress ETS2/ATS plugin SDK.

tested on x86_64 windows 11 with the latest version of ETS2.
even then it might crash the game. use with caution.

## usage

build with `cargo build`, then copy the resulting `truckers.dll`/`truckers.so` to your game's `plugins` directory.

for updating on the fly, you can use the in-game console to unload & reload the plugin with `sdk unload` and `sdk reload`.

## features

![nothing](https://media1.tenor.com/m/XQLVLptLIBEAAAAd/maes-b-lost-in-a-field.gif)

## roadmap

- [x] tracing subscriber for ingame console
- [ ] finish translating the telemetry functions of the C SDK to rust
- [ ] build a high-level safe API on top of the telemetry functions
- [ ] translate the input functions of the C SDK to rust
- [ ] build a high-level safe API on top of the input functions
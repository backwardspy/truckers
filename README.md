# trucke.rs

an _extremely_ work-in-progress ETS2/ATS plugin SDK.

tested on x86_64 windows 11 with the latest version of ETS2.
even then it might crash the game. use with caution.

## usage

build with `cargo build`, then copy the resulting `truckers.dll`/`truckers.so` to your game's `plugins` directory.

for updating on the fly, you can use the in-game console to unload & reload the plugin with `sdk unload` and `sdk reload`.

## features

![nothing](https://github.com/backwardspy/truckers/assets/289746/7b505a37-8a33-4bff-9481-8540b0d627b7)

## safety

![nothing](https://github.com/backwardspy/truckers/assets/289746/7b505a37-8a33-4bff-9481-8540b0d627b7)

## roadmap

- [x] tracing subscriber for ingame console
- [x] finish translating the telemetry functions of the C SDK to rust (bindgen)
- [x] build a high-level safe API on top of the telemetry event functions
- [ ] build a high-level safe API on top of the telemetry channel functions
- [x] translate the input functions of the C SDK to rust (bindgen)
- [ ] build a high-level safe API on top of the input functions
- [ ] expose telemetry _somehow_
  - metrics
  - action items
  - learnings
  - _takeaways_

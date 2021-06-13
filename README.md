# ogage for Gameforce Chi

Prequisites
===========
- brightnessctl
- rustc
- evdev_rs

Build
=====
```
git clone https://github.com/christianhaitian/ogage.git -b gameforce-chi
cd ogage
cargo build --release
strip target/release/ogage
```
ogage executable will be in the target/release folder.

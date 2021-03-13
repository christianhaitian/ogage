# ogage

Prequisites
===========
- brightnessctl
- rustc
- evdev_rs

Build
=====
```
git clone https://github.com/christianhaitian/ogage.git -b rk2020
cd ogage
cargo build --release
strip target/release/ogage
```
ogage executable will be in the target/release folder.

# ogage for RG351P/M

Prequisites
===========
- brightnessctl
- rustc
- evdev_rs

Build
=====
```
git clone https://github.com/christianhaitian/ogage.git -b rg351
cd ogage
cargo build --release
strip target/release/ogage
```
ogage executable will be in the target/release folder.

# ogage for RG351V

Prequisites
===========
- brightnessctl
- rustc
- evdev_rs

Build
=====
```
git clone https://github.com/christianhaitian/ogage.git -b rg351v
cd ogage
cargo build --release
strip target/release/ogage
```
ogage executable will be in the target/release folder.

If you have issues with Cargo, do the following: (Thanks to romadu for the find)
================================================================================
```
apt remove cargo
apt autoremove
apt remove rustc
apt install brightnessctl rustc autotools-dev automake libtool libtool-bin libevdev-dev
```

# WACKY WHEEL
a wacky spinner wheel that gives you stuff to do on stream.


# building
https://rustup.rs you def need this

once you have rust just do this!

```sh 
git clone https://github.com/zurrty/wheel-spin/wheel-spin.git

cd wheel-spin

cargo build --release
```

## dependencies
### Windows
Windows should work fine if you have the Visual Studio build tools
### Linux
```sh
# Debian/Ubuntu
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# Fedora
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# Arch Linux
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```
### macOS
good luck buddy you are in uncharted territory. you probably just need xcode but i am not buying a mac to confirm.
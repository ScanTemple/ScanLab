# ScanLab

```bash
bun i
bun t:dev

# generate TS bindings
TS_RS_EXPORT_DIR=../app/types/ cargo test export_bindings
```

OpenCV installation: https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md

```
git clone https://github.com/microsoft/vcpkg.git ~/vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh 

./vcpkg install opencv4:x64-linux

export VCPKG_ROOT=~/vcpkg
export VCPKG_DEFAULT_TRIPLET=x64-linux

cargo build
```

git clone --recurse-submodules git@github.com:azazdeaz/good-bug.git
git config --global submodule.recurse true
cd good-bug

# packages required by Rust crates
sudo apt install build-essential cmake libclang-dev pkg-config libssl-dev libzmq3-dev

# build OpenVSlam
cd openvslam-wrap/openvslam/
make install-dependencies
# `cargo build` will also build OpenVSlam, but it's better to run it first separately to see if there are any errors
make run-cmake 
make build-api


# build on-board process
cd ../.. # go to the repo root
make build-robot-release

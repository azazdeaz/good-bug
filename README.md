# :construction: - :seedling: 🤖 :tomato: 


### Install notes (Ubuntu 21.04, RPi4)
```sh
# clone the repo with `--recurse-submodules`
git clone --recurse-submodules git@github.com:azazdeaz/good-bug.git

# packages required by Rust crates
sudo apt install build-essential cmake libclang-dev pkg-config libssl-dev libzmq3-dev

# build OpenVSlam
cd openvslam-wrap/openvslam/
make install-dependencies
# `cargo build` will also build OpenVSlam, but it's better to run it first separately to see if there are any errors
make run-cmake 
make build-api


# run on-board process
cd ../.. # go to the repo root
make run-robot-release

# run visualization
make run-robot-release
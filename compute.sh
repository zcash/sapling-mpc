#!/usr/bin/env bash

# install rust
if [[ `rustc -V | grep '('` = '' ]]
then
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
fi

# download params file
echo "download params: "$1
wget --no-check-certificate $1

# execute mpc
cargo run --release --bin compute
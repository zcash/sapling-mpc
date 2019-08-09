# sapling-mpc

This code can be used to participate Multi-party Computation(MPC) phase 2 and verify the result.

## What do I do?

Contact **jiangyuanshu@tron.network** to schedule a time to participate and get an index num. When it's your turn, you'll receive a `params` file from us.

1.Download and install IM tool [keybase](https://keybase.io/), if you don't have an account, create it first. You can use it to send file back to us later, our keybase account is `tron_brown`.

2.If you are running on macOS / Linux / another Unix-like OS, download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions:               
```
# curl https://sh.rustup.rs -sSf | sh
```

If you are running on windows, refer to [rust](https://www.rust-lang.org/learn/get-started) homepage for installation.

3.Get the source code of project sapling-mpc:
```
# git clone https://github.com/tronprotocol/sapling-mpc
```
if git not installed, please refer to [git](https://git-scm.com/downloads) first.

4.Place `params` file in the sapling-mpc directory, then run:
```
# cargo run --release --bin compute
```

The process could take one to four hours according to your hardware, occupies 1.5 ~ 2GB of memory, and then spits out a `new_params` file. The process also prints a hash. This hash is what you and others can use to verify that your contribution actually ended up in the final parameters. 

5.You are encouraged to send file `new_params` to us with keybase, and send us the hash value and your system configuration information with email, such as os version, cpu model. After we receive this `new_params` file, we will check the correctness of transformation and upload it to amazon storage if ok that next participant can use it, then we will also publish hash on [project wiki](https://github.com/tronprotocol/sapling-mpc/wiki).

## Dependency
We reuse the phase 2 result of zcash https://storage.googleapis.com/sapling-mpc/params as the first params of our ceremony. At the end of ceremony, we generate the random beacon by using one Bitcoin block's hash as the seed of pseudo random number generator, the block's height will not be determined until last participant completes.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

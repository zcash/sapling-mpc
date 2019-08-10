# sapling-mpc

This document can be used to participate Multi-party Computation(MPC) phase 2 and verify the result.

## What should I do?

1. Send a mail to **jiangyuanshu@tron.network** to apply to participate MPC. You should introduce yourself in email.

2. Tron will send you a email to show your turn and time to participate MPC.

3. When it is your turn, tron will send your a email to notice you and give you a `params` file link which you need to download.

4. Download the `params` file.

5. Download and install IM tool [keybase](https://keybase.io/), if you don't have an account, create it first and add tron' account to your friends. Tron's account is `tron_brown`.

6. Download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions if you are running on macOS / Linux / another Unix-like OS:               

```
# curl https://sh.rustup.rs -sSf | sh
```

&emsp;&emsp;If you are running on windows, refer to [rust](https://www.rust-lang.org/learn/get-started) homepage for installation.

7. Get the source code of project sapling-mpc:
```
# git clone https://github.com/tronprotocol/sapling-mpc
```
&emsp;&emsp;if git has not been installed on your computer, please refer to [git](https://git-scm.com/downloads) first.

8. Put `params` file in the sapling-mpc directory, then run:
```
# cargo run --release --bin compute
```
&emsp;&emsp;When it finish, you will get a `new_params` file and hash. Must remember the hash. You can find the hash and `new_params` as follows:

![](https://raw.githubusercontent.com/tronprotocol/documentation-EN/master/docs_without_index/MPC/sapling-output.jpg)

&emsp;&emsp;Notice: this process could take one to four hours according to your hardware, occupies 1.5 ~ 2GB of memory.

9. Send your result to Tron. Send the file `new_params` by keybase and send hash by email, you can record os version in email, such as windows 10/centos 7.


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

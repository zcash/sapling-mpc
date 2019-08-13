# sapling-mpc

This document is a guide to participating in Multi-party Computation(MPC) phase 2 and verifying the result.

## What should I do?

1. Send an email to Brown.Jiang@tron.network to apply to participate in MPC. Please introduce yourself in the email.

2. TRON will send you an email about the time for you to participate in MPC.

3. When it is your turn, TRON will send you a notification email and give you a params file link which you will need to download.

4. Download the `params` file.

5. Download and install IM tool [keybase](https://keybase.io/). If you don't have an account yet, create an account and add TRON' account as your friends. TRON's account is `tron_brown`.

6. Run the following code in your terminal to download Rustup and install Rust, then follow the instructions if you are running on macOS / Linux / another Unix-like OS:
```
# curl https://sh.rustup.rs -sSf | sh
```

&emsp;&emsp;If you are running on windows, please go to [rust](https://www.rust-lang.org/learn/get-started) homepage for installation instruction.

7. Obtain the source code of project sapling-mpc:
```
# git clone https://github.com/tronprotocol/sapling-mpc
```
&emsp;&emsp;If git has not been installed on your computer, please refer to [git](https://git-scm.com/downloads) first.

8. Put `params` file in the sapling-mpc directory, then run:
```
# cargo run --release --bin compute
```
&emsp;&emsp;When it’s finished, you will get a `new_params` file and a `hash` which you will need to memorize. You can find the `hash` and `new_params` as shown below:
![](https://raw.githubusercontent.com/tronprotocol/documentation-EN/master/docs_without_index/MPC/sapling-output.jpg)

&emsp;&emsp;Notice: this process could take one to four hours according to your hardware capacity and occupy 1.5 ~ 2GB of memory.

9. Send your result to TRON. Send the file `new_params` by keybase and `hash` by email. Feel free to state your os version in email. E.g. windows 10/centos 7.

## Dependency
We reuse the phase 2 result of zcash https://storage.googleapis.com/sapling-mpc/params as the first `params` of the process. At the end of process, we generate the random beacon by using one Bitcoin block's hash as the seed of pseudo random number generator; The block's height will not be determined until last participant completes his operation.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

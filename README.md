# sapling-mpc

This code can be used to participate and verify the Sapling network upgrade MPC.

## What do I do?

Contact **mpc@z.cash** to schedule a time to participate. You'll need the latest (stable) [Rust compiler](https://www.rust-lang.org/) to participate using this code.

When it's your turn, you'll receive a `params` file from us. Place this file in the current directory and run:

```
cargo run --release --bin compute
```

This will compute for a little while, and then spit out a `new_params` file. That's what you'll upload back to us.

The tool also prints a hash. This hash is what you and others can use to verify that your contribution actually ended up in the final parameters, so you're encouraged to save it to check later!

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

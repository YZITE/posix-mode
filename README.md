# yz-posix-mode

This crate provides the types `FileType` and `Mode`.

## Features

 * `nix`: Provides compatibility with the `nix` crate
   (implements the `From` & `Into` traits).
 * `num`: Provides conversion between primitive integer
   types and the types defined in this crate.
 * `serde`: Activates implementations of the `serde`
   `Deserialize` & `Serialize` traits.
 * `umask`: Provides compatibility with the `umask` crate
   (similiar to the `nix` feature, just for another crate)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

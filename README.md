# grue

[![Latest version](https://img.shields.io/crates/v/grue.svg)](https://crates.io/crates/grue)
[![Documentation](https://docs.rs/grue/badge.svg)](https://docs.rs/grue)
[![](https://tokei.rs/b1/github/gwihlidal/grue-rs)](https://github.com/gwihlidal/grue-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![APACHE2](https://img.shields.io/badge/license-APACHE2-blue.svg)

Various tools and algorithms for building rpg and adventure games

https://en.wikipedia.org/wiki/Grue_(monster)

```txt
It is pitch black. You are likely to be eaten by a grue.
Further investigation will reveal more about their nature:
```

`> what is a grue?`

```txt
The grue is a sinister, lurking presence in the dark places of the
earth. Its favorite diet is adventurers, but its insatiable appetite
is tempered by its fear of light. No grue has ever been seen by the
light of day, and few have survived its fearsome jaws to tell the tale.
```

https://www.wihlidal.com/blog/general/2019-07-14-name-generation/

## Name Generator

```sh
cargo run --release --example namegen
```

## Markov Generator

```sh
cargo run --release --example markov
```

## Adventure Generator

```sh
cargo run --release --example adventure
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
grue = "0.1.0"
```

and add this to your crate root:

```rust
extern crate grue;
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

Contributions are always welcome; please look at the [issue tracker](https://github.com/gwihlidal/grue-rs/issues) to see what
known improvements are documented.

## Code of Conduct

Contribution to the grue crate is organized under the terms of the
Contributor Covenant, the maintainer of grue, @gwihlidal, promises to
intervene to uphold that code of conduct.

# SoFiZa

An SFZ format parser

[![Crate](https://img.shields.io/crates/v/sofiza.svg)](https://crates.io/crates/sofiza)
[![API](https://docs.rs/sofiza/badge.svg)](https://docs.rs/sofiza/)
[![dependency status](https://deps.rs/crate/sofiza/0.1.2/status.svg)](https://deps.rs/crate/sofiza/0.1.2)
[![MSRV: 1.43.0](https://flat.badgen.net/badge/MSRV/1.43.0/purple)](https://blog.rust-lang.org/2020/04/23/Rust-1.43.0.html)
[![Lines Of Code](https://tokei.rs/b1/github/andamira/sofiza?category=code)](https://github.com/andamira/sofiza)

## Implementation progress

The only implemented opcodes for now are the ones necessary to parse the
sfz files present in the following free (as in freedom) musical libraries:

- [Versilian Studios Chamber Orquestra 2 (Community Edition)](https://vis.versilstudios.com/vsco-community.html)
  - [VSCO-2-CE-1.1.0](https://github.com/sgossner/VSCO-2-CE)
- [Versilian Community Sample Library](https://vis.versilstudios.com/vcsl.html)
  - [VCSL-1.2.2-RC](https://github.com/sgossner/VCSL)
- [Virtual Playing Orchestra Free Sample Library](http://virtualplaying.com/)
- [Salamander Grand Piano v3](https://musical-artifacts.com/artifacts/3)
- [SCC Taiko Drums](http://www.schristiancollins.com/vi-percussion.php)


## Future plans

My intention is to provide conversion to Csound code, which will allow to
create samplers, among other things.

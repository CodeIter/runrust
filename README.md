# runrust - Run Rust script

[![MSRV](https://img.shields.io/badge/rustc-1.64.0+-ab6000.svg)](https://blog.rust-lang.org/2022/09/22/Rust-1.64.0.html)

- Original `rust-script` :
  - [![rust-script CI](https://github.com/fornwall/rust-script/workflows/CI/badge.svg)](https://github.com/fornwall/rust-script/actions?query=workflow%3ACI)
    [![Crates.io/rust-script](https://img.shields.io/crates/v/rust-script.svg)](https://crates.io/crates/rust-script)

- `runrust` :
  - [![runrust CI](https://github.com/CodeIter/runrust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/CodeIter/runrust/actions/workflows/ci.yml)
    [![Crates.io/runrust](https://img.shields.io/crates/v/rust.svg)](https://crates.io/crates/runrust)

Run Rust script files without any setup or explicit compilation step,
with seamless use of crates specified as dependencies inside the scripts.

- **Example**:
```bash

set -e

cargo install runrust

cat - << EOF > my-rust-script.rs
#!/usr/bin/env runrust
//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! rand = "0.8.0"
//! ```
use rand::prelude::*;
fn main() {
    let x: u64 = random();
    println!("A random number: {}", x);
}
EOF

chmod -c u+x my-rust-script.rs

./my-rust-script.rs
```

- **Output**:
```txt
A random number: 9240261453149857564
```

Rust version 1.64 or newer required.

See the [documentation at codeiter.github.io/runrust](https://codeiter.github.io/runrust).

## Related projects
- [rust-script](https://github.com/fornwall/rust-script) - the original project that `runrust` was forked from.
- [cargo-script](https://github.com/DanielKeep/cargo-script) - the unmaintained project that `rust-script` was forked from.
- [cargo-eval](https://github.com/reitermarkus/cargo-eval/) - maintained fork of `cargo-script`.
- [cargo-play](https://github.com/fanzeyi/cargo-play) - local Rust playground.
- [runner](https://github.com/stevedonovan/runner/) - tool for running Rust snippets.
- [scriptisto](https://github.com/igor-petruk/scriptisto) - language-agnostic "shebang interpreter" that enables you to write scripts in compiled languages.
- [official cargo-script RFC](https://github.com/rust-lang/cargo/issues/12207) - in progress integration into cargo

## License
`runrust` is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

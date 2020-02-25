# falconre

Falconre is a python3 library using [pyo3](https://github.com/PyO3/pyo3) to wrap:

  * [Falcon](https://github.com/falconre/falcon) - A binary analysis framework in Rust
  * [Finch](https://github.com/falconre/finch) - A symbolic executor built on Falcon
  * [Raptor](https://github.com/falconre/raptor) - Higher order IR and analysis on top of Falcon

This is beta-quality software. Bug reports are welcome.

# Installing

## Docker

```
docker build -t falconre .
```

Start a python3 interpreter:

```
docker run --rm -ti -v $(pwd)/examples:/examples falconre python3 examples/bin_ls.py
```

## Natively on OSX

Install rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You'll need this thing called "Rust nightly":

```
rustup toolchain install nightly
```

Build everything:

```
cargo +nightly build --release --features capstone4
```

Copy `target/release/libfalconre.dylib` to... somewhere in `sys.path` and then `import falconre`.
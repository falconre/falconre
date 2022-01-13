# falconre

Falconre is a python3 library using [pyo3](https://github.com/PyO3/pyo3) to wrap:

  * [Falcon](https://github.com/falconre/falcon) - A binary analysis framework in Rust
  * [Finch](https://github.com/falconre/finch) - A symbolic executor built on Falcon
  * [Raptor](https://github.com/falconre/raptor) - Higher order IR and analysis on top of Falcon

This is alpha-quality software.

## What is this really?

This is me (endeav0r) hacking on Falcon and other things to try and automate
different simple static analysis tasks.

Here are some things you can try:

### Print out all the function calls in a program

```bash
git clone https://github.com/endeav0r/corpora
git clone https://github.com/falconre/falconre

# Build some example programs
pushd corpora
./build.sh
popd

# Build falconre in a Docker container
pushd falconre
docker build -t falconre .
# Watch youtube, this will take a few minutes
popd

docker run --rm -ti -v $(pwd):/opt falconre \
  python3 /opt/falconre/examples/print-calls.py \
  /opt/corpora/build/stack_buffer/vuln/one

```

### Find trivial stack buffer overflows

```bash
# Run the example stack-writes.py script against a trivially vulnerable stack
# buffer overflow program.
docker run --rm -ti -v $(pwd):/opt falconre \
  python3 /opt/falconre/examples/stack-writes.py \
  /opt/corpora/build/stack_buffer/vuln/one

# Run the example stack-writes.py against a non-vulnerable version of the same
# program.
docker run --rm -ti -v $(pwd):/opt falconre \
  python3 /opt/falconre/examples/stack-writes.py \
  /opt/corpora/build/stack_buffer/not_vuln/one
```

### Symbolically execute a toy amd64 assembly function

```bash
docker run --rm -ti -v $(pwd):/opt falconre \
  python3 /opt/falconre/examples/symex-one.py \
  /opt/corpora/build/symex/one
```

### Print out DOT graph of Falcon IL

```bash
docker run --rm -ti -v $(pwd):/opt falconre \
  python3 /opt/falconre/examples/falcon-dot-graph.py \
  /opt/corpora/build/symex/one run | dot -Tpng -o /tmp/main.png
```

# Installing

## Docker

```
docker build -t falconre .
```

## Natively on OSX

_This is how I use falconre._

Install rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Get the dependencies. _Note: You must have capstone 4.0.2 to use falcon._

```
brew install z3 capstone
```

Install with setuptools

```
python3 setup.py install
```

If you don't want to install with setuptools, assuming you want to run the
examples:

```
cargo build --release
cp target/release/libfalconre.dylib examples/falconre.so
```
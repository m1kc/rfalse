# rfalse

An interpreter for the [FALSE](https://esolangs.org/wiki/FALSE) language — possibly the fastest in the world.

- Supports all language features;
- Written in Rust;
- Up to 38 times more performant than [false-js](http://www.quirkster.com/iano/js/false-js.html) (see benchmarks below). If you are aware of any faster implementations, please let me know!

## Table of Contents

- [Benchmarks](#benchmarks)
- [How to run](#how-to-run)
- [WASM support](#wasm-support)
- [Hacking](#hacking)
- [License](#license)

## Benchmarks

| Test | false-js | rfalse | difference |
|------|----------|--------|------------|
| Primes, n=1999 | 4392 ms | **168 ms** | ~26x
| Fibonacci, n=33 | 37597 ms | **987 ms** | ~38x
| Fibonacci, n=25 | 802 ms | **21 ms** | ~38x


## How to run

```sh
cargo run --release
```

## WASM support

Experimental feature. WASM performance is about 2x worse than native build.

First, compile the WASM files:

```sh
wasm-pack build --release --target web
```

After that, run a webserver. For example:

```
python3 -m http.server 8000 -d .
```

Navigate to http://localhost:8000 and open the console.

## Hacking

Run linter:

```sh
cargo clippy
```

Run tests:

```sh
cargo test
```

Run benchmarks:

```sh
cargo bench --bench perf
```

## License

GNU LGPL v3

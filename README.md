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
- [See also](#see-also)

## Benchmarks

| Test | false-js | rfalse | difference |
|------|----------|--------|------------|
| Primes, n=1999 | 4392 ms | **168 ms** | ~26x
| Fibonacci, n=33 | 37597 ms | **987 ms** | ~38x
| Fibonacci, n=25 | 802 ms | **21 ms** | ~38x

Experimental VM:

| Test | rfalse-vm2 | false-js | solkin/false-vm |
|------|------------|----------|-----------------|
| Primes, n=1999 | **102 ms** | 4392 ms (~43x slower) | 154 ms (~1.5x slower)
| Fibonacci, n=33 | **435 ms** | 37597 ms (~86x slower) | 504 ms (~1.16x slower)
| Fibonacci, n=25 | **9 ms** | 802 ms (~89x slower) | 11 ms (~1.2x slower)

## How to run

```sh
cargo run --release -- examples/hello.false
```

In no filename is given, stdin will be used instead.

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

## See also

* [FALSE home page](https://strlen.com/false-language/)
* [FALSE page on Esolangs](https://esolangs.org/wiki/FALSE) — the closest thing we have to a language reference
* [Archived language reference](https://web.archive.org/web/20110716155733/http://strlen.com/false/false.txt), was very hard to find
* [FALSE article on Russian Wikipedia](https://ru.wikipedia.org/wiki/FALSE) which contains some useful explanations; funny enough, there's no English version

Other notable implementations:

* [Awesome FALSE interpreter](https://github.com/solkin/false-vm) by Igor Solkin
* [false-js](https://www.quirkster.com/iano/js/false-js.html) by Ian Osgood
* [JavaScript FALSE interpreter](https://morphett.info/false/false.html) by Anthony Morphett

# rfalse

An interpreter for the [FALSE](https://esolangs.org/wiki/FALSE) language.

- Supports all language features;
- Written in Rust;
- Up to 5x faster than [false-js](http://www.quirkster.com/iano/js/false-js.html) (test: Primes, n=1999; 4392 ms vs. 869 ms)

### Table of Contents

- [How to run](#how-to-run)
- [WASM support](#wasm-support)
- [How to run tests](#how-to-run-tests)
- [License](#license)

### How to run

```sh
cargo run --release
```

### WASM support

Experimental feature. First, compile the WASM files:

```sh
wasm-pack build --release --target web
```

After that, run a webserver. For example:

```
python3 -m http.server 8000 -d .
```

Navigate to http://localhost:8000 and open the console.

### How to run tests

```sh
cargo test
```

### License

GNU LGPL v3

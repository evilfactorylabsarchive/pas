# pas

A random passphrase generator using indonesian wordlist.

## Notes

This is only used for poc to generate random passphrase based on @geovedi's indonesian-wordlist
[dataset](https://github.com/geovedi/indonesian-wordlist) using `rand` crates.

Also, for now the `05-ivanlanin2011-sort-alpha.lst` file (dataset) is required to live in cwd to
make this program works. In future, we'll embed the files (~156K) into program at compile-time.

## Development

Clone this repository, run `cargo run`, that's it.

## Wasi

If you want build the project as Wasm in [WASI](https://wasi.dev) format, first of all:

You need to add `wasm32-wasi` as the build target:

```
rustup target add wasm32-wasi
```

And then build the project for that target:

```
cargo build --target wasm32-wasi
```

Verify the build by using this command:

```
$ file target/wasm32-wasi/debug/pas.wasm
target/wasm32-wasi/debug/pas.wasm: WebAssembly (wasm) binary module version 0x1 (MVP)
```

If the output similar with above, your wasm module is ready to use.

This is the example using [wasmtime](https://wasmtime.dev/) runtime.

```
$ wasmtime --version
0.7.0

$ wasmtime --dir=. target/wasm32-wasi/debug/pas.wasm
kedai kasur meringkus mukjizat bergegas miris kliyengan melarat peradaban menggerumit intuisi jujur
```

Pretty good.

Btw, this is just for fun (unoptimized build):

```bash
λ  hyperfine --warmup 3 './target/debug/pas' 'wasmtime --dir=. target/wasm32-wasi/debug/pas.wasm'
Benchmark #1: ./target/debug/pas
  Time (mean ± σ):      46.6 ms ±   3.5 ms    [User: 42.2 ms, System: 2.2 ms]
  Range (min … max):    42.3 ms …  57.6 ms    46 runs

Benchmark #2: wasmtime --dir=. target/wasm32-wasi/debug/pas.wasm
  Time (mean ± σ):     317.4 ms ±  11.6 ms    [User: 289.0 ms, System: 23.3 ms]
  Range (min … max):   302.3 ms … 339.8 ms    10 runs

Summary
  './target/debug/pas' ran
    6.80 ± 0.57 times faster than 'wasmtime --dir=. target/wasm32-wasi/debug/pas.wasm'

λ  du -sh target/release/pas
344K	target/release/pas

λ  du -sh target/wasm32-wasi/debug/pas.wasm
3.7M	target/wasm32-wasi/debug/pas.wasm

λ  time ./target/debug/pas > /dev/null
        0.04 real         0.04 user         0.00 sys
```

## License

WTFPL.

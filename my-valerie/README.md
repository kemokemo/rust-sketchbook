# my-valerie

My tiny rust webassembly sample using [valerie](https://github.com/emmanuelantony2000/valerie).

## build

When you change the `src/lib.rs`, run following.

```sh
wasm-pack build --target web --out-name wasm --out-dir ./static
```

## run

Using `miniserve`, check the result via `http://localhost:8081`.

```sh
miniserve -p 8081 ./static
```

Open `http://127.0.0.1:8081/index.html` by your browser.

## examples

Other examples of common WebAssembly implementations using Rust can be found [here](https://github.com/rustwasm/wasm-bindgen/tree/master/examples).

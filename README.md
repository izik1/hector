# Hector

### What?

Hector is a hex encoding library,
persuing the library trifecta of fast, free, and easy to use.

(Safety is a given.)

### Why?

Because I can.

## Install

```toml
# Cargo.toml
[dependencies]
hector = "0.1" # or the current latest version.
```

## MSRV

Hector currently targets the latest stable Rust.
Hector may compile on older versions of Rust...
But until certain nightly only features are stable,
Hector will likely only target the most recent stable.

## Examples

```rust
const MESSAGE: &str = "Hello, Hector!";
let encoded = hector::encode(MESSAGE);

println!("{MESSAGE}: {encoded}");

let decoded = hector::decode(&encoded)?;

assert_eq!(&*decoded, MESSAGE.as_bytes())
```

## License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Project status

Currently _encoding_ is at a fairly nice point,
other than possible SIMD support...

_decoding_ on the other hand needs a decent chunk more polish in the following areas:

- array decoding (like encoding has)
- slice decoding (once again, like encoding has)
- performance(?), likely not as fast as it could be.

## No std support?

Yup, now and forever. 

# WASI

A Rust wrapper for calling [WASI](https://wasi.dev)

[Documentation](https://docs.rs/wasi/)

This wrapper adheres to this [API](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md) to the best of its ability. Note that this API is extremely new and may change.

# Example

```rust
use wasi::*;
fn main(){
  let r = random::get();
  let d6 = 6*r;
  console_log(&format!("you rolled a {}",d6));
}
```

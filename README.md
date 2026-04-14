# rust_dylib

Simplify Rust's invocation of dynamic link libraries with usage similar to C#'s P/Invoke. When the library does not exist, clear error information will be provided, unlike the official #\[link(name = "my_lib", kind = "raw-dylib")] which does not output effective debugging information. (tested 2026)

It's base on libloading 0.8

1. Add dependency to your project:

```toml
[dependencies]
rust_dylib = “0.1.0”
```

2. Use the #[import("my_lib.dll")] macro to import functions from the dynamic link library.

```rust
use rust_dylib::import;

#[import("my_lib.dll")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}
```

3. Call the imported function just like any other Rust function.

```rust
let result = add(10, 20);
```

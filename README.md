# rust\_dylib

Simplify Rust's invocation of dynamic link libraries with usage similar to C#'s P/Invoke. When the library does not exist, clear error information will be provided, unlike the official #\[link(name = "my\_lib", kind = "raw-dylib")] which does not output effective debugging information. (tested 2026)



It's base on libloading 0.8


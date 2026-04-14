use rust_dylib::import;

#[import("my_lib.dll")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    // 调用 DLL 函数
    let result = add(10, 20);
    println!("10 + 20 = {}", result);
}

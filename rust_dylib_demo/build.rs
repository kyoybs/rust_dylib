// build.rs
fn main() {
    // // 1. 设置库的搜索路径
    // // 假设你的 mylib.lib 文件在项目根目录下的 "lib" 文件夹里
    // // 将 "lib" 换成你存放 .lib 文件的实际路径
    // // windows下，会自动查找exe程序所在目录
    // println!("cargo:rustc-link-search=native=./lib");

    // 2. 指定要链接的库的名字
    // 假设 .lib 文件名为 "mylib.lib"，名字就是 "mylib"
    // `static` 关键字告诉编译器我们希望静态链接
    //println!("cargo:rustc-link-lib=static=my_lib");

    // 3. （可选）如果库路径发生变更，让 Cargo 重新运行 build.rs
    println!("cargo:rerun-if-changed=lib");
}

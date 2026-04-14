use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemForeignMod, LitStr};

/// 动态链接库调用宏, 类似c# pinvoke
/// 用法：#[import("my_lib.dll")] extern "C" { fn add(...) -> ...; }
#[proc_macro_attribute]
pub fn import(args: TokenStream, input: TokenStream) -> TokenStream {
    // 1. 解析库路径参数（如 "my_lib.dll"）
    let lib_path = parse_macro_input!(args as LitStr).value();
    // 2. 解析 extern "C" 函数块
    let foreign_mod = parse_macro_input!(input as ItemForeignMod);

    // 收集所有 extern 函数
    let mut functions = Vec::new();
    for item in foreign_mod.items {
        if let syn::ForeignItem::Fn(func) = item {
            functions.push(func);
        }
    }

    // 3. 生成安全的包装代码
    let gen_code = generate_wrapper_code(&lib_path, &functions);
    gen_code.into()
}

/// 生成懒加载动态库 + 函数包装代码
fn generate_wrapper_code(
    lib_path: &str,
    functions: &[syn::ForeignItemFn],
) -> proc_macro2::TokenStream {
    let mut wrappers = Vec::new();

    for func in functions {
        let fn_name = &func.sig.ident; // 函数名：add
        let fn_abi = &func.sig.abi; // 调用约定："C"
        let fn_inputs = &func.sig.inputs; // 参数：a: i32, b: i32
        let fn_output = &func.sig.output; // 返回值：i32
        let fn_args_names: Vec<_> = func
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(pat) => Some(&pat.pat),
                _ => None,
            })
            .collect();

        // 生成：静态懒加载的函数指针（使用不同的名称避免冲突，使用大写遵循命名规范）
        let fn_ptr_name =
            syn::Ident::new(&format!("{}_PTR", fn_name).to_uppercase(), fn_name.span());
        let static_fn_ptr = quote! {
            static #fn_ptr_name: once_cell::sync::Lazy<libloading::Symbol<#fn_abi fn(#fn_inputs) #fn_output>> = once_cell::sync::Lazy::new(|| {
                // 全局懒加载动态库
                static LIB: once_cell::sync::Lazy<libloading::Library> = once_cell::sync::Lazy::new(|| {
                    unsafe {
                        libloading::Library::new(#lib_path).expect(&format!("动态库加载失败: {}", #lib_path))
                    }
                });
                // 查找函数符号
                unsafe { LIB.get(stringify!(#fn_name).as_bytes()) }.expect(&format!("函数符号查找失败: {} in {}", stringify!(#fn_name), #lib_path))
            });
        };

        // 生成：安全的调用函数（对外暴露）
        let wrapper_fn = quote! {
            #[inline(always)]
            pub fn #fn_name(#fn_inputs) #fn_output {
                unsafe { (**#fn_ptr_name)(#(#fn_args_names),*) }
            }
        };

        wrappers.push(quote! {
            #static_fn_ptr
            #wrapper_fn
        });
    }

    // 合并所有生成代码
    quote! {
        #(#wrappers)*
    }
}

extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();

    csbindgen::Builder::default()
    .input_extern_file("./src/compiler.rs")
    .csharp_dll_name("agl.dll")
    .csharp_class_accessibility("public")
    .generate_csharp_file("./NativeMethodsAGL.g.cs")
    .unwrap();
}
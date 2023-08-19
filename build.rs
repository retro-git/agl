use std::process::Command;

extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();

    // Generate Swift bindings with uniffi
    let udl_file = "./src/agl.udl";
    let out_dir= "./bindings";
    uniffi::generate_scaffolding("src/agl.udl").unwrap();
    uniffi::generate_bindings(udl_file.into(), None, vec![uniffi::TargetLanguage::Swift], Some(out_dir.into()), None, true).unwrap();

    // Command::new("uniffi-bindgen-cs").arg("--out-dir").arg(out_dir).arg(udl_file).output().expect("Failed when generating C# bindings");

    csbindgen::Builder::default()
    .input_extern_file("./src/compiler.rs")
    .csharp_dll_name("agl.dll")
    .csharp_class_accessibility("public")
    .generate_csharp_file("./bindings/NativeMethodsAGL.g.cs")
    .unwrap();
}
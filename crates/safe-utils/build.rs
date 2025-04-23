use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=../../ts-eel");

    let mut eel_compiler = Command::new("deno");

    eel_compiler
        .arg("compile")
        .arg("--allow-all")
        .arg("--output")
        .arg("dist/")
        .arg("--target")
        .arg(target_triple::TARGET)
        .arg("main.ts")
        .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../../ts-eel"));

    eel_compiler.status().expect("failed to build, did you install deno?");
}

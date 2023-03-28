use std::env;
use std::process;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let key = "OS_DRIVER";
    match env::var(key) {
        Ok(val) => {
            let key_ = "SERIAL";
            let file_path_key = "ELF_PATH";
            println!("{key}: {val:?}");
            match val.as_str() {
                "Linux" => {
                    println!("Running linux flasher.");
                    process::Command::new("sh")
                    .arg("./scripts/flash_.sh")
                    .arg(env::var(file_path_key).as_deref().unwrap())
                    .arg(
                        env::var(key_)
                        .as_deref()
                        .unwrap()
                    )
                    .spawn()
                    .expect("Could not run script.");
                },
                "Windows" => println!("Running windows flasher."),
                _ => println!("No OS found, therefore cannot run flash script.")
            }
        },
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }
    // println!("In file {}", file_path);
    // let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // File::create(out.join("memory.x"))
    //     .unwrap()
    //     .write_all(include_bytes!("memory.x"))
    //     .unwrap();
    // println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=main.rs");
}

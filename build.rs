use std::process::Command;

fn main() {
    // Run `python3-config --ldflags` to determine library paths
    let output = Command::new("python3-config")
        .arg("--ldflags")
        .output()
        .expect("Failed to run `python3-config`");

    let ldflags = String::from_utf8(output.stdout)
        .expect("Failed to parse output from `python3-config`");

    // Extract the relevant `-L` and `-rpath` paths
    for flag in ldflags.split_whitespace() {
        if flag.starts_with("-L") {
            let lib_path = flag.trim_start_matches("-L");
            println!("cargo:rustc-link-search=native={}", lib_path);
        }
        if flag.starts_with("-Wl,-rpath,") {
            let rpath = flag.trim_start_matches("-Wl,-rpath,");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", rpath);
        }
    }

    // Link the Python library
    println!("cargo:rustc-link-lib=dylib=python3.13");
}

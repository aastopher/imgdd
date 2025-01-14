use std::process::Command;
use std::str;

fn main() {
    if std::env::var("CARGO_FEATURE_BENCHMARK").is_ok() {
        // Run `python --version` to dynamically determine system python version
        let output = Command::new("python")
            .arg("--version")
            .output()
            .expect("Failed to execute `python --version`");
        
        let version_output = str::from_utf8(&output.stdout)
            .expect("Failed to parse `python --version` output")
            .trim(); // "Python 3.13.0"

        // Extract the major and minor version (e.g., "3.13")
        let version_parts: Vec<&str> = version_output.split_whitespace().nth(1).unwrap_or_default().split('.').collect();
        let python_version = format!("python{}.{}", version_parts[0], version_parts[1]);

        // Run `python3-config --ldflags` to collect library paths
        let ldflags_output = Command::new("python3-config")
            .arg("--ldflags")
            .output()
            .expect("Failed to run `python3-config`");

        let ldflags = str::from_utf8(&ldflags_output.stdout)
            .expect("Failed to parse output from `python3-config`");

        // Extract relevant `-L` and `-rpath` paths
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

        // Dynamically link the python library to current version
        println!("cargo:rustc-link-lib=dylib={}", python_version);
    }

    // Ensure `pyo3` build script is invoked
    println!("cargo:rerun-if-env-changed=PYTHON_SYS_EXECUTABLE");
    println!("cargo:rerun-if-changed=imgdd/imgdd.pyi");
}

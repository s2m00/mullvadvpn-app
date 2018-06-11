#[cfg(windows)]
mod win {
    use std::env;
    use std::path::PathBuf;

    static WINFW_BUILD_DIR: &'static str = "..\\windows\\winfw\\bin";

    pub fn default_winfw_output_dir() -> PathBuf {
        let target = env::var("TARGET").expect("TARGET env var not set");

        let target_dir = match target.as_str() {
            "i686-pc-windows-msvc" => format!("Win32-{}", get_build_mode()),
            "x86_64-pc-windows-msvc" => format!("x64-{}", get_build_mode()),
            _ => panic!("uncrecognized target: {}", target),
        };

        manifest_dir().join(WINFW_BUILD_DIR).join(&target_dir)
    }

    fn manifest_dir() -> PathBuf {
        env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .expect("CARGO_MANIFEST_DIR env var not set")
    }

    fn get_build_mode() -> &'static str {
        let profile = env::var("PROFILE").expect("PROFILE env var not set");
        if profile == "release" {
            "Release"
        } else {
            "Debug"
        }
    }
}

#[cfg(windows)]
fn main() {
    use std::env;
    use std::path::PathBuf;
    use win::*;

    let winfw_dir = env::var_os("WINFW_INCLUDE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_winfw_output_dir);

    println!(
        "cargo:rustc-link-search={}",
        winfw_dir
            .to_str()
            .expect("failed to construct path for winfw include directory")
    );
    println!("cargo:rustc-link-lib=dylib=winfw");
}

#[cfg(not(windows))]
fn main() {}
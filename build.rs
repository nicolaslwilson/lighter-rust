use std::{env, fs, path::PathBuf};

// We will build the `lighter-signer` bindings here
// instead of mapping them manually, since we have the header files from v0.1.3
fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // header file
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let (header, lib_path, lib_name) = (
        format!("{dir}/libs/linux/amd64/liblighter-signer.h"),
        format!("{dir}/libs/linux/amd64/liblighter-signer.so"),
        "liblighter-signer.so",
    );
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    let (header, lib_path, lib_name) = (
        format!("{dir}/libs/linux/arm64/liblighter-signer.h"),
        format!("{dir}/libs/linux/arm64/liblighter-signer.so"),
        "liblighter-signer.so",
    );
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let (header, lib_path, lib_name) = (
        format!("{dir}/libs/darwin/arm64/liblighter-signer.h"),
        format!("{dir}/libs/darwin/arm64/liblighter-signer.dylib"),
        "liblighter-signer.dylib",
    );
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    let (header, lib_path, lib_name) = (
        format!("{dir}/libs/windows/amd64/liblighter-signer.h"),
        format!("{dir}/libs/windows/amd64/liblighter-signer.dll"),
        "liblighter-signer.dll",
    );

    // Copy the library to OUT_DIR so it's accessible at runtime
    let lib_dest = out_path.join(lib_name);
    fs::copy(&lib_path, &lib_dest).unwrap_or_else(|e| {
        panic!(
            "Failed to copy {} to {}: {}",
            lib_path,
            lib_dest.display(),
            e
        )
    });

    // Try to also copy to target directory next to the binary
    // This helps when the SDK is used as a dependency
    // The binary is typically in target/{profile}/, so we copy the library there
    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
        // Copy to the same directory as the binary would be
        let target_lib_path = PathBuf::from(&target_dir).join(&profile).join(lib_name);
        if let Some(parent) = target_lib_path.parent() {
            let _ = fs::create_dir_all(parent);
            let _ = fs::copy(&lib_path, &target_lib_path);
        }
    } else {
        // If CARGO_TARGET_DIR is not set, try to infer it from OUT_DIR
        // OUT_DIR is typically target/{profile}/build/{package}-{hash}/out
        // So we go up 3 levels to get to target/{profile}/
        if let Some(target_profile_dir) = out_path
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
        {
            let target_lib_path = target_profile_dir.join(lib_name);
            let _ = fs::copy(&lib_path, &target_lib_path);
        }
    }

    // tell the linker where to look for for the lib
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        println!("cargo:rustc-link-search=native={dir}/libs/linux/amd64");
        println!("cargo:rustc-link-lib=dylib=lighter-signer");
        // Set multiple rpath options:
        // 1. $ORIGIN - looks in the same directory as the binary (best for deployment)
        // 2. OUT_DIR - looks in the build output directory (works for dependencies)
        // 3. Source directory - works for direct development
        let out_dir_str = out_path.to_string_lossy();
        let source_lib_dir = format!("{dir}/libs/linux/amd64");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{out_dir_str}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{source_lib_dir}");
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        println!("cargo:rustc-link-search=native={dir}/libs/linux/arm64");
        println!("cargo:rustc-link-lib=dylib=lighter-signer");
        let out_dir_str = out_path.to_string_lossy();
        let source_lib_dir = format!("{dir}/libs/linux/arm64");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{out_dir_str}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{source_lib_dir}");
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        println!("cargo:rustc-link-search=native={dir}/libs/darwin/arm64");
        println!("cargo:rustc-link-lib=dylib=lighter-signer");
        // On macOS, use @rpath and @loader_path
        let out_dir_str = out_path.to_string_lossy();
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{out_dir_str}");
    }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        println!("cargo:rustc-link-search=native={dir}/libs/windows/amd64");
        println!("cargo:rustc-link-lib=dylib=lighter-signer");
    }

    bindgen::Builder::default()
        .header(header)
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}

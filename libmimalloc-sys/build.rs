use cmake::Config;

fn main() {
    let mut cfg = &mut Config::new("c_src/mimalloc");

    cfg = cfg.define("MI_OVERRIDE", "OFF");

    if cfg!(feature = "secure") {
        cfg = cfg.define("MI_SECURE", "OFF");
    }

    if cfg!(all(windows, target_env = "msvc")) {
        // cc::get_compiler have /nologo /MD default flags that are cmake::Config
        // defaults to. Those flags prevents mimalloc from building on windows
        // extracted from default cmake configuration on windows
        if cfg!(debug_assertions) {
            // CMAKE_C_FLAGS + CMAKE_C_FLAGS_DEBUG
            cfg = cfg.cflag("/DWIN32 /D_WINDOWS /W3 /MDd /Zi /Ob0 /Od /RTC1");
        } else {
            // CMAKE_C_FLAGS + CMAKE_C_FLAGS_RELEASE
            cfg = cfg.cflag("/DWIN32 /D_WINDOWS /W3 /MD /O2 /Ob2 /DNDEBUG");
        }
    }

    let (out_dir, out_name) = if cfg!(all(windows, target_env = "msvc")) {
        if cfg!(debug_assertions) {
            ("./build/Debug", "mimalloc-static-debug")
        } else {
            ("./build/Release", "mimalloc-static")
        }
    } else {
        if cfg!(debug_assertions) {
            ("./build", "mimalloc-debug")
        } else  {
            ("./build", "mimalloc")
        }
    };

    // Build mimalloc-static
    let mut dst = cfg.build_target("mimalloc-static").build();
    dst.push(out_dir);

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib={}", out_name);
}

use std::env;

#[cfg(windows)] const NITE2_DIR64: &str = "NITE2_LIB64";
#[cfg(windows)] const NITE2_DIR: &str = "NITE2_LIB";
#[cfg(not(windows))] const NITE2_DIR64: &str = "NITE2_REDIST64";
#[cfg(not(windows))] const NITE2_DIR: &str = "NITE2_REDIST";

fn main() {
    let nite2_dir = env::var(NITE2_DIR)
                      .or(env::var(NITE2_DIR64))
                      .expect(&format!("Required env var for dynamic libraries missing. Expected `{}` or `{}`.", NITE2_DIR, NITE2_DIR64));
    println!("cargo:rustc-link-search=native={}", nite2_dir);
    println!("cargo:rustc-link-lib=dylib=nite2");
}

fn main() {
    println!(
        "cargo:rustc-env=BUILD_VERSION={} ({})",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_HASH").unwrap_or("dev-build")
    );
}

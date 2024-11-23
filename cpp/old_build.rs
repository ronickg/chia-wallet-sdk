fn main() {
    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .flag_if_supported("-std=c++17")
        .shared_flag(true)
        .static_flag(true) // Add this line
        .cpp_link_stdlib("c++_shared")
        .compile("chia_wallet_sdk_cpp_bindings");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

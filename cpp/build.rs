fn main() {
    cxx_build::bridge("src/lib.rs")
        .std("c++11")
        .include("include") // Path to the directory containing clvm.h
        .compile("chia_wallet_sdk_cpp_bindings");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn main() {
    let occlum_include_dir = "/opt/occlum/include";
    let occlum_lib_dir = "/opt/occlum/build/lib";
    let occlum_lib = "occlum-pal";

    println!("cargo:include={}", occlum_include_dir);
    println!("cargo:rustc-link-search=native={}", occlum_lib_dir);
    println!("cargo:rustc-link-lib=dylib={}", occlum_lib);
    println!("cargo:rustc-link-lib=sgx_uae_service");
    println!("cargo:rustc-link-lib=sgx_enclave_common");
    println!("cargo:rustc-link-lib=sgx_dcap_ql");
    println!("cargo:rustc-link-lib=sgx_dcap_quoteverify");
    println!("cargo:rustc-link-lib=sgx_qe3_logic");
    println!("cargo:rustc-link-lib=sgx_pce_logic");
    println!("cargo:rustc-link-lib=sgx_urts");
}

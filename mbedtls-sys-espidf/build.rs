fn main() -> anyhow::Result<()> {
    println!("cargo::rustc-check-cfg=cfg(esp_idf_mbedtls_hardware_aes)");
    println!("cargo::rustc-check-cfg=cfg(esp_idf_mbedtls_hardware_sha)");
    println!("cargo::rustc-check-cfg=cfg(esp_idf_mbedtls_ecdh_legacy_context)");
    println!("cargo::rustc-check-cfg=cfg(esp_idf_version_full, values(any()))");
    println!("cargo::rustc-check-cfg=cfg(esp_idf_version_minor, values(any()))");

    embuild::build::CfgArgs::output_propagated("ESP_IDF")
}

use anyhow::Context;

fn dbc_codegen() {
    let dbc_path = String::from("./deltaq-xv3300.dbc");
    let dbc_contents = std::fs::read_to_string(&dbc_path)
        .context("failed to read XV3300 DBC file {dbc_path}\n")
        .unwrap();
    println!("cargo:rerun-if-changed={}", dbc_path);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let output_path = std::path::Path::new(&out_dir).join("deltaq_xv3300_can_messages.rs");
    let output_path = output_path.to_string_lossy().to_string();

    dbc_codegen::Config::builder()
        .dbc_name(&dbc_path)
        .dbc_content(&dbc_contents)
        .allow_dead_code(true) // Don't emit warnings if not all generated code is used
        //.impl_arbitrary(dbc_codegen::FeatureConfig::Gated("arbitrary")) // Optional impls.
        .impl_debug(dbc_codegen::FeatureConfig::Always) // See rustdoc for more,
        .impl_error(dbc_codegen::FeatureConfig::Gated("std"))
        //.check_ranges(dbc_codegen::FeatureConfig::Never)                // or look below for an example.
        .build()
        .write_to_file(&output_path)
        .unwrap();
}

fn eds_codegen() {
    let eds_path = String::from("./eds/CANOPEN STD v3.1.0 ELECTRONIC DATASHEET.eds");
    println!("cargo:rerun-if-changed={}", eds_path);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let output_path = std::path::Path::new(&out_dir).join("deltaq_xv3300_object_dictionary.rs");

    let eds = eds_codegen::read_eds(&eds_path).unwrap();
    eds_codegen::write(&eds, &output_path.to_string_lossy()).unwrap();
}

fn main() {
    dbc_codegen();
    eds_codegen();
}

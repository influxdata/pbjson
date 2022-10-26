//! Compiles Protocol Buffers definitions into native Rust types

use std::env;
use std::path::PathBuf;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protos");

    let proto_files = vec![
        root.join("syntax3.proto"),
        root.join("common.proto"),
        root.join("duplicate_name.proto"),
        root.join("escape.proto"),
    ];

    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");
    let mut prost_config = prost_build::Config::new();
    prost_config
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        .extern_path(".test.external", "crate")
        .bytes(&[".test"])
        .protoc_arg("--experimental_allow_proto3_optional");

    if cfg!(feature = "btree") {
        prost_config.btree_map([".test"]);
    }

    prost_config.compile_protos(&proto_files, &[root])?;

    let descriptor_set = std::fs::read(&descriptor_path)?;
    let mut builder = pbjson_build::Builder::new();
    builder
        .register_descriptors(&descriptor_set)?
        .extern_path(".test.external", "crate");

    if cfg!(feature = "ignore-unknown-fields") {
        builder.ignore_unknown_fields();
    }

    if cfg!(feature = "btree") {
        builder.btree_map([".test"]);
    }

    if cfg!(feature = "emit-fields") {
        builder.emit_fields();
    }

    if cfg!(feature = "use-integers-for-enums") {
        builder.use_integers_for_enums();
    }

    if cfg!(feature = "preserve-proto-field-names") {
        builder.preserve_proto_field_names();
    }

    builder.build(&[".test"])?;

    Ok(())
}

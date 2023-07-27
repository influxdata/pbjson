//! Compiles Protocol Buffers and FlatBuffers schema definitions into
//! native Rust types.

use std::env;
use std::path::PathBuf;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let descriptor_path = root.join("descriptors.bin");
    println!("cargo:rerun-if-changed={}", descriptor_path.display());

    let mut config = prost_build::Config::new();
    config
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .disable_comments(["."])
        .bytes([".google"])
        .skip_protoc_run();

    let std = std::env::var("CARGO_FEATURE_STD").map_or(false, |_| true);

    if !std {
        config.btree_map([".google"]);
    }

    let empty: &[&str] = &[];
    config.compile_protos(empty, empty)?;

    let descriptor_set = std::fs::read(descriptor_path)?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .exclude([
            ".google.protobuf.Duration",
            ".google.protobuf.Timestamp",
            ".google.protobuf.Value",
            ".google.protobuf.Struct",
            ".google.protobuf.ListValue",
            ".google.protobuf.NullValue",
            ".google.protobuf.BoolValue",
            ".google.protobuf.BytesValue",
            ".google.protobuf.DoubleValue",
            ".google.protobuf.FloatValue",
            ".google.protobuf.Int32Value",
            ".google.protobuf.Int64Value",
            ".google.protobuf.StringValue",
            ".google.protobuf.UInt32Value",
            ".google.protobuf.UInt64Value",
        ])
        .build(&[".google"])?;

    Ok(())
}

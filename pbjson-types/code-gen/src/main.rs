//! Compiles Protocol Buffers and FlatBuffers schema definitions into
//! native Rust types.
//!
//! This is kept as a separate binary to generate code for manual check-in, instead of running at
//! compile time as `build.rs` -- that way downstream consumers do not require the build
//! dependencies (espeically protoc) and can import rust sources directly.

use std::env;
use std::path::PathBuf;

use clap::Parser;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
struct Args {
    /// The path of the directory containing the protobuf sources.
    #[clap(short, long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/../protos"))]
    input_proto_dir: PathBuf,

    /// The destination directory for generated code.
    #[clap(short, long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/pb/"))]
    output_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let root = args.input_proto_dir;
    let out_dir = &args.output_dir;

    let proto_files = vec![root.join("google/protobuf/types.proto")];

    std::fs::create_dir_all(out_dir)?;

    let temp_dir = tempfile::tempdir()?;
    let descriptor_path = temp_dir.path().join("proto_descriptor.bin");
    prost_build::Config::new()
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .disable_comments(&["."])
        .bytes(&[".google"])
        .out_dir(out_dir)
        .compile_protos(&proto_files, &[root])?;

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
        .out_dir(out_dir)
        .build(&[".google"])?;

    Ok(())
}

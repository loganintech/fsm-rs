use std::io::Result;
fn main() -> Result<()> {
// Use this in build.rs
    let custom = protobuf_codegen::Customize::default()
        .generate_getter(true)
        .generate_accessors(true);
    protobuf_codegen::Codegen::new()
        .customize(custom)
        // Use `protoc` parser, optional.
        .protoc()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["proto"])
        // Inputs must reside in some of include paths.
        .input("proto/button.proto")
        // Specify output directory relative to Cargo output directory.
        .out_dir("src/proto")
        .run_from_script();
    Ok(())
}
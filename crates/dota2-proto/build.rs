use std::io::Result;

fn main() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(
            &[
                "proto/steammessages.proto",
                "proto/gcsdk_gcmessages.proto",
                "proto/base_gcmessages.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}

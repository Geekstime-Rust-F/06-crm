use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    let builder = tonic_build::configure();
    builder.out_dir("src/pb").compile_protos(
        &[
            "../protos/metadata/message.proto",
            "../protos/metadata/rpc.proto",
        ],
        &["../protos/metadata"],
    )?;

    Ok(())
}

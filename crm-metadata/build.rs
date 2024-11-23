use std::fs;

use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    let builder = tonic_build::configure();
    builder
        .with_type_attributes(&["MaterializeRequest"], &[r#"#[derive(Eq, Hash)]"#])
        .out_dir("src/pb")
        .compile_protos(
            &[
                "../protos/metadata/message.proto",
                "../protos/metadata/rpc.proto",
            ],
            &["../protos/metadata"],
        )?;

    Ok(())
}

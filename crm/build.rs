use std::fs;

use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    let builder = tonic_build::configure();
    builder
        .with_derive_builder(&["WelcomeRequest", "RecallRequest", "RemindRequest"], None)
        .with_field_attributes(
            &["WelcomeRequest.content_ids"],
            &[r#"#[builder(setter(each(name = "content_id", into)))]"#],
        )
        .out_dir("src/pb")
        .compile_protos(
            &["../protos/crm/message.proto", "../protos/crm/rpc.proto"],
            &["../protos/crm"],
        )?;

    Ok(())
}

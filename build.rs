use anyhow::Result;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    EmitBuilder::builder()
        .build_timestamp()
        .all_build()
        .all_git()
        .emit()?;
    Ok(())
}

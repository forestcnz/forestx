use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn forestx_command(forestx_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(forestx_utils_cargo_bin::cargo_bin("forestx")?);
    cmd.env("FORESTX_HOME", forestx_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let forestx_home = TempDir::new()?;

    forestx_command(forestx_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`forestx update` is not available in debug builds"));

    Ok(())
}

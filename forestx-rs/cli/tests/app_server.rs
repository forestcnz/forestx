use std::path::Path;

use anyhow::Result;
use app_test_support::app_server_json_shutdown_event;
use predicates::str::contains;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::TempDir;

fn forestx_command(forestx_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(forestx_utils_cargo_bin::cargo_bin("forestx")?);
    cmd.env("FORESTX_HOME", forestx_home);
    Ok(cmd)
}

#[test]
fn strict_config_rejects_unknown_config_fields_for_app_server() -> Result<()> {
    let forestx_home = TempDir::new()?;
    std::fs::write(
        forestx_home.path().join("config.toml"),
        r#"
foo = "bar"
"#,
    )?;

    let mut cmd = forestx_command(forestx_home.path())?;
    cmd.args(["app-server", "--strict-config", "--listen", "off"])
        .assert()
        .failure()
        .stderr(contains("unknown configuration field"));

    Ok(())
}

#[test]
fn app_server_emits_json_info_events() -> Result<()> {
    let forestx_home = TempDir::new()?;
    let event = app_server_json_shutdown_event("forestx", &["app-server"], forestx_home.path())?;

    assert_eq!(
        event,
        json!({
            "level": "INFO",
            "fields": {
                "message": "processor task exited",
                "exit_reason": "stdio_connection_closed",
                "remaining_connection_count": 0,
                "shutdown_forced": false,
            },
            "target": "forestx_app_server",
        })
    );

    Ok(())
}

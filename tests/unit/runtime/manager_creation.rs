use super::Manager;
use crate::runtime::config::Settings;
use crate::shell::ShellChoice;
use std::path::Path;
fn test_settings() -> Settings {
    Settings {
        daemon_service_name: format!("functerm/test/manager/{}", nanoid::nanoid!()),
        terminal_rows: 30,
        terminal_cols: 120,
        terminal_model_title: "FuncTerm".to_owned(),
        shell_startup_timeout_seconds: 10.0,
        powershell: vec!["powershell.exe".to_owned()],
        bash: vec!["bash.exe".to_owned()],
        nushell: vec!["nu.exe".to_owned()],
        zsh: vec!["zsh".to_owned()],
        cmd: vec!["cmd.exe".to_owned()],
        bun: vec!["bun".to_owned()],
        python: vec!["python".to_owned()],
        mcp: crate::runtime::config::McpSettings::default(),
    }
}
#[test]
fn missing_starting_directory_is_rejected_before_tab_creation() {
    let manager = Manager::new(test_settings()).unwrap();
    let error = manager
        .new_tab(
            Path::new("Z:\\definitely-missing-mcp-pty-cwd"),
            ShellChoice::PowerShell,
            &crate::runtime::protocol::EnvironmentSnapshot::capture(),
        )
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("starting_directory does not exist or is not a directory")
    );
}
#[test]
fn immediately_exiting_shell_is_rejected_before_registration() {
    let mut settings = test_settings();
    settings.powershell = vec![immediately_exiting_executable().to_owned()];
    let manager = Manager::new(settings).unwrap();
    let error = manager
        .new_tab(
            crate::test_fs::temp_root().as_path(),
            ShellChoice::PowerShell,
            &crate::runtime::protocol::EnvironmentSnapshot::capture(),
        )
        .unwrap_err();
    assert!(error.to_string().contains("startup"), "{error:#}");
}
#[cfg(windows)]
fn immediately_exiting_executable() -> &'static str {
    "whoami.exe"
}
#[cfg(not(windows))]
fn immediately_exiting_executable() -> &'static str {
    "false"
}

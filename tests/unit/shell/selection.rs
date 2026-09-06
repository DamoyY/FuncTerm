use super::ShellChoice;
fn assert_rejected_starting_shell(value: &str) {
    if let Ok(choice) = ShellChoice::from_canonical_name(value) {
        panic!("{value} should not parse as starting shell, got {choice:?}");
    }
}
#[test]
fn canonical_shell_names_parse_to_supported_choices() {
    assert_eq!(
        <ShellChoice as strum::VariantNames>::VARIANTS,
        ["powershell", "bash", "nu", "zsh", "cmd", "bun", "python"]
    );
    for &choice in ShellChoice::all() {
        assert_eq!(
            ShellChoice::from_canonical_name(choice.canonical_name()).unwrap(),
            choice
        );
    }
}
#[test]
fn executable_aliases_only_parse_for_shim_invocation() {
    assert_eq!(
        ShellChoice::from_shim_name("powershell.exe"),
        Some(ShellChoice::PowerShell)
    );
    assert_eq!(
        ShellChoice::from_shim_name("pwsh"),
        Some(ShellChoice::PowerShell)
    );
    assert_eq!(
        ShellChoice::from_shim_name("bash.exe"),
        Some(ShellChoice::Bash)
    );
    assert_eq!(
        ShellChoice::from_shim_name("nushell.exe"),
        Some(ShellChoice::NuShell)
    );
    assert_eq!(
        ShellChoice::from_shim_name("cmd.exe"),
        Some(ShellChoice::Cmd)
    );
    assert_eq!(
        ShellChoice::from_shim_name("bun.exe"),
        Some(ShellChoice::Bun)
    );
}
#[test]
fn starting_shell_does_not_accept_executable_aliases() {
    assert_rejected_starting_shell("pwsh");
    assert_rejected_starting_shell("powershell.exe");
    assert_rejected_starting_shell("bash.exe");
    assert_rejected_starting_shell("nushell");
    assert_rejected_starting_shell("cmd.exe");
    assert_rejected_starting_shell("bun.exe");
}

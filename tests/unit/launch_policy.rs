use super::LaunchRoute;
use crate::shell::ShellChoice;
use std::ffi::OsString;
#[test]
fn redirected_input_is_never_claimed_by_a_managed_session() {
    for &choice in ShellChoice::all() {
        for arguments in [Vec::new(), session_arguments(choice)] {
            assert_eq!(
                LaunchRoute::classify(choice, &arguments, false),
                LaunchRoute::NativeProcess,
                "{choice:?} with {arguments:?} must retain redirected stdin"
            );
        }
    }
}
#[test]
fn terminal_input_allows_supported_session_arguments() {
    for &choice in ShellChoice::all() {
        assert_eq!(
            LaunchRoute::classify(choice, &session_arguments(choice), true),
            LaunchRoute::ManagedSession,
            "{choice:?} should support terminal sessions"
        );
        let expected = if choice == ShellChoice::Bun {
            LaunchRoute::NativeProcess
        } else {
            LaunchRoute::ManagedSession
        };
        assert_eq!(
            LaunchRoute::classify(choice, &[], true),
            expected,
            "{choice:?} without arguments"
        );
    }
}
#[test]
fn program_arguments_remain_native_even_with_terminal_input() {
    for &choice in ShellChoice::all() {
        let arguments = match choice {
            ShellChoice::PowerShell => vec!["-Command", "exit 7"],
            ShellChoice::Bash | ShellChoice::Zsh => vec!["-c", "exit 7"],
            ShellChoice::NuShell => vec!["--commands", "exit 7"],
            ShellChoice::Cmd => vec!["/C", "exit 7"],
            ShellChoice::Bun => vec!["run", "script.js"],
            ShellChoice::Python => vec!["-c", "raise SystemExit(7)"],
        };
        let native = arguments
            .into_iter()
            .map(OsString::from)
            .collect::<Vec<_>>();
        for terminal_input in [true, false] {
            assert_eq!(
                LaunchRoute::classify(choice, &native, terminal_input),
                LaunchRoute::NativeProcess,
                "{choice:?} program arguments must be preserved"
            );
        }
    }
}
#[test]
fn python_stdin_script_module_and_file_arguments_are_not_repl_launches() {
    for arguments in [
        vec!["-"],
        vec!["-u", "-"],
        vec!["script.py"],
        vec!["-m", "module"],
        vec!["-i", "script.py"],
        vec!["-i", "-c", "print('code')"],
    ] {
        let native = arguments
            .into_iter()
            .map(OsString::from)
            .collect::<Vec<_>>();
        assert_eq!(
            LaunchRoute::classify(ShellChoice::Python, &native, true),
            LaunchRoute::NativeProcess,
            "Python arguments must be preserved: {native:?}"
        );
    }
}
fn session_arguments(choice: ShellChoice) -> Vec<OsString> {
    let arguments = match choice {
        ShellChoice::PowerShell => vec!["-NoLogo", "-NoProfile"],
        ShellChoice::Bash | ShellChoice::Zsh => vec!["-i"],
        ShellChoice::NuShell => vec!["--no-history"],
        ShellChoice::Cmd => vec!["/D", "/Q"],
        ShellChoice::Bun => vec!["repl"],
        ShellChoice::Python => vec!["-i", "-u", "-q"],
    };
    arguments.into_iter().map(OsString::from).collect()
}

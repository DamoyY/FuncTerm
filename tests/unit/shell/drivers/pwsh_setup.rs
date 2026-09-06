use crate::shell::ShellChoice;
use crate::shell::drivers::StartupContext;
use std::path::Path;
#[test]
fn initialization_sets_literal_location() {
    let script = super::initialization_script(StartupContext {
        cwd: Path::new("F:\\dir with ' quote"),
        startup_directory: Path::new("F:\\session\\startup"),
        ready_file: Path::new("F:\\ready'file"),
    })
    .unwrap();
    assert!(script.contains("function f"));
    assert!(script.contains("Set-Location -LiteralPath ([Text.Encoding]::UTF8.GetString"));
    assert!(script.contains("Set-Content -LiteralPath ([Text.Encoding]::UTF8.GetString"));
}
#[test]
fn invocation_is_short_dispatcher() {
    let invocation = crate::shell::drivers::invocation(ShellChoice::PowerShell)
        .unwrap()
        .unwrap();
    let bytes = invocation.into_bytes();
    assert_eq!(bytes, b"f\r");
}
#[test]
fn keyboard_input_encodes_each_line_break_as_one_enter() {
    assert_eq!(
        super::keyboard_bytes(b"exit\n").as_ref(),
        b"exit\r".as_slice()
    );
    assert_eq!(
        super::keyboard_bytes(b"exit\r\n").as_ref(),
        b"exit\r".as_slice()
    );
}

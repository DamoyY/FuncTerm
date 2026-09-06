use crate::shell::ShellChoice;
#[test]
fn invocation_uses_windows_line_ending() {
    let bytes = crate::shell::drivers::invocation(ShellChoice::Cmd)
        .unwrap()
        .unwrap()
        .into_bytes();
    assert_eq!(bytes, b"f\r\n");
}

use crate::shell::ShellChoice;
#[test]
fn invocation_uses_platform_line_ending() {
    let bytes = crate::shell::drivers::invocation(ShellChoice::NuShell)
        .unwrap()
        .unwrap()
        .into_bytes();
    #[cfg(windows)]
    assert_eq!(bytes, b"f\r\n");
    #[cfg(not(windows))]
    assert_eq!(bytes, b"f\n");
}

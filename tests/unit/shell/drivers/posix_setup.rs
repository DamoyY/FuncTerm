use crate::shell::ShellChoice;
#[test]
fn invocation_uses_line_feed() {
    for choice in [ShellChoice::Bash, ShellChoice::Zsh] {
        let bytes = crate::shell::drivers::invocation(choice)
            .unwrap()
            .unwrap()
            .into_bytes();
        assert_eq!(bytes, b"f\n");
    }
}

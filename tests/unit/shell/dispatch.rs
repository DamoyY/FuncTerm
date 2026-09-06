use super::{InvocationTerminator, ShellInvocation};
#[test]
fn invocation_rejects_embedded_line_breaks() {
    for line in ["command\nnext", "command\rnext"] {
        let result = ShellInvocation::new(line.to_owned(), InvocationTerminator::CarriageReturn);
        assert!(
            result.is_err(),
            "invocation line with a line break should be rejected"
        );
    }
}

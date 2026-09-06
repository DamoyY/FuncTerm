#[test]
fn generated_batch_uses_crlf_for_every_line() {
    let script = super::wrapper();
    assert!(script.starts_with("@echo off\r\n"));
    for line in script.split_inclusive('\n') {
        assert!(
            line.ends_with("\r\n"),
            "CMD label scanning requires CRLF: {}",
            line.escape_debug()
        );
        assert!(!line.ends_with("\r\r\n"));
    }
}

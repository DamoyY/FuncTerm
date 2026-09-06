#[test]
fn command_start_restores_model_title_before_capture_marker() {
    let directory = crate::test_fs::temp_dir("command-start-title");
    let mut output = Vec::new();
    super::write_start_to("command-a", &directory, "Model", &mut output).unwrap();
    assert_eq!(
        output,
        b"\x1b]2;Model\x1b\\\x1b]9999;FuncTerm;start;command-a\x1b\\"
    );
    std::fs::remove_dir_all(directory).unwrap();
}

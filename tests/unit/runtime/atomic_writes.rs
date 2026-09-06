use alloc::sync::Arc;
#[test]
fn concurrent_write_once_publishes_one_complete_value() {
    let directory = crate::test_fs::temp_dir("file-publish-once");
    let destination = Arc::new(directory.join("result.txt"));
    let workers = (0_u8..8)
        .map(|value| {
            let path = Arc::clone(&destination);
            std::thread::spawn(move || {
                let contents = vec![value; 4096];
                super::write_once(&path, contents).unwrap();
            })
        })
        .collect::<Vec<_>>();
    for worker in workers {
        worker.join().unwrap();
    }
    let contents = std::fs::read(destination.as_path()).unwrap();
    let first = contents.first().copied().unwrap();
    assert_eq!(contents.len(), 4096);
    assert!(contents.iter().all(|byte| *byte == first));
}
#[test]
fn write_replace_overwrites_complete_value() {
    let directory = crate::test_fs::temp_dir("file-publish-replace");
    let destination = directory.join("state.txt");
    super::write_replace(&destination, "first").unwrap();
    super::write_replace(&destination, "second").unwrap();
    assert_eq!(std::fs::read_to_string(destination).unwrap(), "second");
}

use core::time::Duration;
use std::process::Stdio;
use std::thread;
#[test]
fn startup_wait_uses_daemon_report_after_launcher_exits() {
    let path = crate::test_fs::temp_dir("startup-wait").join("ready.json");
    let mut launcher = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--help")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let report_path = path.clone();
    let report_worker = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        std::fs::create_dir_all(report_path.parent().unwrap()).unwrap();
        let text =
            sonic_rs::to_string(&crate::runtime::daemon::report::StartupReply::Ready).unwrap();
        std::fs::write(report_path, text).unwrap();
    });
    super::wait_for_startup_file(&path, &mut launcher, Duration::from_secs(2)).unwrap();
    report_worker.join().unwrap();
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}

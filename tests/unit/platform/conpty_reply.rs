use super::{StartupEvent, monitor_child};
use core::time::Duration;
use portable_pty::Child;
use std::sync::mpsc;
#[test]
fn process_monitor_reports_exit_without_polling() {
    let process = std::process::Command::new("whoami.exe")
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut child: Box<dyn Child + Send + Sync> = Box::new(process);
    let (sender, receiver) = mpsc::channel();
    monitor_child(child.as_ref(), sender).unwrap();
    let event = receiver.recv_timeout(Duration::from_secs(2)).unwrap();
    let StartupEvent::ProcessExited(result) = event else {
        panic!("expected process exit event");
    };
    result.unwrap();
    child.wait().unwrap();
}

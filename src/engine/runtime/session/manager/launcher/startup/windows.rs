use super::StartupEvent;
use anyhow::{Context as _, Result, bail};
use portable_pty::Child;
use std::os::windows::io::{AsRawHandle as _, FromRawHandle as _, OwnedHandle, RawHandle};
use std::sync::mpsc;
use std::thread;
use windows::Win32::Foundation::{
    DUPLICATE_SAME_ACCESS, DuplicateHandle, HANDLE, WAIT_FAILED, WAIT_OBJECT_0,
};
use windows::Win32::System::Threading::{GetCurrentProcess, INFINITE, WaitForSingleObject};
pub(super) fn monitor_child(
    child: &(dyn Child + Send + Sync),
    sender: mpsc::Sender<StartupEvent>,
) -> Result<()> {
    let raw_handle = child
        .as_raw_handle()
        .context("shell child has no Windows process handle")?;
    let duplicate = duplicate_handle(raw_handle)?;
    let spawn_result = thread::Builder::new()
        .name("functerm-shell-startup".to_owned())
        .spawn(move || {
            let result = wait_for_process(duplicate);
            let _sent = sender.send(StartupEvent::ProcessExited(result));
        });
    if let Err(error) = spawn_result {
        return Err(error).context("failed to start shell process monitor");
    }
    Ok(())
}
fn duplicate_handle(raw_handle: RawHandle) -> Result<OwnedHandle> {
    let current_process = unsafe { GetCurrentProcess() };
    let source_handle = HANDLE(raw_handle);
    let mut duplicate = HANDLE::default();
    unsafe {
        DuplicateHandle(
            current_process,
            source_handle,
            current_process,
            &raw mut duplicate,
            0,
            false,
            DUPLICATE_SAME_ACCESS,
        )
    }
    .context("failed to duplicate shell handle")?;
    Ok(unsafe { OwnedHandle::from_raw_handle(duplicate.0) })
}
fn wait_for_process(handle: OwnedHandle) -> Result<()> {
    let windows_handle = HANDLE(handle.as_raw_handle());
    let wait_result = unsafe { WaitForSingleObject(windows_handle, INFINITE) };
    drop(handle);
    match wait_result {
        WAIT_OBJECT_0 => Ok(()),
        WAIT_FAILED => {
            Err(std::io::Error::last_os_error()).context("failed to wait for shell process")
        }
        unexpected => bail!("unexpected shell process wait result {unexpected:?}"),
    }
}
#[cfg(test)]
#[path = "../../../../../../../tests/unit/platform/conpty_reply.rs"]
mod tests;

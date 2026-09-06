use anyhow::{Context as _, Result};
use atomicwrites::{AllowOverwrite, AtomicFile, DisallowOverwrite};
use std::io::Write as _;
use std::path::Path;
pub(crate) fn write_once(destination: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
    publish_once(destination, |file| file.write_all(contents.as_ref()))
}
pub(crate) fn copy_once(source: &Path, destination: &Path) -> Result<()> {
    if destination.exists() {
        return Ok(());
    }
    let mut source_file = fs_err::File::open(source)?;
    let permissions = source_file.metadata()?.permissions();
    publish_once(destination, |destination_file| {
        std::io::copy(&mut source_file, destination_file)?;
        destination_file.set_permissions(permissions)
    })
}
pub(crate) fn write_replace(destination: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
    prepare_parent(destination)?;
    AtomicFile::new(destination, AllowOverwrite)
        .write::<_, std::io::Error, _>(|file| file.write_all(contents.as_ref()))
        .with_context(|| {
            format!(
                "failed to atomically replace file {}",
                destination.display()
            )
        })
}
fn publish_once(
    destination: &Path,
    operation: impl FnOnce(&mut std::fs::File) -> std::io::Result<()>,
) -> Result<()> {
    prepare_parent(destination)?;
    match AtomicFile::new(destination, DisallowOverwrite).write(operation) {
        Ok(()) => Ok(()),
        Err(atomicwrites::Error::Internal(error))
            if error.kind() == std::io::ErrorKind::AlreadyExists =>
        {
            Ok(())
        }
        Err(error) => Err(error).with_context(|| {
            format!(
                "failed to atomically publish file {}",
                destination.display()
            )
        }),
    }
}
fn prepare_parent(destination: &Path) -> Result<()> {
    let parent = destination
        .parent()
        .context("published file has no parent")?;
    fs_err::create_dir_all(parent).map_err(Into::into)
}
#[cfg(test)]
#[path = "../../tests/unit/runtime/atomic_writes.rs"]
mod tests;

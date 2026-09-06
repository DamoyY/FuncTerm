#[cfg(windows)]
#[test]
fn windows_detached_flags_break_away_from_parent_job() {
    use windows::Win32::System::Threading::CREATE_BREAKAWAY_FROM_JOB;
    let flags = super::windows_creation_flags_for_job(super::JobState::AllowsBreakaway);
    assert_ne!(flags & CREATE_BREAKAWAY_FROM_JOB.0, 0);
}
#[cfg(windows)]
#[test]
fn windows_detached_flags_avoid_forbidden_breakaway() {
    use windows::Win32::System::Threading::CREATE_BREAKAWAY_FROM_JOB;
    let flags = super::windows_creation_flags_for_job(super::JobState::ForbidsBreakaway);
    assert_eq!(flags & CREATE_BREAKAWAY_FROM_JOB.0, 0);
}
#[cfg(windows)]
#[test]
fn windows_detached_flags_do_not_break_away_outside_jobs() {
    use windows::Win32::System::Threading::CREATE_BREAKAWAY_FROM_JOB;
    let flags = super::windows_creation_flags_for_job(super::JobState::NotInJob);
    assert_eq!(flags & CREATE_BREAKAWAY_FROM_JOB.0, 0);
}
#[cfg(windows)]
#[test]
fn windows_detached_flags_preserve_ctrl_c_for_descendants() {
    use windows::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;
    for job in [
        super::JobState::NotInJob,
        super::JobState::AllowsBreakaway,
        super::JobState::ForbidsBreakaway,
        super::JobState::Unknown,
    ] {
        let flags = super::windows_creation_flags_for_job(job);
        assert_eq!(flags & CREATE_NEW_PROCESS_GROUP.0, 0);
    }
}

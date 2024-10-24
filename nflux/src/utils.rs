use libc::getuid;

// Check if the current user ID is 0 (root)
pub fn is_root_user() -> bool {
    unsafe { getuid() == 0 }
}
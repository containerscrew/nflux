use libc::getuid;

pub fn is_root_user() -> bool {
    // Check if the current user ID is 0 (root)
    unsafe { getuid() == 0 }
}
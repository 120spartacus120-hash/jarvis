use std::path::{Path, PathBuf};

/// Native libraries on Windows (Vosk, etc.) often fail on non-ASCII paths.
#[cfg(windows)]
pub fn native_library_path(path: &Path) -> PathBuf {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};

    extern "system" {
        fn GetShortPathNameW(lpszLongPath: *const u16, lpszShortPath: *mut u16, cchBuffer: u32) -> u32;
    }

    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut buffer = vec![0u16; 1024];

    let len = unsafe {
        GetShortPathNameW(wide.as_ptr(), buffer.as_mut_ptr(), buffer.len() as u32)
    };

    if len == 0 || len as usize >= buffer.len() {
        return path.to_path_buf();
    }

    buffer.truncate(len as usize);
    PathBuf::from(OsString::from_wide(&buffer))
}

#[cfg(not(windows))]
pub fn native_library_path(path: &Path) -> PathBuf {
    path.to_path_buf()
}

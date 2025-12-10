use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM, BOOL},
        UI::WindowsAndMessaging::{
            FindWindowW, SendMessageW, SetParent, EnumWindows, GetWindow, FindWindowExW, 
            GW_HWNDNEXT
        },
    },
};
use crate::core::AppError;
use log::{debug, error, info, warn};

/// Find the WorkerW window that we can parent wallpaper windows to
pub fn find_workerw() -> std::result::Result<HWND, AppError> {
    unsafe {
        // Find the Program Manager window
        let progman = FindWindowW(w!("Progman"), None);
        if progman == HWND(0) {
            return Err(AppError::WallpaperError("Failed to find Progman window".to_string()));
        }

        debug!("Found Progman window: {:?}", progman);

        // Send message to spawn WorkerW behind desktop icons
        // This is the magic message that creates the WorkerW window
        let result = SendMessageW(progman, 0x052C, WPARAM(0), LPARAM(0));
        debug!("Sent spawn message to Progman, result: {:?}", result);

        // Give Windows a moment to create the WorkerW window
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Try multiple methods to find WorkerW
        
        // Method 1: Look for WorkerW with SHELLDLL_DefView child
        let mut workerw = None;
        let result = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut workerw as *mut _ as isize),
        );

        if result.is_ok() && workerw.is_some() {
            let hwnd = workerw.unwrap();
            info!("Found WorkerW window (method 1): {:?}", hwnd);
            return Ok(hwnd);
        }

        // Method 2: Fallback - try to find any WorkerW window
        let workerw_fallback = FindWindowW(w!("WorkerW"), None);
        if workerw_fallback != HWND(0) {
            info!("Found WorkerW window (method 2 - fallback): {:?}", workerw_fallback);
            return Ok(workerw_fallback);
        }

        // Method 3: Last resort - use Progman itself
        warn!("Could not find WorkerW, falling back to Progman window");
        Ok(progman)
    }
}

/// Window enumeration callback to find WorkerW
extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let workerw_ptr = lparam.0 as *mut Option<HWND>;

        // Look for WorkerW window that has SHELLDLL_DefView as a child
        let shelldll = FindWindowExW(hwnd, None, w!("SHELLDLL_DefView"), None);
        if shelldll != HWND(0) {
            // Found a window with SHELLDLL_DefView as child
            // Get the next window after this one - that should be our target WorkerW
            let next = GetWindow(hwnd, GW_HWNDNEXT);
            if next != HWND(0) {
                debug!("Found WorkerW candidate: {:?}", next);
                *workerw_ptr = Some(next);
                return BOOL(0); // Stop enumeration
            }
        }

        BOOL(1) // Continue enumeration
    }
}

/// Parent a window to the desktop (behind desktop icons)
pub fn parent_to_desktop(window_hwnd: HWND) -> std::result::Result<(), AppError> {
    unsafe {
        // Find WorkerW
        let workerw = find_workerw()?;

        // Parent our window to WorkerW
        let result = SetParent(window_hwnd, workerw);
        if result == HWND(0) {
            return Err(AppError::WallpaperError("Failed to parent window to WorkerW".to_string()));
        }

        info!("Successfully parented window {:?} to WorkerW {:?}", window_hwnd, workerw);
        Ok(())
    }
}

/// Check if we can access the desktop integration features
pub fn check_desktop_integration() -> bool {
    match find_workerw() {
        Ok(_) => {
            debug!("Desktop integration is available");
            true
        }
        Err(e) => {
            error!("Desktop integration not available: {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_workerw() {
        // This test will only work on Windows with a desktop environment
        if cfg!(windows) {
            match find_workerw() {
                Ok(hwnd) => {
                    println!("Found WorkerW: {:?}", hwnd);
                    assert_ne!(hwnd, HWND(0));
                }
                Err(e) => {
                    println!("Could not find WorkerW (this is normal in some environments): {}", e);
                }
            }
        }
    }

    #[test]
    fn test_check_desktop_integration() {
        // This should not panic
        let available = check_desktop_integration();
        println!("Desktop integration available: {}", available);
    }
}
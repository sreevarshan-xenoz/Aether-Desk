use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM, RECT, LRESULT},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DestroyWindow, ShowWindow, GetSystemMetrics,
            SetWindowPos, GetWindowRect, RegisterClassExW, WNDCLASSEXW,
            LoadCursorW, DefWindowProcW, PostQuitMessage,
            WINDOW_EX_STYLE, WS_POPUP, SW_SHOW, SW_HIDE, WM_DESTROY,
            SM_CXSCREEN, SM_CYSCREEN, SWP_NOACTIVATE, SWP_NOZORDER,
            CS_HREDRAW, CS_VREDRAW, IDC_ARROW
        },
        System::LibraryLoader::GetModuleHandleW,
    },
};
use crate::core::AppError;
use crate::platform::windows::desktop::find_workerw;
use log::{debug, info};
use std::ptr;

/// Window manager for creating and managing wallpaper windows
pub struct WindowManager {
    /// The wallpaper window handle
    window: Option<HWND>,
    /// The WorkerW window handle (desktop background)
    workerw: Option<HWND>,
    /// Window class name
    class_name: String,
}

impl WindowManager {
    /// Create a new window manager
    pub fn new() -> Self {
        Self {
            window: None,
            workerw: None,
            class_name: "AetherDeskWallpaper".to_string(),
        }
    }

    /// Register the window class for wallpaper windows
    fn register_window_class(&self) -> std::result::Result<(), AppError> {
        let class_name_wide: Vec<u16> = self.class_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        unsafe {
            let hinstance = match GetModuleHandleW(None) {
                Ok(h) => h,
                Err(e) => return Err(AppError::WallpaperError(format!("Failed to get module handle: {}", e))),
            };

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance.into(),
                hIcon: Default::default(),
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
                hbrBackground: Default::default(),
                lpszMenuName: PCWSTR::null(),
                lpszClassName: PCWSTR(class_name_wide.as_ptr()),
                hIconSm: Default::default(),
            };

            let atom = RegisterClassExW(&wc);
            if atom == 0 {
                return Err(AppError::WallpaperError("Failed to register window class".to_string()));
            }
        }

        debug!("Window class registered successfully");
        Ok(())
    }

    /// Window procedure for handling window messages
    extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, WM_DESTROY, PostQuitMessage};

        unsafe {
            match msg {
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }

    /// Create a wallpaper window that will be parented to the desktop
    pub fn create_wallpaper_window(&mut self) -> std::result::Result<HWND, AppError> {
        // First, find the WorkerW window
        self.workerw = Some(match find_workerw() {
            Ok(w) => w,
            Err(e) => return Err(e),
        });
        debug!("Found WorkerW window: {:?}", self.workerw);

        // Register window class if not already done
        self.register_window_class().or_else(|_| {
            // Class might already be registered, which is fine
            debug!("Window class already registered or registration failed (continuing anyway)");
            Ok::<(), AppError>(())
        })?;

        // Get screen dimensions
        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

        debug!("Screen dimensions: {}x{}", screen_width, screen_height);

        // Create the window
        let class_name_wide: Vec<u16> = self.class_name.encode_utf16().chain(std::iter::once(0)).collect();
        let window_title_wide: Vec<u16> = "Aether-Desk Wallpaper".encode_utf16().chain(std::iter::once(0)).collect();

        let window = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),                    // Extended styles
                PCWSTR(class_name_wide.as_ptr()),      // Window class
                PCWSTR(window_title_wide.as_ptr()),    // Window title
                WS_POPUP,                              // Window style (borderless)
                0, 0,                                  // Position
                screen_width, screen_height,           // Size
                None,                                  // Parent window (will be set later)
                None,                                  // Menu
                GetModuleHandleW(None).unwrap_or_default(), // Instance
                Some(ptr::null_mut()),                 // Additional data
            )
        };

        if window == HWND(0) {
            return Err(AppError::WallpaperError("Failed to create wallpaper window".to_string()));
        }

        debug!("Created wallpaper window: {:?}", window);

        // Parent the window to WorkerW
        self.parent_to_desktop(window)?;

        self.window = Some(window);
        info!("Wallpaper window created and parented to desktop");

        Ok(window)
    }

    /// Parent a window to the desktop (behind desktop icons)
    fn parent_to_desktop(&self, window: HWND) -> std::result::Result<(), AppError> {
        use windows::Win32::UI::WindowsAndMessaging::SetParent;

        if let Some(workerw) = self.workerw {
            unsafe {
                let result = SetParent(window, workerw);
                if result == HWND(0) {
                    return Err(AppError::WallpaperError("Failed to parent window to WorkerW".to_string()));
                }
            }
            debug!("Successfully parented window {:?} to WorkerW {:?}", window, workerw);
        } else {
            return Err(AppError::WallpaperError("WorkerW not found".to_string()));
        }

        Ok(())
    }

    /// Get the window handle
    pub fn get_window(&self) -> Option<HWND> {
        self.window
    }

    /// Get the WorkerW handle
    pub fn get_workerw(&self) -> Option<HWND> {
        self.workerw
    }

    /// Show the wallpaper window
    pub fn show_window(&self) -> std::result::Result<(), AppError> {
        if let Some(window) = self.window {
            unsafe {
                ShowWindow(window, SW_SHOW);
            }
            debug!("Wallpaper window shown");
        }
        Ok(())
    }

    /// Hide the wallpaper window
    pub fn hide_window(&self) -> std::result::Result<(), AppError> {
        if let Some(window) = self.window {
            unsafe {
                ShowWindow(window, SW_HIDE);
            }
            debug!("Wallpaper window hidden");
        }
        Ok(())
    }

    /// Resize the wallpaper window
    pub fn resize_window(&self, width: i32, height: i32) -> std::result::Result<(), AppError> {
        if let Some(window) = self.window {
            unsafe {
                match SetWindowPos(
                    window,
                    None,
                    0, 0,
                    width, height,
                    SWP_NOACTIVATE | SWP_NOZORDER,
                ) {
                    Ok(_) => debug!("Wallpaper window resized to {}x{}", width, height),
                    Err(e) => return Err(AppError::WallpaperError(format!("Failed to resize window: {}", e))),
                }
            }
        }
        Ok(())
    }

    /// Get the window dimensions
    pub fn get_window_rect(&self) -> std::result::Result<RECT, AppError> {
        if let Some(window) = self.window {
            let mut rect = RECT::default();
            unsafe {
                match GetWindowRect(window, &mut rect) {
                    Ok(_) => Ok(rect),
                    Err(e) => Err(AppError::WallpaperError(format!("Failed to get window rect: {}", e))),
                }
            }
        } else {
            Err(AppError::WallpaperError("No window created".to_string()))
        }
    }

    /// Check if the window is valid and still exists
    pub fn is_window_valid(&self) -> bool {
        if let Some(window) = self.window {
            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::IsWindow;
                IsWindow(window).as_bool()
            }
        } else {
            false
        }
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        if let Some(window) = self.window {
            unsafe {
                DestroyWindow(window);
            }
            debug!("Wallpaper window destroyed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_manager_creation() {
        let wm = WindowManager::new();
        assert!(wm.window.is_none());
        assert!(wm.workerw.is_none());
    }

    #[test]
    fn test_window_creation() {
        // This test will only work on Windows with a desktop environment
        if cfg!(windows) {
            let mut wm = WindowManager::new();
            match wm.create_wallpaper_window() {
                Ok(hwnd) => {
                    println!("Created window: {:?}", hwnd);
                    assert_ne!(hwnd, HWND(0));
                    assert!(wm.is_window_valid());
                }
                Err(e) => {
                    println!("Could not create window (this is normal in some environments): {}", e);
                }
            }
        }
    }
}
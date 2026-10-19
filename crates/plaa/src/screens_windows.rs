//! The screens and windows plaa offers to record on Windows, where no
//! picker of the system's hands them over: the screens in the order
//! Windows lists them, which the capture numbers them by, and the windows
//! Alt+Tab shows.

#![allow(unsafe_code)]

use windows_sys::Win32::Foundation::{HWND, LPARAM, RECT};
use windows_sys::Win32::Graphics::Dwm::{DWMWA_CLOAKED, DwmGetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GW_OWNER, GWL_EXSTYLE, GetWindow, GetWindowLongPtrW, GetWindowTextLengthW,
    GetWindowTextW, GetWindowThreadProcessId, IsIconic, IsWindowVisible, MONITORINFOF_PRIMARY,
    WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
};
use windows_sys::core::BOOL;

use crate::recorder::Offered;

/// The screens, the main one first, then the windows of other apps.
pub fn offered() -> Vec<Offered> {
    let mut offered = screens();
    offered.extend(windows());
    offered
}

/// Each screen, the main one first, by its number.
fn screens() -> Vec<Offered> {
    unsafe extern "system" fn each(monitor: HMONITOR, _: HDC, _: *mut RECT, found: LPARAM) -> BOOL {
        // SAFETY: `found` is the vector `screens` passed, alive for the call.
        let found = unsafe { &mut *(found as *mut Vec<bool>) };
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            rcWork: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            dwFlags: 0,
        };
        // SAFETY: `info` is a MONITORINFO with its size set.
        let main = unsafe { GetMonitorInfoW(monitor, &mut info) } != 0
            && info.dwFlags & MONITORINFOF_PRIMARY != 0;
        found.push(main);
        1
    }
    let mut found: Vec<bool> = Vec::new();
    // SAFETY: the callback only touches `found`, which outlives the call.
    unsafe {
        EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null(),
            Some(each),
            &mut found as *mut Vec<bool> as LPARAM,
        );
    }
    let several = found.len() > 1;
    let mut screens: Vec<(bool, Offered)> = found
        .into_iter()
        .enumerate()
        .map(|(number, main)| {
            let shown = number + 1;
            let name = match (several, main) {
                (false, _) => crate::fl!("record-screen-whole"),
                (true, true) => crate::fl!("record-screen-main", number = shown),
                (true, false) => crate::fl!("record-screen-number", number = shown),
            };
            let offered = Offered {
                id: number as u64,
                window: false,
                name,
            };
            (main, offered)
        })
        .collect();
    // The main screen first, as the one to start with; numbers stay.
    screens.sort_by_key(|(main, _)| !main);
    screens.into_iter().map(|(_, offered)| offered).collect()
}

/// The windows Alt+Tab would show, but plaa's own and minimized ones,
/// which have no picture to record.
fn windows() -> Vec<Offered> {
    unsafe extern "system" fn each(window: HWND, found: LPARAM) -> BOOL {
        // SAFETY: `found` is the vector `windows` passed, alive for the call.
        let found = unsafe { &mut *(found as *mut Vec<Offered>) };
        // SAFETY: plain queries of a window handle Windows just gave.
        unsafe {
            if IsWindowVisible(window) == 0 || IsIconic(window) != 0 {
                return 1;
            }
            let style = GetWindowLongPtrW(window, GWL_EXSTYLE) as u32;
            let owned = !GetWindow(window, GW_OWNER).is_null();
            if style & WS_EX_TOOLWINDOW != 0 || (owned && style & WS_EX_APPWINDOW == 0) {
                return 1;
            }
            let mut cloaked: u32 = 0;
            let cloaked_read = DwmGetWindowAttribute(
                window,
                DWMWA_CLOAKED as u32,
                (&mut cloaked as *mut u32).cast(),
                size_of::<u32>() as u32,
            );
            if cloaked_read == 0 && cloaked != 0 {
                return 1;
            }
            let mut process = 0;
            GetWindowThreadProcessId(window, &mut process);
            if process == std::process::id() {
                return 1;
            }
            let length = GetWindowTextLengthW(window);
            if length <= 0 {
                return 1;
            }
            let mut title = vec![0u16; length as usize + 1];
            let read = GetWindowTextW(window, title.as_mut_ptr(), title.len() as i32);
            let title = String::from_utf16_lossy(&title[..read.max(0) as usize]);
            if !title.trim().is_empty() {
                found.push(Offered {
                    id: window as u64,
                    window: true,
                    name: title,
                });
            }
        }
        1
    }
    let mut found: Vec<Offered> = Vec::new();
    // SAFETY: the callback only touches `found`, which outlives the call.
    unsafe {
        EnumWindows(Some(each), &mut found as *mut Vec<Offered> as LPARAM);
    }
    found
}

use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetWindowLongW, SetWindowLongW,
    SetLayeredWindowAttributes, GWL_EXSTYLE,
    WS_EX_LAYERED, LWA_ALPHA,
};
use windows::Win32::Foundation::{HWND, COLORREF};
use windows::core::w;

pub fn set_taskbar_opacity(alpha: u8) {
    unsafe {
        let hwnd: HWND = FindWindowW(w!("Shell_TrayWnd"), None);

        if hwnd.0 == 0 {
            return;
        }

        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

        if ex_style & WS_EX_LAYERED.0 as i32 == 0 {
            SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32);
        }

        SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA);
    }
}
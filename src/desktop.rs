use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetClassNameW};

pub fn is_desktop() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return false;
        }

        let mut class_name = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_name);

        if len == 0 {
            return false;
        }

        let name = String::from_utf16_lossy(&class_name[..len as usize]);

        name == "Progman" || name == "WorkerW"
    }
}
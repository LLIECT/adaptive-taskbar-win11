use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, FindWindowExW, GetWindowLongW, SetWindowLongW,
    SetLayeredWindowAttributes, GWL_EXSTYLE,
    WS_EX_LAYERED, LWA_ALPHA,
};
use windows::Win32::Foundation::{HWND, COLORREF};
use windows::core::w;

pub fn set_desktop_opacity(alpha: u8) {
    unsafe {
        // Шаг 1: Находим главное окно менеджера программ (Рабочий стол)
        let mut hwnd: HWND = FindWindowW(w!("Progman"), None);

        if hwnd.0 == 0 {
            return;
        }

        // Шаг 2: Находим дочернее окно, где непосредственно находятся ярлыки
        let shell_view = FindWindowExW(hwnd, HWND(0), w!("SHELLDLL_DefView"), None);
        if shell_view.0 != 0 {
            hwnd = shell_view;
        }

        // Шаг 3: Получаем текущие стили окна
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

        // Шаг 4: Применяем флаг многослойного окна для поддержки прозрачности
        if ex_style & WS_EX_LAYERED.0 as i32 == 0 {
            SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32);
        }

        // Шаг 5: Устанавливаем уровень прозрачности альфа (0-255)
        SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA);
    }
}

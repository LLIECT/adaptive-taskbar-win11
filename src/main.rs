mod is_desktop;
mod taskbar;
mod desktop;

use std::{thread, time::Duration};

fn main() {
    let mut last_state = false;
    loop {
        let is_desktop = is_desktop::is_desktop();
        if is_desktop != last_state {
            if is_desktop {
                // рабочий стол
                taskbar::set_taskbar_opacity(0);
                desktop::set_desktop_opacity(0);    // при значении 0 появляется баг(фича): при нажатии лкм иконки и панель появляются 
                println!("Desktop → transparent");        // если поставить 1(делает их минимально прозрачными(незаметно)): можно двигать иконки 
            }                                             // но баг с лкм пропадает
            else {
                // окно
                taskbar::set_taskbar_opacity(255);
                desktop::set_desktop_opacity(255);
                println!("App → visible");
            }

            last_state = is_desktop;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
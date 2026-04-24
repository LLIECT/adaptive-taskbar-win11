mod desktop;
mod taskbar;

use std::{thread, time::Duration};

fn main() {
    let mut last_state = false;

    loop {
        let is_desktop = desktop::is_desktop();

        if is_desktop != last_state {
            if is_desktop {
                // рабочий стол
                taskbar::set_taskbar_opacity(0);
                println!("Desktop → transparent");
            } else {
                // окно
                taskbar::set_taskbar_opacity(255);
                println!("App → visible");
            }

            last_state = is_desktop;
        }

        thread::sleep(Duration::from_millis(300));
    }
}
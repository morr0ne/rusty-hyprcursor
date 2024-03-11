use std::{ffi::CString, fs::File};

use hyprcursor::HyprCursorManager;

fn main() {
    let manager = HyprCursorManager::new(None);
    let style_info = manager.new_style_info(200);

    manager.load_theme_style(&style_info);

    if manager.is_theme_valid() {
        let data = manager.get_cursor_image_data(&CString::new("left_ptr").unwrap(), &style_info);

        data[0]
            .surface
            .write_to_png(&mut File::create("arrowC.png").expect("Failed to open image"))
            .expect("Cairo failed to write png");
    } else {
        println!("No valid theme found");
    }
}

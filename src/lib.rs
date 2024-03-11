use std::{
    ffi::CStr,
    ptr::{null, NonNull},
    slice,
};

use libhyprcursor_sys::{
    hyprcursor_cursor_style_info, hyprcursor_get_cursor_image_data, hyprcursor_load_theme_style,
    hyprcursor_manager_create, hyprcursor_manager_free, hyprcursor_manager_t,
    hyprcursor_manager_valid,
};

mod cursor;

pub use cursor::{CursorImageData, CursorStyleInfo};

pub struct HyprCursorManager {
    inner: NonNull<hyprcursor_manager_t>,
}

impl HyprCursorManager {
    pub fn new(theme_name: Option<&CStr>) -> Self {
        unsafe {
            let theme_name = match theme_name {
                Some(theme_name) => theme_name.as_ptr(),
                None => null(),
            };

            let manager = hyprcursor_manager_create(theme_name);

            Self {
                inner: NonNull::new_unchecked(manager),
            }
        }
    }

    pub fn new_style_info(&self, size: u32) -> CursorStyleInfo {
        CursorStyleInfo {
            manager: self.inner,
            inner: hyprcursor_cursor_style_info { size },
        }
    }

    pub fn is_theme_valid(&self) -> bool {
        unsafe { hyprcursor_manager_valid(self.inner.as_ptr()) != 0 }
    }

    pub fn load_theme_style(&self, style_info: &CursorStyleInfo) -> bool {
        unsafe { hyprcursor_load_theme_style(self.inner.as_ptr(), style_info.inner) != 0 }
    }

    pub fn get_cursor_image_data(
        &self,
        shape: &CStr,
        style_info: &CursorStyleInfo,
    ) -> &mut [CursorImageData] {
        let mut len: i32 = 0;

        let image_data = unsafe {
            hyprcursor_get_cursor_image_data(
                self.inner.as_ptr(),
                shape.as_ptr(),
                style_info.inner,
                &mut len,
            )
            .cast::<CursorImageData>()
        };

        let image_data = unsafe { slice::from_raw_parts_mut(image_data, len as usize) };

        image_data
    }
}

impl Drop for HyprCursorManager {
    fn drop(&mut self) {
        unsafe { hyprcursor_manager_free(self.inner.as_ptr()) }
    }
}

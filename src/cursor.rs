use std::ptr::NonNull;

use cairo::Surface;
use libhyprcursor_sys::{
    hyprcursor_cursor_style_info, hyprcursor_manager_t, hyprcursor_style_done, SCursorImageData,
};

#[derive(Debug, Clone)]
pub struct CursorImageData {
    pub surface: Surface,
    pub size: i32,
    pub delay: i32,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
}

impl CursorImageData {
    pub unsafe fn from_raw(raw: *mut SCursorImageData) -> Self {
        Self {
            surface: Surface::from_raw_none((*raw).surface),
            size: (*raw).size,
            delay: (*raw).delay,
            hotspot_x: (*raw).hotspotX,
            hotspot_y: (*raw).hotspotY,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CursorStyleInfo {
    pub(crate) manager: NonNull<hyprcursor_manager_t>,
    pub(crate) inner: hyprcursor_cursor_style_info,
}

impl Drop for CursorStyleInfo {
    fn drop(&mut self) {
        unsafe { hyprcursor_style_done(self.manager.as_ptr(), self.inner) }
    }
}

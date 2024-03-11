use std::ptr::NonNull;

use cairo::Surface;
use libhyprcursor_sys::{
    hyprcursor_cursor_style_info, hyprcursor_manager_t, hyprcursor_style_done, SCursorImageData,
};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct CursorImageData {
    inner: NonNull<SCursorImageData>,
}

impl CursorImageData {
    pub unsafe fn from_raw(raw: *mut SCursorImageData) -> Self {
        Self {
            inner: NonNull::new_unchecked(raw),
        }
    }

    pub fn surface(&self) -> Surface {
        unsafe { Surface::from_raw_none(self.inner.as_ref().surface) }
    }

    pub fn size(&self) -> i32 {
        unsafe { self.inner.as_ref().size }
    }

    pub fn delay(&self) -> i32 {
        unsafe { self.inner.as_ref().delay }
    }

    pub fn hotspot_x(&self) -> i32 {
        unsafe { self.inner.as_ref().hotspotX }
    }

    pub fn hotspot_y(&self) -> i32 {
        unsafe { self.inner.as_ref().hotspotY }
    }
}

impl Drop for CursorImageData {
    fn drop(&mut self) {
        // FIXME
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

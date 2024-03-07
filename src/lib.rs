use std::ptr::{null, NonNull};

use libhyprcursor_sys::{
    hyprcursor_manager_create, hyprcursor_manager_free, hyprcursor_manager_t,
    hyprcursor_manager_valid,
};

pub struct HyprCursorManager {
    inner: NonNull<hyprcursor_manager_t>,
}

impl HyprCursorManager {
    pub fn new(theme_name: Option<&str>) -> Option<Self> {
        unsafe {
            // TODO: pass actual theme
            let manager = hyprcursor_manager_create(null());
            if hyprcursor_manager_valid(manager) != 0 {
                Some(Self {
                    inner: NonNull::new_unchecked(manager),
                })
            } else {
                None
            }
        }
    }

    pub fn load_theme_style() {
        todo!()
    }
}

impl Drop for HyprCursorManager {
    fn drop(&mut self) {
        unsafe { hyprcursor_manager_free(self.inner.as_ptr()) }
    }
}

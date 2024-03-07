#![allow(nonstandard_style)]

use cairo_sys::cairo_surface_t;

#[doc = "struct for a single cursor image"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCursorImageData {
    pub surface: *mut cairo_surface_t,
    pub size: ::core::ffi::c_int,
    pub delay: ::core::ffi::c_int,
    pub hotspotX: ::core::ffi::c_int,
    pub hotspotY: ::core::ffi::c_int,
}
#[doc = "struct for a single cursor image"]
pub type hyprcursor_cursor_image_data = SCursorImageData;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hyprcursor_manager_t {
    _unused: [u8; 0],
}
#[doc = "Simple struct for styles"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hyprcursor_cursor_style_info {
    #[doc = "Shape size.\n0 means \"any\" or \"unspecified\"."]
    pub size: ::core::ffi::c_uint,
}
extern "C" {
    #[doc = "Basic Hyprcursor manager.\n\nHas to be created for either a specified theme, or\nnullptr if you want to use a default from the env.\n\nIf no env is set, picks the first found.\n\nIf none found, hyprcursor_manager_valid will be false.\n\nIf loading fails, hyprcursor_manager_valid will be false.\n\nThe caller gets the ownership, call hyprcursor_manager_free to free this object."]
    pub fn hyprcursor_manager_create(
        theme_name: *const ::core::ffi::c_char,
    ) -> *mut hyprcursor_manager_t;
}
extern "C" {
    #[doc = "Free a hyprcursor_manager_t*"]
    pub fn hyprcursor_manager_free(manager: *mut hyprcursor_manager_t);
}
extern "C" {
    #[doc = "Returns true if the theme was successfully loaded,\ni.e. everything is A-OK and nothing should fail."]
    pub fn hyprcursor_manager_valid(manager: *mut hyprcursor_manager_t) -> ::core::ffi::c_int;
}
extern "C" {
    #[doc = "Loads a theme at a given style, synchronously.\n\nReturns whether it succeeded."]
    pub fn hyprcursor_load_theme_style(
        manager: *mut hyprcursor_manager_t,
        info: hyprcursor_cursor_style_info,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    #[doc = "Returns a hyprcursor_cursor_image_data*[] for a given cursor\nshape and size.\n\nThe entire array needs to be freed instantly after using, see hyprcursor_cursor_image_data_free()\n\nSurfaces stay valid.\n\nOnce done with a size, call hyprcursor_style_done()"]
    pub fn hyprcursor_get_cursor_image_data(
        manager: *mut hyprcursor_manager_t,
        shape: *const ::core::ffi::c_char,
        info: hyprcursor_cursor_style_info,
        out_size: *mut ::core::ffi::c_int,
    ) -> *mut *mut hyprcursor_cursor_image_data;
}
extern "C" {
    #[doc = "Free a returned hyprcursor_cursor_image_data."]
    pub fn hyprcursor_cursor_image_data_free(
        data: *mut *mut hyprcursor_cursor_image_data,
        size: ::core::ffi::c_int,
    );
}
extern "C" {
    #[doc = "Marks a certain style as done, allowing it to be potentially freed"]
    pub fn hyprcursor_style_done(
        manager: *mut hyprcursor_manager_t,
        info: hyprcursor_cursor_style_info,
    );
}

extern crate x11_dl;

use std::os::raw::{c_int, c_char, c_ulong};
use std::ffi::CString;
use std::ptr;
use std::mem;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct Timestamp(c_ulong);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270
}

impl From<c_int> for Rotation {
    fn from(f: c_int) -> Rotation {
        match c_int {
            x11_dl::xrandr::RR__Rotate_0 => Rotate0,
            x11_dl::xrandr::RR__Rotate_90 => Rotate90,
            x11_dl::xrandr::RR__Rotate_180 => Rotate180,
            x11_dl::xrandr::RR__Rotate_270 => Rotate270,
        }
    }
}

#[derive(Debug)]
pub struct Crtc {
    crtc: String,
    changing: bool,
    crtc_info: XRRCrtcInfo,
    mode_info: XRRModeInfo,
    panning_info: XRRPanningInfo,
    x: int,
    y: int,
    rotation: Rotation,
    outputs: *mut *mut output_t,
    noutput: int,
    current_transform: transform_t,
    pending_transform: transform_t,
}

/// Contains references to xlib objects, all access is provided through methods
pub struct Xrandr {
    xlib: x11_dl::xlib::Xlib,
    xrandr: x11_dl::xrandr::Xrandr,
    display: *mut x11_dl::xlib::Display,
    screen: c_int,
    version_cache: (u32, u32),
    event_base: c_int,
    error_base: c_int,
    crtcs: Vec<Crtc>
}

impl Xrandr {
    unsafe fn open(display_name: *const c_char) -> Xrandr {
        let xlib = x11_dl::xlib::Xlib::open().unwrap();
        let xrandr = x11_dl::xrandr::Xrandr::open().unwrap();
        let display = (xlib.XOpenDisplay)(display_name);
        if display.is_null() {
            panic!("Couldn't load display");
        };
        let screen = (xlib.XDefaultScreen)(display);

        let mut version: (c_int, c_int) = (mem::uninitialized(), mem::uninitialized());
        let mut evt_base: c_int = mem::uninitialized();
        let mut err_base: c_int = mem::uninitialized();
        if (xrandr.XRRQueryVersion)(display, &mut (version.0), &mut (version.1)) == 0 {
            panic!("Xrandr plugin not present");
        }
        if (xrandr.XRRQueryExtension)(display, &mut evt_base, &mut err_base) == 0 {
            panic!("Xrandr plugin not present");
        }

        Xrandr {
            xlib: xlib,
            xrandr: xrandr,
            display: display,
            screen: screen,
            version_cache: (version.0 as u32, version.1 as u32),
            event_base: evt_base,
            error_base: err_base,
        }
    }

    /// Crate a new xrandr instance with the default display (env var DISPLAY)
    pub fn new() -> Xrandr {
        unsafe {
            Xrandr::open(ptr::null())
        }
    }

    /// Create a new xrandr instance with a specific display
    pub fn with_display(display_name: &str) -> Xrandr {
        let display_name = CString::new(display_name).unwrap();
        unsafe {
            Xrandr::open(display_name.as_ptr())
        }
    }

    /// The xrandr plugin version (major, minor)
    pub fn version(&self) -> (u32, u32) {
        self.version_cache
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

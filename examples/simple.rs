
extern crate x11_dl;
extern crate xrandr;

use x11_dl::xrandr::Xrandr;
use x11_dl::xlib::Xlib;

use std::os::raw::{c_int, c_ulong};
use std::ptr;
use std::time::Duration;

/// value is in millis
fn from_millis(value: c_ulong) -> Duration {
    Duration::new(
        (value / 1_000) as u64 ,
        ((value % 1_000) * 1_000_000) as u32
    )
}


fn main() {
    unsafe {
        let xlib = Xlib::open().unwrap();
        let xrandr = Xrandr::open().unwrap();
        let display = (xlib.XOpenDisplay)(ptr::null());

        if display.is_null() {
            panic!("Error opening default X display");
        }

        let screen = (xlib.XDefaultScreen)(display);
        let root = (xlib.XRootWindow)(display, screen);

        let mut event_base: c_int = 0;
        let mut error_base: c_int = 0;
        let mut major_v: c_int = 0;
        let mut minor_v: c_int = 0;

        // load extension info and check presence of randr extension

        if (xrandr.XRRQueryExtension)(display, &mut event_base, &mut error_base) == 0
            || (xrandr.XRRQueryVersion)(display, &mut major_v, &mut minor_v) == 0
        {
            panic!("Randr extension not present on current display");
        }

        let mut min_width: c_int = 0;
        let mut min_height: c_int = 0;
        let mut max_width: c_int = 0;
        let mut max_height: c_int = 0;
        (xrandr.XRRGetScreenSizeRange)(display, root, &mut min_width, &mut min_height, &mut max_width, &mut max_height);

        let resources = (xrandr.XRRGetScreenResourcesCurrent)(display, root);
        if resources.is_null() {
            panic!("Could not get screen resources");
        }

        for i in 0..(*resources).ncrtc {
            let info = (xrandr.XRRGetCrtcInfo)(display,
                                               resources,
                                               *(*resources).crtcs.offset(i as isize));
            println!("x: {}, y: {}, width: {}, height: {}, rotation: {}",
                     (*info).x, (*info).y, (*info).width, (*info).height, (*info).rotation);
        }

        println!("XRandr root {:?} screen {} version {}.{}", root, screen, major_v, minor_v);
        println!("min: {}x{}, max: {}x{}", min_width, min_height, max_width, max_height);
    }

    let xrr = xrandr::Xrandr::new();
    println!("{:?}", xrr.version());
}

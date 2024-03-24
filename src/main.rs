#![no_std]
#![no_main]

use libdragon::*;

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    console::init();
    console::set_debug(true);

    println!("Hello, Rust on N64!");

    loop {}
}


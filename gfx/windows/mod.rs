use alloc::string::String;

use super::{
    drawables::Drawable,
    vga::Color
};

use crate::gfx;

use core::panic::PanicInfo;
use crate::interrupts::global_timer;


#[allow(non_camel_case_types)]
pub struct panic_window {
    info : String
}

impl Drawable for panic_window {
    fn draw_self(&self) {
        gfx::clear(Color::Cyan);
        gfx::draw_string!(0,0, (Color::Red, Color::Cyan), "An Unrecoverable Error Occurred...");
        gfx::draw_string!(0,1, (Color::Red, Color::Cyan), "Error Info:\n {}", self.info);
        gfx::draw_string!(0,3, (Color::Red, Color::Cyan), "Uptime: {} Seconds", global_timer::get_seconds());
        gfx::draw_string!(0,2, (Color::Red, Color::Cyan), "Press Any Key To Do ABSOLUTELY NOTHING. NADA. NOUGHT. ZILCH!");
    } 
}

impl panic_window {
    pub fn from(info : &PanicInfo) -> panic_window {
        let mut s = String::new();
        core::fmt::write(&mut s, format_args!("{}", info)).expect("ERROR FORMATTING STRING...");
        panic_window {
            info : s
        }
    }
}

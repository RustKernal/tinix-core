pub mod idt;
pub mod pic;
pub mod pit;
pub mod gdt;
pub mod global_timer;

pub fn init() {
    idt::init();
    pic::init();

    x86_64::instructions::interrupts::enable();
}


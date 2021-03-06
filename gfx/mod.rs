pub mod vga;
pub mod drawables;
pub mod widgets;
pub mod windows;

pub use vga::Color as Color;

use core::fmt::{Write};

use x86_64::instructions::interrupts::without_interrupts;

use vga::Char;

use vga::{ColorCode};

pub fn swap() {
    vga::swap_buffers();
}

pub fn draw_line(
    x1 : usize, y1 : usize,
    x2 : usize, y2 : usize,
    color : Color
) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx == 0 { draw_rect(x1, y1, dx, dy, color) } //Vertical Line
    if dy == 0 { draw_rect(x1, y1, dx, dy, color) } //Horizontal Line

    
}

pub fn set_cell_color(x:usize, y:usize, fg:vga::Color, bg:vga::Color) {
    set_cell(x, y, b' ', fg, bg);
}

pub fn set_cell(mut x:usize,mut  y:usize, chr:u8, fg:vga::Color, bg:vga::Color) {
    if x >= 80 { x = 79 }
    if y >= 25 { y = 24 }
    without_interrupts(|| {
        vga::GLOBAL_VGA_BUFFER_2.lock().set_char(
                x, y,
            Char::new(chr,ColorCode::from_colors(fg, bg)
            )
        );
    });
}

pub fn get_bg(x:usize, y:usize) -> Color {
    vga::GLOBAL_VGA_BUFFER_2.lock().get_char(x,y).color.bg_as_color()
}

pub fn clear(color:Color) {
    for x in  0..vga::SCREEN_WIDTH {
        for y in 0..vga::SCREEN_HEIGHT {
            vga::GLOBAL_VGA_BUFFER_2.lock().set_char(x, y, Char::blank(ColorCode::from_colors(Color::White, color)));
        }
    }
    
}

pub fn draw(x:usize, y:usize, chr:u8, fg:vga::Color, bg:vga::Color) {
    set_cell(x,y,chr,fg,bg)
}

pub fn draw_string(x:usize,y:usize, text:&str, color:(vga::Color, vga::Color)) {
    let mut mut_y : usize = y;
    let mut mut_x : usize = x;
    let mut mut_color = color;
    for chr in text.bytes() {
        if chr == b'\n' { 
            mut_y += 1;
            mut_x = x; 
        } else if is_color_escape(chr) {
            mut_color = escape_code_to_color_tuple(chr, mut_color);
        } else {
            draw(mut_x, mut_y, chr, mut_color.0, mut_color.1);
            mut_x += 1;
        }
        
    }
}

fn escape_code_to_color_tuple(chr:u8, active_colors:(vga::Color, vga::Color)) -> (vga::Color, vga::Color) {
    let mut mut_colors = active_colors;
    if  chr <= 0x0f { //Set Background Color
        mut_colors.0 = vga::Color::from_u8(chr & 0x0f);
    }

    if chr >= 0x10 && chr <= 0x1f { //Set Foreground Color
        mut_colors.1 = vga::Color::from_u8(chr & 0x0f);
    }

    return mut_colors;
}

fn is_color_escape(chr:u8) -> bool {
   chr <= 0x1f
}

pub fn draw_rect(x:usize, y:usize, w:usize, h:usize, color:vga::Color) {
    for row in y..y+h {
        for col in x..x+w {
            draw(col, row, b' ', Color::Black, color);
        }
    }
}

pub fn set_gfx_mode(mode : vga::VgaMode) {
    vga::set_mode(mode)
}

pub macro draw_string($x:expr, $y:expr, $color:expr, $($arg:tt)*) {
    {
        let mut writer = VgaWriter {x : $x, y : $y, color : $color};
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    }
}


pub struct VgaWriter {
    x : usize,
    y : usize,
    color : (Color, Color)
}

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        draw_string(self.x, self.y, s, self.color);
        self.x += s.len();
        Ok(())
    }
} 

impl VgaWriter {
    pub fn print_str(&self, text : &str) {
        draw_string(self.x, self.y, text, self.color);
    }
}

use core::fmt;

pub struct Black(&'static str);

impl Black {
    pub fn new(s : &'static str) -> Black {Black(s)}
}

impl fmt::Display for Black {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x00")?; // prefix code
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
pub struct Blue(&'static str);

impl Blue {
    pub fn new(s : &'static str) -> Blue {Blue(s)}
}

impl fmt::Display for Blue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x01")?; // prefix code
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
pub struct Green(&'static str);

impl Green {
    pub fn new(s : &'static str) -> Green {Green(s)}
}

impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x02")?; // prefix code
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

pub struct Cyan(&'static str);

impl Cyan {
    pub fn new(s : &'static str) -> Cyan {Cyan(s)}
}

impl fmt::Display for Cyan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x03")?; // prefix code
        write!(f, "{}", self.0)?;
        Ok(())
    }
}


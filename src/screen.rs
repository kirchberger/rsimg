use std::io::stdout;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, WindowSize, window_size, enable_raw_mode,
        disable_raw_mode},
    event::{read,Event,poll},
};

use crate::image;

pub struct Window {
    pub height : u16,
    pub width : u16,
}

pub fn setup(window : &mut Window) -> Result<(),()> {
    let mut stdout = std::io::stdout();
    let window_size = window_size().unwrap();
    window.width = window_size.width;
    window.height = window_size.height;

    execute!(stdout, EnterAlternateScreen);
    enable_raw_mode();

    Ok(())
}

pub fn exit() -> Result<(),()> {
    let mut stdout = std::io::stdout();

    execute!(stdout, LeaveAlternateScreen);
    disable_raw_mode();

    Ok(())
}

pub fn render(image : & image::Image, window : & Window) {
    println!("Rendering screen");
}

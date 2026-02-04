use std::io::{BufWriter, Write};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, window_size, enable_raw_mode,
        disable_raw_mode,BeginSynchronizedUpdate, EndSynchronizedUpdate},
};

use crate::image;

pub struct Window {
    pub height : usize,
    pub width : usize,
}

pub fn setup(window : &mut Window) -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let win_size = window_size()?;
    window.width = win_size.columns as usize;
    window.height = win_size.rows as usize;

    Ok(())
}

pub fn exit() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

pub fn render(image : & image::Image, window : & Window) {

    let acsii_lookup : [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    let mut temp : usize;
    let mut temp_index : usize;
    let mut bufout = BufWriter::new(std::io::stdout());
    let reset : [u8; 4] = [27, b'[', b'0', b'm'];
    let blank : [u8; 1] = [b' '];
    let mut forebuf : [u8; 19] = [27, b'[', b'3', b'8', b';', b'2', b';', b'0', b'0',b'0', b';', b'0', b'0' ,b'0', b';', b'0', b'0', b'0', b'm'];
    let mut backbuf : [u8; 19] = [27, b'[', b'4', b'8', b';', b'2', b';', b'0', b'5',b'5', b';', b'2', b'5' ,b'5', b';', b'2', b'5', b'5', b'm'];
    let unicode : [u8; 3] = [0b11100010, 0b10010110, 0b10000000];
    
    execute!(std::io::stdout(),BeginSynchronizedUpdate).unwrap();    
    if image.height/2 < window.height {
        bufout.write(& reset).unwrap();
        for _ in 0..window.width*((window.height - (image.height/2))/2) {
            bufout.write(& blank).unwrap();
        }
    }

    for i in 0..(image.height + 1)/2 {
        if image.width < window.width {
        bufout.write(& reset).unwrap();
            for _ in 0..(window.width - image.width) / 2 {
                bufout.write(& blank).unwrap();
            }
        }
        for j in 0..image.width {
            // Red 
            temp = image.pixels[3*(2*i*image.width + j)] as usize;
            temp_index = temp%10;
            forebuf[9] = acsii_lookup[temp_index];
            temp_index = temp/10%10;
            forebuf[8] = acsii_lookup[temp_index];
            temp_index = temp/100%10;
            forebuf[7] = acsii_lookup[temp_index];
            // Green 
            temp = image.pixels[3*(2*i*image.width + j)+1] as usize;
            temp_index = temp%10;
            forebuf[13] = acsii_lookup[temp_index];
            temp_index = temp/10%10;
            forebuf[12] = acsii_lookup[temp_index];
            temp_index = temp/100%10;
            forebuf[11] = acsii_lookup[temp_index];
            // Blue 
            temp = image.pixels[3*(2*i*image.width + j)+1] as usize;
            temp_index = temp%10;
            forebuf[17] = acsii_lookup[temp_index];
            temp_index = temp/10%10;
            forebuf[16] = acsii_lookup[temp_index];
            temp_index = temp/100%10;
            forebuf[15] = acsii_lookup[temp_index];

            if i != image.height/2 { // Background
                // Red
                temp = image.pixels[3*((2*i+1)*image.width + j)] as usize;
                temp_index = temp%10;
                backbuf[9] = acsii_lookup[temp_index];
                temp_index = temp/10%10;
                backbuf[8] = acsii_lookup[temp_index];
                temp_index = temp/100%10;
                backbuf[7] = acsii_lookup[temp_index];
                // Green 
                temp = image.pixels[3*((2*i+1)*image.width + j)+1] as usize;
                temp_index = temp%10;
                backbuf[13] = acsii_lookup[temp_index];
                temp_index = temp/10%10;
                backbuf[12] = acsii_lookup[temp_index];
                temp_index = temp/100%10;
                backbuf[11] = acsii_lookup[temp_index];
                // Blue 
                temp = image.pixels[3*((2*i+1)*image.width + j)+1] as usize;
                temp_index = temp%10;
                backbuf[17] = acsii_lookup[temp_index];
                temp_index = temp/10%10;
                backbuf[16] = acsii_lookup[temp_index];
                temp_index = temp/100%10;
                backbuf[15] = acsii_lookup[temp_index];

                bufout.write(& backbuf).unwrap();
            } else {
                bufout.write(& reset).unwrap();
            }

            bufout.write(& forebuf).unwrap();
            bufout.write(& unicode).unwrap();
        }
        if image.width < window.width {
            bufout.write(& reset).unwrap();
            for _ in 0..(window.width - image.width + 1) / 2 {
                bufout.write(& blank).unwrap();
            }
        }
    }

    if image.height/2 < window.height {
        bufout.write(& reset).unwrap();
        for _ in 0..window.width*((window.height - (image.height/2))/2) {
            bufout.write(& blank).unwrap();
        }
    }

    bufout.flush().unwrap();
    execute!(std::io::stdout(),EndSynchronizedUpdate).unwrap();    
}

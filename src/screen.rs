use std::{time::Duration,io::{BufWriter, Write}};
use crossterm::{
    event::poll, 
    terminal::{window_size, enable_raw_mode, disable_raw_mode,},
};

use crate::image;

pub struct Window {
    pub height : usize,
    pub width : usize,
    pub columns : usize,
    pub rows : usize,
}

// Set alternate buffer
pub fn setup() -> std::io::Result<()> {

    let mut bufout = BufWriter::new(std::io::stdout());
    let disable_cursor : [u8; 6] = [27, b'[', b'?', b'2', b'5', b'l'];
    let enable_alt_buffer : [u8; 8] = [27, b'[', b'?', b'1', b'0', b'4', b'9', b'h'];

    bufout.write(& enable_alt_buffer).unwrap();
    bufout.write(& disable_cursor).unwrap();
    bufout.flush().unwrap();
    
    enable_raw_mode()?;

    Ok(())
}

// Exit alternate buffer
pub fn exit() -> std::io::Result<()> {

    let mut bufout = BufWriter::new(std::io::stdout());
    let enable_cursor : [u8; 6] = [27, b'[', b'?', b'2', b'5', b'h'];
    let disable_alt_buffer  : [u8; 8] = [27, b'[', b'?', b'1', b'0', b'4', b'9', b'l'];

    disable_raw_mode()?;

    bufout.write(& disable_alt_buffer).unwrap();
    bufout.write(& enable_cursor).unwrap();
    bufout.flush().unwrap();

    Ok(())
}

// Get window size
pub fn update_window(window : &mut Window) {

    let win_size = window_size().unwrap();

    window.height = win_size.height as usize;
    window.width = win_size.width as usize;
    window.columns = win_size.columns as usize;
    window.rows = win_size.rows as usize;

}

pub fn usr_cancel() {
    loop {
        if  poll(Duration::from_millis(100)).unwrap() {
            break;
        }
    }
}

// Render image in the center of the screen
pub fn render_image(image : & image::Image, window : & Window) {

    let acsii_lookup : [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    let mut forebuf : [u8; 19] = [27, b'[', b'3', b'8', b';', b'2', b';', b'0', b'0',b'0', b';', b'0', b'0' ,b'0', b';', b'0', b'0', b'0', b'm'];
    let mut backbuf : [u8; 19] = [27, b'[', b'4', b'8', b';', b'2', b';', b'0', b'5',b'5', b';', b'2', b'5' ,b'5', b';', b'2', b'5', b'5', b'm'];
    let blank : [u8; 1] = [b' '];
    let unicode : [u8; 3] = [0b11100010, 0b10010110, 0b10000000];
    let clear : [u8; 6] = [27, b'[', b'H', 27, b'[', b'J'];
    let reset : [u8; 4] = [27, b'[', b'0', b'm'];
    let begin_sync_update : [u8; 8] = [27, b'[', b'?', b'2', b'0', b'2', b'6', b'h'];
    let end_sync_update : [u8; 8] = [27, b'[', b'?', b'2', b'0', b'2', b'6', b'l'];

    let mut temp : usize;
    let mut temp_index : usize;
    
    let mut bufout = BufWriter::new(std::io::stdout());

    // All code below produces one frame
    bufout.write(& begin_sync_update).unwrap();
    bufout.write(& clear).unwrap();

    // Pad top of image
    if image.height / 2 < window.rows {
        bufout.write(& reset).unwrap();
        for _ in 0..window.columns * ((window.rows - (image.height / 2)) / 2) {
            bufout.write(& blank).unwrap();
        }
    }
    
    // Display image
    for i in 0..(image.height + 1) / 2 {
        
        // Pad left of image row
        if image.width < window.columns{
        bufout.write(& reset).unwrap();
            for _ in 0..(window.columns - image.width) / 2 {
                bufout.write(& blank).unwrap();
            }
        }

        // Display image row
        for j in 0..image.width {

            // Gernerate top pixel as foreground
            // Red 
            temp = image.pixels[3 * (2 * i * image.width + j)] as usize;
            temp_index = temp % 10;
            forebuf[9] = acsii_lookup[temp_index];
            temp_index = temp / 10 % 10;
            forebuf[8] = acsii_lookup[temp_index];
            temp_index = temp / 100 % 10;
            forebuf[7] = acsii_lookup[temp_index];
            // Green 
            temp = image.pixels[3 * (2 * i * image.width + j) + 1] as usize;
            temp_index = temp % 10;
            forebuf[13] = acsii_lookup[temp_index];
            temp_index = temp / 10 % 10;
            forebuf[12] = acsii_lookup[temp_index];
            temp_index = temp / 100 % 10;
            forebuf[11] = acsii_lookup[temp_index];
            // Blue 
            temp = image.pixels[3 * (2 * i * image.width + j) + 2] as usize;
            temp_index = temp % 10;
            forebuf[17] = acsii_lookup[temp_index];
            temp_index = temp / 10 % 10;
            forebuf[16] = acsii_lookup[temp_index];
            temp_index = temp / 100 % 10;
            forebuf[15] = acsii_lookup[temp_index];

            // Generate bottow pixel as baskgroud
            if i != image.height/2 { 
                // Red
                temp = image.pixels[3 * ((2 * i + 1)*image.width + j)] as usize;
                temp_index = temp % 10;
                backbuf[9] = acsii_lookup[temp_index];
                temp_index = temp / 10 % 10;
                backbuf[8] = acsii_lookup[temp_index];
                temp_index = temp/100 % 10;
                backbuf[7] = acsii_lookup[temp_index];
                // Green 
                temp = image.pixels[3 * ((2 * i + 1) * image.width + j) + 1] as usize;
                temp_index = temp % 10;
                backbuf[13] = acsii_lookup[temp_index];
                temp_index = temp / 10 % 10;
                backbuf[12] = acsii_lookup[temp_index];
                temp_index = temp / 100 % 10;
                backbuf[11] = acsii_lookup[temp_index];
                // Blue 
                temp = image.pixels[3 * ((2 * i + 1) * image.width + j) + 2] as usize;
                temp_index = temp % 10;
                backbuf[17] = acsii_lookup[temp_index];
                temp_index = temp / 10 % 10;
                backbuf[16] = acsii_lookup[temp_index];
                temp_index = temp / 100 % 10;
                backbuf[15] = acsii_lookup[temp_index];

                // Write background
                bufout.write(& backbuf).unwrap();

            } else { // Reset background when there is no bottom pixel
                bufout.write(& reset).unwrap();
            }

            // Write forground and top half unicode charactor
            bufout.write(& forebuf).unwrap();
            bufout.write(& unicode).unwrap();
        }

        // Pad left of image row
        if image.width < window.columns {
            bufout.write(& reset).unwrap();
            for _ in 0..(window.columns - image.width + 1) / 2 {
                bufout.write(& blank).unwrap();
            }
        }
    }

    // Pad bottom of image
    if image.height / 2 < window.rows {
        bufout.write(& reset).unwrap();
        for _ in 0..window.columns * ((window.rows - (image.height / 2)) / 2) {
            bufout.write(& blank).unwrap();
        }
    }
    
    // Flush buffer writes and update screen
    bufout.write(& end_sync_update).unwrap();
    bufout.flush().unwrap();
    //execute!(std::io::stdout(),EndSynchronizedUpdate).unwrap();    
}

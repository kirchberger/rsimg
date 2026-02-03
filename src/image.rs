use zune_jpeg::JpegDecoder;
use std::{io::{BufReader},string,any::type_name,fs::read};

use crate::screen;

pub struct Image {
    pub pixels : Vec<u8>,
    pub width : u16,
    pub height : u16,
}

pub fn decode_file(file : & String, image : &mut Image) -> Result<(),()> {

    let file_contents = BufReader::new(std::fs::File::open(file).unwrap());
    let mut decoder = JpegDecoder::new(file_contents);
    image.pixels = decoder.decode().unwrap();
    let image_info = decoder.info().unwrap();
    
    image.width = image_info.width;
    image.height = image_info.height;


    Ok(())
}

pub fn downsize(image : & Image, image_downsize : &mut Image, window : & screen::Window) {
    println!("Downsizing Image");
}

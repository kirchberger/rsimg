use zune_jpeg::JpegDecoder;
//use zune_png::PngDecoder;
use std::io::BufReader;

use crate::screen;

pub struct Image {
    pub pixels : Vec<u8>,
    pub width : usize,
    pub height : usize,
}

pub fn decode_file(file : & String, image : &mut Image) -> Result<(),()> {

    let file_contents = BufReader::new(std::fs::File::open(file).unwrap());
    let mut decoder = JpegDecoder::new(file_contents);
    image.pixels = decoder.decode().unwrap();
    let image_info = decoder.info().unwrap();
    
    image.width = image_info.width as usize;
    image.height = image_info.height as usize;

    Ok(())
}

pub fn downsize(image : & Image, image_downsize : &mut Image, window : & screen::Window) {

    // width to height ratio of image and window
    let image_ratio = image.width as f64 / image.height as f64;
    let window_ratio = window.width as f64 / window.height as f64;

    // terminal charactor boxes are not always the same siz and this accounts for this 
    let box_width = window.width / window.columns;
    let box_height = window.height / window.rows;
    
    if image_ratio > window_ratio { // Image is wider the window
        image_downsize.width = window.columns;
        image_downsize.height = (2 * box_width * image.height * window.columns) / (box_height * image.width);
    }
    else if image_ratio < window_ratio { // Window is wider than image
        image_downsize.height = 2*window.rows;
        image_downsize.width = (box_height * image.width * window.rows) / (box_width * image.height);
    }
    else { // Image perfectly fills window
        image_downsize.width = window.columns;
        image_downsize.height = 2 * window.rows;
    }

    // Downsample image by averaging samples
    // Cannot find the format of the the byte stream so assuming R,G,B of (0,0) then (0,1) ect
    image_downsize.pixels = vec![0; 3 * image_downsize.width * image_downsize.height];
    let mut red : usize;        
    let mut green : usize;        
    let mut blue : usize;        
    let mut low_range_col : usize;
    let mut high_range_col : usize;
    let mut low_range_row : usize ;
    let mut high_range_row : usize = 0;
    
    for i in 0..image_downsize.height { // Rows in downsampled image

        low_range_row = high_range_row;
        high_range_row = ((i + 1) * image.height) / image_downsize.height;

        high_range_col = 0;

        for j in 0..image_downsize.width { // Rows in downsampled image

            red = 0;        
            green = 0;        
            blue = 0;        

            low_range_col = high_range_col;
            high_range_col = ((j + 1) * image.width) / image_downsize.width;
            
            for k in low_range_row..high_range_row { 
                for l in low_range_col..high_range_col { // Rows which count towards the pixel
                                                         //
                    red += image.pixels[3 * ((k * image.width) + l)] as usize;
                    green += image.pixels[3 * ((k * image.width) + l) + 1] as usize;
                    blue += image.pixels[3 * ((k * image.width) + l) + 2] as usize;
                }
            }
            image_downsize.pixels[3 * ((i * image_downsize.width) + j)] = 
                (red / ((high_range_row - low_range_row) * (high_range_col - low_range_col))) as u8;

            image_downsize.pixels[3 * ((i * image_downsize.width) + j) + 1] = 
                (green / ((high_range_row - low_range_row) * (high_range_col - low_range_col))) as u8;

            image_downsize.pixels[3 * ((i * image_downsize.width) + j) + 2] = 
                (blue  / ((high_range_row - low_range_row) * (high_range_col - low_range_col))) as u8;
        }
    }
}

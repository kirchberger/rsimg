use zune_jpeg::JpegDecoder;
use zune_png::PngDecoder;
use zune_core::colorspace::ColorSpace;
use std::{io::BufReader};

pub struct Image {
    pub pixels : Vec<u8>,
    pub width : usize,
    pub height : usize,
}

fn rgba_to_rgb(image : &mut Image) {

    let length = image.width * image.height;
    let mut new : Vec<u8> = vec![0; 3*length];

    for i in 0..length {
        new[3 * i] = image.pixels[4 * i];
        new[3 * i + 1] = image.pixels[4 * i + 1];
        new[3 * i + 2] = image.pixels[4 * i + 2];
    }
    image.pixels = new;
}

fn decode_jpeg(file : & String, image : &mut Image) {

    let file_contents = BufReader::new(std::fs::File::open(file).unwrap());
    let mut decoder = JpegDecoder::new(file_contents);
    image.pixels = decoder.decode().unwrap();
    
    let image_info = decoder.info().unwrap();
    image.width = image_info.width as usize;
    image.height = image_info.height as usize;

    match decoder.input_colorspace().unwrap() {
        ColorSpace::RGB => (),
        ColorSpace::RGBA => rgba_to_rgb(image),
        _ => (),
    }
}

fn decode_png(file : & String, image : &mut Image) {

    let file_contents = BufReader::new(std::fs::File::open(file).unwrap());
    let mut decoder = PngDecoder::new(file_contents);
    image.pixels = decoder.decode_raw().unwrap();
    
    let image_info = decoder.info().unwrap();
    image.width = image_info.width as usize;
    image.height = image_info.height as usize;

    match decoder.colorspace().unwrap() {
        ColorSpace::RGB => (),
        ColorSpace::RGBA => rgba_to_rgb(image),
        _ => (),
    }
}
    

pub fn decode_file(file : & String, image : &mut Image) -> Result<(),()> {

    let ext = file.split('.').last().unwrap();

    match ext {
        "jpg" | "jpeg" | "JPG" | "JPEG"  => decode_jpeg(file, image),
        "png" => decode_png(file, image),
        _ => panic!("unknown file type"),
    }

    Ok(())
}

// not downsampled in a good way at this point
pub fn downsample(image : & Image, image_downsample : &mut Image) {

    // Downsample image by averaging samples
    image_downsample.pixels = vec![0; 3 * image_downsample.width * image_downsample.height];
    let mut red : usize;        
    let mut green : usize;        
    let mut blue : usize;        
    let mut low_range_col : usize;
    let mut high_range_col : usize;
    let mut low_range_row : usize ;
    let mut high_range_row : usize = 0;

    
    for i in 0..image_downsample.height { // Rows in downsampled image

        low_range_row = high_range_row;
        high_range_row = ((i + 1) * image.height) / image_downsample.height;
        high_range_col = 0;

        for j in 0..image_downsample.width { // Rows in downsampled image

            red = 0;        
            green = 0;        
            blue = 0;        

            low_range_col = high_range_col;
            high_range_col = ((j + 1) * image.width) / image_downsample.width;
            
            for k in low_range_row..high_range_row { 
                for l in low_range_col..high_range_col { // Rows which count towards the pixel
                                                         //
                    red += image.pixels[3 * ((k * image.width) + l)] as usize;
                    green += image.pixels[3 * ((k * image.width) + l) + 1] as usize;
                    blue += image.pixels[3 * ((k * image.width) + l) + 2] as usize;
                }
            }
            image_downsample.pixels[3 * ((i * image_downsample.width) + j)] = 
                (red / ((high_range_row - low_range_row) * (high_range_col - low_range_col))) as u8;

            image_downsample.pixels[3 * ((i * image_downsample.width) + j) + 1] = 
                (green / ((high_range_row - low_range_row) * (high_range_col - low_range_col))) as u8;

            image_downsample.pixels[3 * ((i * image_downsample.width) + j) + 2] = 
                (blue  / ((high_range_row - low_range_row) * (high_range_col - low_range_col))) as u8;
        }
    }
}

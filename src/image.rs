use zune_jpeg::JpegDecoder;
use zune_png::PngDecoder;
use zune_core::{colorspace::ColorSpace, options::DecoderOptions,bit_depth::BitDepth};
use std::{io::BufReader};

pub struct Image {
    pub pixels : Vec<u8>,
    pub width : usize,
    pub height : usize,
}

fn rgba_to_rgb(image : &mut Image) {

    let length = image.width * image.height;
    let mut new : Vec<u8> = vec![0; 3 * length];

    for i in 0..length {
        new[3 * i] = image.pixels[4 * i];
        new[3 * i + 1] = image.pixels[4 * i + 1];
        new[3 * i + 2] = image.pixels[4 * i + 2];
    }
    image.pixels = new;
}

/*fn ycbcr_to_rgb(_image : &mut Image) {
    panic!("YCbCr not supported");
}*/

fn luma_to_rgb(image : &mut Image) {

    let length = image.width * image.height;
    let mut new : Vec<u8> = vec![0; 3 * length];

    for i in 0..length {
        new[3 * i] = image.pixels[i];
        new[3 * i + 1] = image.pixels[i];
        new[3 * i + 2] = image.pixels[i];
    }
    image.pixels = new;
}

fn lumaa_to_rgb(image : &mut Image) {

    let length = image.width * image.height;
    let mut new : Vec<u8> = vec![0; 3 * length];

    for i in 0..length {
        new[3 * i] = image.pixels[2 * i];
        new[3 * i + 1] = image.pixels[2 * i];
        new[3 * i + 2] = image.pixels[2 * i];
    }
    image.pixels = new;
}

/*fn tcck_to_rgb(_image : &mut Image) {
    panic!("Not implemented for tcck colorspace");
}

fn cmyk_to_rgb(image : &mut Image) {
    
    let length = image.width * image.height;
    let mut new : Vec<u8> = vec![0; 3 * length];
    let mut temp : usize;

    for i in 0..length {
        temp = 255 - image.pixels[4 * i + 3] as usize;
        new[3 * i] = (((255 - image.pixels[4 * i] as usize) * temp)/255) as u8;
        new[3 * i + 1] = (((255 - image.pixels[4 * i + 1] as usize) * temp)/255) as u8;
        new[3 * i + 2] = (((255 - image.pixels[4 * i + 2] as usize) * temp)/255) as u8;
    }
    image.pixels = new;
}

fn bgr_to_rgb(image : &mut Image) {

    let length = image.width * image.height;
    let mut temp : u8;

    for i in 0..length {
        temp = image.pixels[3 * i];
        image.pixels[3 * i] = image.pixels[3 * i + 2];
        image.pixels[3 * i + 2] = temp;
    }
}

fn bgra_to_rgb(image : &mut Image) {

    let length = image.width * image.height;
    let mut new : Vec<u8> = vec![0; 3 * length];

    for i in 0..length {
        new[3 * i] = image.pixels[4 * i + 2];
        new[3 * i + 1] = image.pixels[4 * i + 1];
        new[3 * i + 2] = image.pixels[4 * i];
    }
    image.pixels = new;
}

fn argb_to_rgb(image : &mut Image) {

    let length = image.width * image.height;
    let mut new : Vec<u8> = vec![0; 3 * length];

    for i in 0..length {
        new[3 * i] = image.pixels[4 * i + 1];
        new[3 * i + 1] = image.pixels[4 * i + 2];
        new[3 * i + 2] = image.pixels[4 * i + 3];
    }
    image.pixels = new;
}

fn hsl_to_rgb(_image : &mut Image) {
    panic!("Not implemented for hsl colourspace");
}

fn hsv_to_rgb(_image : &mut Image) {
    panic!("Not implemented for hsv colourspace");
}*/

fn decode_jpeg(file : & String, image : &mut Image) {

    let file_contents = BufReader::new(std::fs::File::open(file).unwrap());
    // Default option is to decode into RGB colourspace
    let mut decoder = JpegDecoder::new(file_contents);

    image.pixels = decoder.decode().unwrap();
    
    let image_info = decoder.info().unwrap();
    image.width = image_info.width as usize;
    image.height = image_info.height as usize;

}

fn decode_png(file : & String, image : &mut Image) {

    let file_contents = BufReader::new(std::fs::File::open(file).unwrap());
    let mut decoder = PngDecoder::new(file_contents);
    decoder.options().png_set_strip_to_8bit(true);
    
    image.pixels = decoder.decode_raw().unwrap();
    
    let image_info = decoder.info().unwrap();
    image.width = image_info.width as usize;
    image.height = image_info.height as usize;

    if decoder.depth().unwrap() == BitDepth::Sixteen {

    let mut new : Vec<u8> = vec![0; image.pixels.len()/2];
    
    for i in 0..image.pixels.len()/2 {
        new[i] = image.pixels[2*i];
    }
    image.pixels = new;
    }


    // These are all the png supported colourspaces
    match decoder.colorspace().unwrap() {
        ColorSpace::RGB     => (),
        ColorSpace::RGBA    => rgba_to_rgb(image),
        ColorSpace::Luma    => luma_to_rgb(image),
        ColorSpace::LumaA   => lumaa_to_rgb(image),
        _ => (),
    }
}
    

pub fn decode_file(file : & String, image : &mut Image) -> Result<(),()> {

    let ext = file.split('.').last().unwrap();

    match ext {
        "jpg" | "jpeg" | "JPG" | "JPEG"  => decode_jpeg(file, image),
        "png" | "PNG" => decode_png(file, image),
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

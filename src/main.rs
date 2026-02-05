// Will slowly reduce external dependancies
use std::{env,time::Duration};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};

pub mod screen;
pub mod image;

fn dimension_image(image : & image::Image, image_downsample : &mut image::Image, window : & screen::Window){
    // width to height ratio of image and window
    let image_ratio = image.width as f64 / image.height as f64;
    let window_ratio = window.width as f64 / window.height as f64;

    // terminal charactor boxes are not always the same siz and this accounts for this 
    let box_width = window.width / window.columns;
    let box_height = window.height / window.rows;
    
    if image_ratio > window_ratio { // Image is wider the window
        image_downsample.width = window.columns;
        image_downsample.height = (2 * box_width * image.height * window.columns) / (box_height * image.width);
    }
    else if image_ratio < window_ratio { // Window is wider than image
        image_downsample.height = 2*window.rows;
        image_downsample.width = (box_height * image.width * window.rows) / (box_width * image.height);
    }
    else { // Image perfectly fills window
        image_downsample.width = window.columns;
        image_downsample.height = 2 * window.rows;
    }
}


fn main() -> Result<(),String> {
    
    let args : Vec<String> = env::args().collect(); // collect arguments
    let arglen = args.len();
    
    if arglen != 2 { // check number of arguments
        return Err(String::from("Incorrect number of arguments"));
    }
    
    let file = & args[1];
    let mut image = image::Image{
        pixels : Vec::new(),
        width : 0,
        height : 0
    };
    let mut image_downsample = image::Image{
        pixels : Vec::new(),
        width : 0,
        height : 0
    };
    let mut window = screen::Window{
        width : 0,
        height : 0,
        columns : 0,
        rows : 0
    };

    // Decode image 
    image::decode_file(file, & mut image).expect("Issue opening or decoding the file");
    
    // Setup screen
    screen::setup().unwrap();

    // Get window size
    screen::update_window(& mut window);

    // Find dimensions of image that can be displayed
    dimension_image(& image, &mut image_downsample, & window);

    // Downsample image 
    image::downsample(& image, &mut image_downsample);

    // Render image
    screen::render_image(& image_downsample, & window);

    // Wait for cancel input
    loop { 
        if poll(Duration::from_millis(50)).unwrap() {

            match read().unwrap() {
                Event::FocusGained => (),
                Event::FocusLost => (),
                Event::Key(event) => 
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                        break;
                    },
                Event::Mouse(_event) => (),
                Event::Paste(_data) => (),
                Event::Resize(_, _) => { 
                    // Update window size
                    screen::update_window(& mut window);
                    
                    // Check if image dimensions have changed
                    let height = image_downsample.height;
                    let width = image_downsample.width;
                    dimension_image(& image, &mut image_downsample, & window);

                    // If required dimensions have changed re downsample image
                    if (height != image_downsample.height) | (width != image_downsample.width) {
                        image::downsample(& image, &mut image_downsample);
                    }

                    screen::update_window(& mut window);
                    
                    // Re-render image
                    screen::render_image(& image_downsample, & window);},
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }


    // screen::usr_cancel();

    // Exit alternate screen
    screen::exit().unwrap();

    Ok(())
}

// Will slowly reduce external dependancies
use std::{env,io::{BufReader},string,time::Duration,any::type_name,fs::read,ptr::null};
use crossterm::{event::poll};

pub mod screen;
pub mod image;


fn usr_cancel() {
    loop {
        if  poll(Duration::from_millis(100)).unwrap() {
            break;
        }
    }
}

fn main() -> Result<(),String> {
    
    let args : Vec<String> = env::args().collect(); // collect arguments
    let arglen = args.len();
    
    if arglen != 2 { // check number of arguments
        return Err(String::from("Incorrect number of arguments"));
    }
    
    let file = String::from("./testImages/japanese_maple.jpg");
    let mut image = image::Image{
        pixels : Vec::new(),
        width : 0,
        height : 0
    };
    let mut image_downsize = image::Image{
        pixels : Vec::new(),
        width : 0,
        height : 0
    };
    let mut window = screen::Window{
        width : 0,
        height : 0
    };

    // Decode image 
    image::decode_file(& file, & mut image);
    
    // Setup screen and get window size
    screen::setup(& mut window);

    // Down size image to correct resolution
    image::downsize(& image, &mut image_downsize, &window);

    // Render image
    screen::render(& image_downsize, & window);

    // Wait for cancel input
    usr_cancel();

    // Exit alternate screen
    screen::exit();
    println!("{:?}",image_downsize.pixels);
    Ok(())
}

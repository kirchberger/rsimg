// Will slowly reduce external dependancies
use std::{env,io,string,time::Duration};
use crossterm::{event::poll};
//struct image

pub mod screen;

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
    
    // decode image 
    
    // down size image to correct resolution


    // render image
    screen::setup();
    screen::render();

    // wait for cancel input
    usr_cancel();
    
    Ok(())
}

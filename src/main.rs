// Will slowly reduce external dependancies
use std::{env,io,string,time::Duration};
use crossterm::{event::poll};
//struct image

fn usr_cancel() -> Result<(),()> {
    loop {
        if  poll(Duration::from_millis(100)).unwrap() {
            return Ok(());
        }
    }
}

fn main() -> Result<(),bool> {
    
    let args : Vec<String> = env::args().collect(); // collect arguments
    let arglen = args.len();
    
    if arglen != 2 { // check number of arguments
        //return Err(String::from("Incorrect number of arguments"));
        return Ok(());
    }
    
    // decode image 
    
    // down size image to correct resolution

    // render image

    // wait for cancel input
    loop {
        if  poll(Duration::from_millis(100)).unwrap() {
            break;
        }
    }
    
    Ok(())
}

use std::env;
use std::io;


fn main() -> std::io::Result<()> {
    for argument in env::args() {
            println!("{argument}");
    }

    Ok(())
}

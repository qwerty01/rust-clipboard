extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;

    let the_string = "Hello, world!";

    ctx.set_contents(the_string.to_owned())?;

    Ok(())
}

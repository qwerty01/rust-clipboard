extern crate clipboard;

#[cfg(target_os = "linux")]
use clipboard::x11_clipboard::{Primary, X11ClipboardContext};
use clipboard::ClipboardProvider;
use std::error::Error;

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx: X11ClipboardContext<Primary> = ClipboardProvider::new()?;

    let the_string = "Hello, world!";

    ctx.set_contents(the_string.to_owned())?;

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Primary selection is only available under linux!");
}

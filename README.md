# rust-clipboard

rust-clipboard is a cross-platform library for getting and setting the contents of the OS-level clipboard.  
It has been tested on Windows, Mac OSX, GNU/Linux, and FreeBSD.
It is used in Mozilla Servo.

[![](http://meritbadge.herokuapp.com/clipboard)](https://crates.io/crates/clipboard)
[![Appveyor Build Status](https://ci.appveyor.com/api/projects/status/github/aweinstock314/rust-clipboard)](https://ci.appveyor.com/project/aweinstock314/rust-clipboard)
[![Travis Build Status](https://travis-ci.org/aweinstock314/rust-clipboard.svg?branch=master)](https://travis-ci.org/aweinstock314/rust-clipboard)

## Prerequisites

On Linux you need the x11 library, install it with something like:

```bash
sudo apt-get install xorg-dev
```

## Example

```rust
extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use clipboard::Error;

fn example() -> Result<(), Error> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned())?;
    Ok(())
}
```

## API

The `ClipboardProvider` trait has the following functions:

```rust
fn new() -> clipboard::Result<Self>;
fn get_contents(&mut self) -> clipboard::Result<String>;
fn set_contents(&mut self, String) -> clipboard::Result<()>;
```

`ClipboardContext` is a type alias for one of {`WindowsClipboardContext`, `OSXClipboardContext`, `X11ClipboardContext`, `NopClipboardContext`}, all of which implement `ClipboardProvider`. Which concrete type is chosen for `ClipboardContext` depends on the OS (via conditional compilation).

## License

`rust-clipboard` is dual-licensed under MIT and Apache2.

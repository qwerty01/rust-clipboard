/*
Copyright 2016 Avraham Weinstock

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#![crate_name = "clipboard"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

use std::error;
use std::fmt;
use std::result;

#[cfg(target_os = "macos")]
use osx_clipboard::OSXError;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
extern crate x11_clipboard as x11_clipboard_crate;

#[cfg(windows)]
extern crate clipboard_win;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
extern crate objc_foundation;
#[cfg(target_os = "macos")]
extern crate objc_id;

mod common;
pub use common::ClipboardProvider;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub mod x11_clipboard;

#[cfg(windows)]
pub mod windows_clipboard;

#[cfg(target_os = "macos")]
pub mod osx_clipboard;

pub mod nop_clipboard;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub type ClipboardContext = x11_clipboard::X11ClipboardContext;
#[cfg(windows)]
pub type ClipboardContext = windows_clipboard::WindowsClipboardContext;
#[cfg(target_os = "macos")]
pub type ClipboardContext = osx_clipboard::OSXClipboardContext;
#[cfg(target_os = "android")]
pub type ClipboardContext = nop_clipboard::NopClipboardContext; // TODO: implement AndroidClipboardContext (see #52)
#[cfg(not(any(
    unix,
    windows,
    target_os = "macos",
    target_os = "android",
    target_os = "emscripten"
)))]
pub type ClipboardContext = nop_clipboard::NopClipboardContext;

#[derive(Debug)]
pub enum Error {
    #[cfg(all(
        unix,
        not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
    ))]
    X11Error(x11_clipboard::X11Error),
    #[cfg(target_os = "macos")]
    OSXError(OSXError),
    #[cfg(windows)]
    WindowsError(windows_clipboard::WindowsError),
    // TODO: create AndroidError type when AndroidClipboardContext is created
    NopError(nop_clipboard::NopError),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(all(
                unix,
                not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
            ))]
            Self::X11Error(e) => e.fmt(f),
            #[cfg(target_os = "macos")]
            Self::OSXError(e) => e.fmt(f),
            #[cfg(windows)]
            Self::WindowsError(e) => e.fmt(f),
            // TODO: create AndroidError type when AndroidClipboardContext is created
            Self::NopError(e) => e.fmt(f),
        }
    }
}
impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

#[test]
fn test_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents("some string".to_owned()).unwrap();
    assert!(ctx.get_contents().unwrap() == "some string");
}

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

use clipboard_win::{get_clipboard_string, set_clipboard_string};

use common::ClipboardProvider;
use std::error;
use std::io;
use std::fmt;
use crate::{Error, Result};

pub struct WindowsClipboardContext;

#[derive(Debug)]
pub enum WindowsError {
    IoError(io::Error)
}
impl fmt::Display for WindowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f)
        }
    }
}
impl error::Error for WindowsError {}
impl Into<Error> for WindowsError {
    fn into(self) -> Error {
        Error::WindowsError(self)
    }
}
impl From<io::Error> for WindowsError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl ClipboardProvider for WindowsClipboardContext {
    fn new() -> Result<Self> {
        Ok(WindowsClipboardContext)
    }
    fn get_contents(&mut self) -> Result<String> {
        Ok(match get_clipboard_string() {
            Ok(c) => c,
            Err(e) => return Err(Error::WindowsError(WindowsError::IoError(e)))
        })
    }
    fn set_contents(&mut self, data: String) -> Result<()> {
        Ok(match set_clipboard_string(&data) {
            Ok(c) => c,
            Err(e) => return Err(Error::WindowsError(WindowsError::IoError(e)))
        })
    }
}

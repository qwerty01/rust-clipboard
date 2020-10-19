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

use crate::{Error, Result};
use common::ClipboardProvider;
use std::error;
use std::fmt;

pub struct NopClipboardContext;

#[derive(Debug)]
pub enum NopError {
    NopError,
}
impl fmt::Display for NopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NopError => write!(f, "This platform is not supported"),
        }
    }
}
impl error::Error for NopError {}
impl Into<Error> for NopError {
    fn into(self) -> Error {
        Error::NopError(self)
    }
}

impl ClipboardProvider for NopClipboardContext {
    fn new() -> Result<NopClipboardContext> {
        Ok(NopClipboardContext)
    }
    fn get_contents(&mut self) -> Result<String> {
        Err(Error::NopError(NopError::NopError))
    }
    fn set_contents(&mut self, _: String) -> Result<()> {
        Err(Error::NopError(NopError::NopError))
    }
}

/*
Copyright 2017 Avraham Weinstock

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

use std::error;
use std::fmt;
use std::time::Duration;
use std::marker::PhantomData;
use common::*;
use x11_clipboard_crate::Atoms;
use x11_clipboard_crate::Clipboard as X11Clipboard;
use x11_clipboard_crate::xcb::xproto::Atom;
use crate::{Error, Result};
use std::string::FromUtf8Error;

pub trait Selection {
    fn atom(atoms: &Atoms) -> Atom;
}

pub struct Primary;

impl Selection for Primary {
    fn atom(atoms: &Atoms) -> Atom {
        atoms.primary
    }
}

pub struct Clipboard;

impl Selection for Clipboard {
    fn atom(atoms: &Atoms) -> Atom {
        atoms.clipboard
    }
}

#[derive(Debug)]
pub enum X11Error {
    FromUtf8Error(FromUtf8Error),
    ClipboardError(x11_clipboard_crate::error::Error),
}
impl fmt::Display for X11Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FromUtf8Error(e) => e.fmt(f),
            Self::ClipboardError(e) => e.fmt(f),
        }
    }
}
impl Into<Error> for X11Error {
    fn into(self) -> Error {
        Error::X11Error(self)
    }
}
impl From<FromUtf8Error> for X11Error {
    fn from(e: FromUtf8Error) -> Self {
        Self::FromUtf8Error(e)
    }
}
impl From<x11_clipboard_crate::error::Error> for X11Error {
    fn from(e: x11_clipboard_crate::error::Error) -> Self {
        X11Error::ClipboardError(e)
    }
}

impl Into<Error> for FromUtf8Error> {
    fn into(self) -> Error {
        Error::X11Error(X11Error::FromUtf8Error(self))
    }
}
impl Into<Error> for x11_clipboard_crate::error::Error {
    fn into(self) -> Error {
        Error::X11Error(X11Error::ClipboardError(self))
    }
}

pub struct X11ClipboardContext<S = Clipboard>(X11Clipboard, PhantomData<S>)
where
    S: Selection;

impl<S> ClipboardProvider for X11ClipboardContext<S>
where
    S: Selection,
{
    fn new() -> Result<X11ClipboardContext<S>> {
        Ok(X11ClipboardContext(match X11Clipboard::new() {
            Ok(c) => c,
            Err(e) => return Err(Error::X11Error(X11Error::ClipboardError(e))),
        }, PhantomData))
    }

    fn get_contents(&mut self) -> Result<String> {
        Ok(match String::from_utf8(match self.0.load(
            S::atom(&self.0.getter.atoms),
            self.0.getter.atoms.utf8_string,
            self.0.getter.atoms.property,
            Duration::from_secs(3),
        ) {
            Ok(l) => l,
            Err(e) => return Err(Error::X11Error(X11Error::ClipboardError(e))),
        }) {
            Ok(s) => s,
            Err(e) => return Err(Error::X11Error(X11Error::FromUtf8Error(e))),
        })
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        Ok(match self.0.store(
            S::atom(&self.0.setter.atoms),
            self.0.setter.atoms.utf8_string,
            data,
        ) {
            Ok(s) => s,
            Err(e) => return Err(Error::X11Error(X11Error::ClipboardError(e))),
        })
    }
}

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
use common::*;
use objc::runtime::{Class, Object};
use objc_foundation::{INSArray, INSObject, INSString};
use objc_foundation::{NSArray, NSDictionary, NSObject, NSString};
use objc_id::{Id, Owned};
use std::error;
use std::fmt;
use std::mem::transmute;

pub struct OSXClipboardContext {
    pasteboard: Id<Object>,
}

// required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[derive(Debug)]
pub enum OSXError {
    PasteWriteObjectsError,
    ReadObjectsForClassesEmpty,
    ReadObjectsForClassesNull,
    PasteboardNotFound,
    NullPasteboard,
}
impl fmt::Display for OSXError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::PasteWriteObjectsError => "Could not paste objects to clipboard",
            Self::ReadObjectsForClassesEmpty => "Clipboard is empty",
            Self::ReadObjectsForClassesNull => "No objects to read",
            Self::PasteboardNotFound => "Pasteboard not found",
            Self::NullPasteboard => "General pasteboard not found",
        };
        write!(f, "{}", msg)
    }
}
impl error::Error for OSXError {}
impl Into<Error> for OSXError {
    fn into(self) -> Error {
        Error::OSXError(self)
    }
}

impl ClipboardProvider for OSXClipboardContext {
    fn new() -> Result<OSXClipboardContext> {
        let cls = try!(Class::get("NSPasteboard").ok_or(OSXError::PasteboardNotFound)?);
        let pasteboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };
        if pasteboard.is_null() {
            return Err(OSXError::NullPasteboard);
        }
        let pasteboard: Id<Object> = unsafe { Id::from_ptr(pasteboard) };
        Ok(OSXClipboardContext {
            pasteboard: pasteboard,
        })
    }
    fn get_contents(&mut self) -> Result<String> {
        let string_class: Id<NSObject> = {
            let cls: Id<Class> = unsafe { Id::from_ptr(class("NSString")) };
            unsafe { transmute(cls) }
        };
        let classes: Id<NSArray<NSObject, Owned>> = NSArray::from_vec(vec![string_class]);
        let options: Id<NSDictionary<NSObject, NSObject>> = NSDictionary::new();
        let string_array: Id<NSArray<NSString>> = unsafe {
            let obj: *mut NSArray<NSString> =
                msg_send![self.pasteboard, readObjectsForClasses:&*classes options:&*options];
            if obj.is_null() {
                return Err(OSXError::ReadObjectsForClassesNull);
            }
            Id::from_ptr(obj)
        };
        if string_array.count() == 0 {
            Err(OSXError::ReadObjectsForClassesEmpty)
        } else {
            Ok(string_array[0].as_str().to_owned())
        }
    }
    fn set_contents(&mut self, data: String) -> Result<()> {
        let string_array = NSArray::from_vec(vec![NSString::from_str(&data)]);
        let _: usize = unsafe { msg_send![self.pasteboard, clearContents] };
        let success: bool = unsafe { msg_send![self.pasteboard, writeObjects: string_array] };
        return if success {
            Ok(())
        } else {
            Err(OSXError::PasteWriteObjectsError)
        };
    }
}

// this is a convenience function that both cocoa-rs and
//  glutin define, which seems to depend on the fact that
//  Option::None has the same representation as a null pointer
#[inline]
pub fn class(name: &str) -> *mut Class {
    unsafe { transmute(Class::get(name)) }
}

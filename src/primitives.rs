use std::ops::Deref;

use crate::isolate::Isolate;
use crate::support::Opaque;
use crate::Local;
use crate::Value;

/// The superclass of primitive values.  See ECMA-262 4.3.2.
#[repr(C)]
pub struct Primitive(Opaque);

/// A primitive boolean value (ECMA-262, 4.3.14).  Either the true
/// or false value.
#[repr(C)]
pub struct Boolean(Opaque);

/// A superclass for symbols and strings.
#[repr(C)]
pub struct Name(Opaque);

extern "C" {
  fn v8__Null(isolate: &Isolate) -> *mut Primitive;

  fn v8__Undefined(isolate: &Isolate) -> *mut Primitive;

  fn v8__True(isolate: &Isolate) -> *mut Boolean;

  fn v8__False(isolate: &Isolate) -> *mut Boolean;
}

pub fn new_null(isolate: &Isolate) -> Local<Primitive> {
  unsafe { Local::from_raw(v8__Null(isolate)) }.unwrap()
}

pub fn new_undefined(isolate: &Isolate) -> Local<Primitive> {
  unsafe { Local::from_raw(v8__Undefined(isolate)) }.unwrap()
}

pub fn new_true(isolate: &Isolate) -> Local<Boolean> {
  unsafe { Local::from_raw(v8__True(isolate)) }.unwrap()
}

pub fn new_false(isolate: &Isolate) -> Local<Boolean> {
  unsafe { Local::from_raw(v8__False(isolate)) }.unwrap()
}

impl Deref for Primitive {
  type Target = Value;
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const _ as *const Value) }
  }
}

impl Deref for Boolean {
  type Target = Primitive;
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const _ as *const Primitive) }
  }
}

impl Deref for Name {
  type Target = Primitive;
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const _ as *const Primitive) }
  }
}
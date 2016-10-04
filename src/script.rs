use v8_sys as v8;

use context;
use error;
use isolate;
use value;
use util;

/// A compiled JavaScript script, tied to a Context which was active when the script was compiled.
#[derive(Debug)]
pub struct Script<'a>(&'a isolate::Isolate, v8::ScriptRef);

impl<'a> Script<'a> {
    pub fn compile(isolate: &'a isolate::Isolate,
                   context: &context::Context,
                   source: &value::String)
                   -> error::Result<Script<'a>> {
        let raw = unsafe {
            try!(util::invoke(isolate,
                              |c| v8::Script_Compile(c, context.as_raw(), source.as_raw())))
        };
        Ok(Script(isolate, raw))
    }

    pub fn compile_with_name(isolate: &'a isolate::Isolate,
                             context: &context::Context,
                             name: &value::Value,
                             source: &value::String)
                             -> error::Result<Script<'a>> {
        use std::ptr::null_mut as n;
        let raw = unsafe {
            try!(util::invoke(isolate, |c| {
                v8::Script_Compile_Origin(c,
                                          context.as_raw(),
                                          source.as_raw(),
                                          name.as_raw(),
                                          n(),
                                          n(),
                                          n(),
                                          n(),
                                          n(),
                                          n(),
                                          n())
            }))
        };
        Ok(Script(isolate, raw))
    }

    pub fn run(&self, context: &context::Context) -> error::Result<value::Value<'a>> {
        unsafe {
            let raw = try!(util::invoke(self.0, |c| v8::Script_Run(c, self.1, context.as_raw())));
            Ok(value::Value::from_raw(self.0, raw))
        }
    }
}

impl<'a> Drop for Script<'a> {
    fn drop(&mut self) {
        unsafe { v8::Script_DestroyRef(self.1) }
    }
}
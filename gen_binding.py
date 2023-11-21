from doxybindgen.model import Class, ClassManager, pascal_to_snake
from doxybindgen.binding import CxxClassBinding, RustClassBinding

from itertools import chain

import string
import subprocess
import toml


# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    with open('Doxybindgen.toml', 'r') as f:
        config = toml.load(f)
    
    classes = ClassManager()
    parsed = []
    xmlfiles = config['wxml_files']
    progress('Parsing')
    for file in xmlfiles:
        progress('.')
        for cls in Class.in_xml(classes, file, config['types']):
            parsed.append(cls)
    # Register all classes once parsing finished.
    classes.register(parsed)

    progress('\nGenerating')
    with open('docs/OverloadTree.md', 'w') as overload_tree_md:
        print('''\
# Overload Method Name Decision Tree
''', file=overload_tree_md)
        generate_library(classes, config, overload_tree_md)


generated = []
def generate_library(classes, config, overload_tree_md):
    files_per_initial = {
        'src/generated/ffi_%s.rs': ffi_i_rs,
        'src/generated/methods_%s.rs': methods_i_rs,
        'src/generated/class_%s.rs': class_i_rs,
        'include/generated/ffi_%s.h': ffi_i_h,
        'src/generated/ffi_%s.cpp': ffi_i_cpp,
    }
    rust_bindings = [RustClassBinding(cls, overload_tree_md) for cls in classes.all()]
    cxx_bindings = [CxxClassBinding(cls, config) for cls in classes.all()]
    initials = []
    for initial in string.ascii_lowercase:
        rust_bindings_i = [b for b in rust_bindings if b.has_initial(initial)]
        if len(rust_bindings_i) == 0:
            continue
        initials.append(initial)
        cxx_bindings_i = [c for c in cxx_bindings if c.has_initial(initial)]
        for path, generator in files_per_initial.items():
            progress('.')
            path = path % (initial,)
            is_rust = path.endswith('.rs')
            with open(path, 'w', newline='\n', encoding='utf-8') as f:
                for chunk in generator(
                    rust_bindings_i if is_rust else cxx_bindings_i
                ):
                    print(chunk, file=f)
            # if is_rust:
            #     error = subprocess.check_output(['rustfmt', path])
            #     if error:
            #         print(error)
    to_be_generated = {
        'src/generated/class.rs': class_rs,
        'src/generated/ffi.rs': ffi_rs,
        'src/generated/methods.rs': methods_rs,
        'src/generated.rs': generated_rs,
        'include/generated.h': generated_h,
        'src/generated.cpp': generated_cpp,
    }
    for path, generator in to_be_generated.items():
        progress('.')
        is_rust = path.endswith('.rs')
        with open(path, 'w', newline='\n', encoding='utf-8') as f:
            for chunk in generator(
                initials,
            ):
                print(chunk, file=f)

def progress(s):
    print(s, end='', flush=True)

def ffi_i_rs(classes):
    yield '''\
use super::*;

extern "C" {'''
    indent = ' ' * 4 * 1
    for cls in classes:
        for line in cls.lines(for_ffi=True):
            if not line:
                yield ''
            else:
                yield '%s%s' % (indent, line)
    yield '''\

}\
'''

def methods_i_rs(classes):
    yield '''\
use super::*;
'''
    for cls in classes:
        for line in cls.lines(for_methods=True):
            yield line

def class_i_rs(classes):
    yield '''\
use super::*;
'''
    for cls in classes:
        for line in cls.lines():
            yield line


def ffi_i_h(classes):
    yield '''\
#pragma once
'''
    uniq = set()
    for cls in classes:
        uniq.add(cls.include())
    for (include, condition) in sorted(uniq):
        if condition:
            yield condition
        yield include
        if condition:
            yield '#endif'
    yield '''\

extern "C" {
'''
    for cls in classes:
        for line in cls.lines():
            yield line
    yield '''\
} // extern "C"
'''

def ffi_i_cpp(classes):
    yield '''\
#include "generated.h"

extern "C" {
'''
    for cls in classes:
        for line in cls.lines(is_cc=True):
            yield line
    yield '''\
} // extern "C"
'''

def class_rs(initials):
    yield '''\
use super::*;
'''
    for i in initials:
        yield 'pub use class_%s::*;' % (i,)

def ffi_rs(initials):
    yield '''\
pub use crate::ffi::*;
'''
    for i in initials:
        yield 'pub use super::ffi_%s::*;' % (i,)

def methods_rs(initials):
    yield '''\
use std::os::raw::c_void;

pub trait RustBindingMethods {
    type CppManaged;
    unsafe fn as_ptr(&self) -> *mut c_void;
    unsafe fn from_ptr(ptr: *mut c_void) -> Self;
    unsafe fn from_cpp_managed_ptr(ptr: *mut c_void) -> Self::CppManaged;
    unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F);
    unsafe fn option_from(ptr: *mut c_void) -> Option<Self::CppManaged>
    where
        Self: Sized,
    {
        if ptr.is_null() {
            None
        } else {
            Some(Self::from_cpp_managed_ptr(ptr))
        }
    }
}
'''
    for i in initials:
        yield 'pub use super::methods_%s::*;' % (i,)

def generated_rs(initials):
    yield '''\
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

use super::*;
use methods::*;
'''
    yield 'mod ffi;'
    for i in initials:
        yield 'mod ffi_%s;' % (i,)
    yield ''
    yield 'pub mod methods;'
    for i in initials:
        yield 'mod methods_%s;' % (i,)
    yield ''
    yield 'pub mod class;'
    for i in initials:
        yield 'mod class_%s;' % (i,)


def generated_h(initials):
    yield '''\
#pragma once

// TODO: include required headers
'''
    for i in initials:
        yield '#include "generated/ffi_%s.h"' % (i,)

def generated_cpp(initials):
    yield '''\
#include "generated.h"

// Including splitted source files into single source file to keep build.rs simple\
'''
    for i in initials:
        yield '#include "generated/ffi_%s.cpp"' % (i,)

if __name__ == '__main__':
    main()

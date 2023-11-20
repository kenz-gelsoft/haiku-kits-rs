from doxybindgen.constants import generate_constants_in, has_constants

import os
import subprocess
import xml.etree.ElementTree as ET

PROLOGUE = '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

// FIXME: workaround for windows (LLP64)
#![allow(overflowing_literals)]

use std::os::raw::{c_int, c_long, c_uint};

//use crate::manual::*;

pub const _VIEW_TOP_: c_int = 1;
pub const _VIEW_LEFT_: c_int = 2;
pub const _VIEW_BOTTOM_: c_int = 3;
pub const _VIEW_RIGHT_: c_int = 4;
pub const _VIEW_CENTER_: c_int = 5;
pub const B_BEOS_VERSION: c_int = 0x0500;
pub const INT_MIN: c_int = -2147483647;
pub const MAXPATHLEN: c_int = 1024;
pub const NAME_MAX: c_int = 256;
pub const O_APPEND: c_int = 0x00000800;
pub const O_RDONLY: c_int = 00;
pub const O_WRONLY: c_int = 01;
pub const O_RDWR: c_int = 02;
pub const O_EXCL: c_int = 0x0100;
pub const O_CREAT: c_int = 0x0200;
pub const O_TRUNC: c_int = 0x0400;
pub const SYMLOOP_MAX: c_int = 16;
'''

# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    outpath = 'src/constants.rs'
    with open(outpath, 'w', newline='\n') as f:
        print(PROLOGUE, file=f)
        for file in xml_files_in('bxml/'):
            # print(file)
            tree = ET.parse(file)
            root = tree.getroot()
            for line in generate_constants_in(root):
                print(line, file=f)
    print(subprocess.check_output(['rustfmt', outpath]))

def xml_files_in(dir):
    index = os.path.join(dir, 'index.xml')
    with open(index, 'r') as f:
        tree = ET.parse(f)
        root = tree.getroot()
        for compound in root.findall('./compound'):
            if has_constants(compound):
                xml = compound.get('refid') + '.xml'
                yield os.path.join(dir, xml)

if __name__ == '__main__':
    main()


from doxybindgen.model import Class, ClassManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

import os
import toml


# place Haiku doxygen xml files in bxml/ dir and run this.
def main():
    with open('Doxybindgen.toml', 'r') as f:
        config = toml.load(f)
    
    classes = ClassManager()
    parsed = []
    for (file, path) in bxml_files():
        for cls in Class.in_xml(classes, path, config['types']):
            cls.file = file
            parsed.append(cls)
    classes.register(parsed)
    
    # print_classes_in_lib(classes, config, 'base')
    print_classes_in_lib(classes, config, 'core')

def bxml_files():
    for root, dirs, files in os.walk('bxml'):
        for file in files:
            if (file.startswith('classwx_') and
                file.endswith('.xml')):
                yield (file, os.path.join(root, file))

generated = []
def print_classes_in_lib(classes, config, libname):
    # print('%s:' % (libname,))
    n = 0
    for file in sorted(cls.file for cls in classes.in_lib(libname, generated)):
        print("    'bxml/%s'," % (file,))
        n += 1
    # print('\t%s classes.' % (n))

if __name__ == '__main__':
    main()

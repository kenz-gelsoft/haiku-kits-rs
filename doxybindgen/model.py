import xml.etree.ElementTree as ET
import re

CXX2CXX = {
    'long': 'int32_t',
}

CXX2RUST = {
    'double': 'f64',
    'int': 'i32',
    'long': 'i32',
    'unsigned int': 'u32',
    'wxByte': 'u8',
    'wxCoord': 'i32',
    'wxWindowID': 'i32',
}

class Class:
    def in_xml(xmlfile, blocklist):
        tree = ET.parse(xmlfile)
        root = tree.getroot()
        for cls in root.findall(".//compounddef[@kind='class']"):
            yield Class(cls, blocklist)

    def __init__(self, e, blocklist):
        self.name = e.findtext('compoundname')
        self.methods = []
        self.__blocklist = blocklist.get(self.name)
        for method in e.findall(".//memberdef[@kind='function']"):
            m = Method(self, method)
            if not m.is_public:
                continue
            self.methods.append(m)

    def unprefixed(self):
        return self.name[2:]

    def blocks(self, name):
        if self.__blocklist is None:
            return False
        return name in self.__blocklist

class Method:
    def __init__(self, cls, e):
        self.is_public = e.get('prot') == 'public'
        self.is_static = e.get('static') == 'yes'
        self.returns = CxxType(e.find('type'))
        self.cls = cls
        self.name = e.findtext('name')
        self.overload_index = self._overload_index()
        self.is_ctor = self.name == cls.name
        self.const = e.get('const') == 'yes'
        if self.is_ctor:
            self.returns = SelfType(cls.name, self.const, ctor_retval=True)
        self.params = []
        for param in e.findall('param'):
            ptype = CxxType(param.find('type'))
            pname = param.findtext('declname')
            self.params.append(Param(ptype, pname))

    def _overload_index(self):
        return sum(m.name == self.name for m in self.cls.methods)

    def call_params(self):
        return ', '.join(p.name for p in self.params)

    def overload_name(self):
        name = self.name
        if self.is_ctor:
            name = 'New%s' % (self.cls.unprefixed(),)
        index = self.overload_index
        if self.overload_index == 0:
            index = ''
        return '%s%s' % (name, index)
    
class Param:
    def __init__(self, type, name):
        self.type = type
        self.name = name
    
    def is_self(self):
        return self.name == 'self'

class SelfType:
    def __init__(self, s, const, ctor_retval=False):
        self.type = s
        self.const = const
        self.__ctor_retval = ctor_retval

    def in_rust(self, with_ffi=False):
        t = self.type
        if self.__ctor_retval:
            return t[2:]
        t = prefixed(t, with_ffi)
        if self.const:
            return '&%s' % (t)
        return 'Pin<&mut %s>' % (t,)
    
    def not_supported(self):
        return False
    
    def is_void(self):
        return False

OS_UNSUPPORTED_TYPES = [
    'wxAccessible',
]
CXX_SUPPORTED_VALUE_TYPES = [
    'bool',
    'void',
]
class CxxType:
    def __init__(self, e):
        self.__srctype = ''.join(e.itertext())
        # print('parsing: |%s|' % (s,))
        matched = re.match(r'(const )?([^*&]*)([*&]+)?', self.__srctype)
        self.__typename = None
        if matched:
            self.__is_mut = matched.group(1) is None
            self.__typename = matched.group(2).strip()
            self.__indirection = matched.group(3)
        if self.__indirection is None:
            self.__indirection = ''
    
    def in_cxx(self):
        if self.__srctype in CXX2CXX:
            return CXX2CXX[self.__srctype]
        return self.__srctype

    def in_rust(self, with_ffi=False):
        t = self.__typename
        if t in CXX2RUST:
            t = CXX2RUST[t]
        t = prefixed(t, with_ffi)
        ref = self.__indirection
        mut = ''
        if ref:
            mut = 'mut ' if self.__is_mut else ''
            if ref.startswith('*') and not self.__is_mut:
                mut = 'const '
        if ref.startswith('&') and self.__is_mut:
            return 'Pin<&mut %s>' % (t,)
        return '%s%s%s' % (ref, mut, t)
    
    def not_supported(self):
        if self.__typename in OS_UNSUPPORTED_TYPES:
            return True
        if not self._is_cxx_supported_value_type():
            return not self.__indirection
        return False
    
    def _is_cxx_supported_value_type(self):
        if self.__typename in CXX_SUPPORTED_VALUE_TYPES:
            return True
        if self.__typename in CXX2RUST:
            return True
        return False
    
    def is_ptr(self):
        return self.__indirection.startswith('*')
    
    def is_void(self):
        if self.is_ptr():
            return False
        return self.__typename == 'void'

RUST_PRIMITIVES = [
    'bool',
    'f64',
    'i32',
    'i64',
    'u8',
]
def prefixed(t, with_ffi=False):
    if t in RUST_PRIMITIVES:
        return t
    elif with_ffi:
        t = 'ffi::%s' % (t,)
    return t

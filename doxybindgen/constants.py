import re
import xml.etree.ElementTree as ET


# MEMO: don't replace `wx` prefix of `wx_GL_COMPAT_PROFILE`
RE_IDENT = re.compile(r'wx([^_]\w)')

RE_ENUM_INITALIZER = re.compile(r'=\s+(.*)')
RE_BYTES_LITERAL = re.compile(r"'([^']+)'")

RE_LONG_SUFFIX = re.compile(r'(\d+)[Ll]')
RE_UINT_SUFFIX = re.compile(r'(\d+)[uU]')
RE_FLOAT_SUFFIX = re.compile(r'(\d+\.\d+)f')

def generate_constants_in(element):
    empty = True
    for define in defines_in(element):
        empty = False
        for line in generate_define(define):
            yield line

    for enum in enums_in(element):
        empty = False
        for line in generate_enum(enum):
            yield line
    
    if not empty:
        yield ''

def has_constants(compound):
    kind = compound.get('kind')
    if kind in ['class', 'struct']:
        return False
    for member in compound.findall("./member"):
        kind = member.get('kind')
        if kind in ['define', 'enum']:
            return True
    return False

def defines_in(root):
    memberdefs = root.findall(".//memberdef[@kind='define']")
    for memberdef in memberdefs:
        yield memberdef

def enums_in(root):
    memberdefs = root.findall(".//memberdef[@kind='enum']")
    for memberdef in memberdefs:
        yield memberdef

typedefs = [
]
blocklist = [
    # out of range hex escape: TODO: support non-ASCII 0x escape
    'B_UTF8_BULLET',
    'B_UTF8_CLOSE_QUOTE',
    'B_UTF8_COPYRIGHT',
    'B_UTF8_ELLIPSIS',
    'B_UTF8_HIROSHI',
    'B_UTF8_OPEN_QUOTE',
    'B_UTF8_REGISTERED',
    'B_UTF8_SMILING_FACE',
    'B_UTF8_TRADEMARK',

    # non-trivial-object
    'B_CATALOG', # BLocaleRoster::Default()->GetCatalog()
    'B_TRANSLATE',
    'B_TRANSLATE_ALL',
    'B_TRANSLATE_COMMENT',
    'B_TRANSLATE_CONTEXT',
    'B_TRANSLATE_ID',
    'B_TRANSLATE_MARK',
    'B_TRANSLATE_MARK_ALL',
    'B_TRANSLATE_MARK_COMMENT',
    'B_TRANSLATE_MARK_CONTEXT',
    'B_TRANSLATE_MARK_ID',
    'B_TRANSLATE_MARK_SYSTEM_NAME',
    'B_TRANSLATE_NOCOLLECT',
    'B_TRANSLATE_NOCOLLECT_ALL',
    'B_TRANSLATE_NOCOLLECT_COMMENT',
    'B_TRANSLATE_NOCOLLECT_ID',
    'B_TRANSLATE_NOCOLLECT_SYSTEM_NAME',
    'B_TRANSLATE_SYSTEM_NAME',
    'B_TRANSLATION_SYSTEM_NAME_CONTEXT',
]
generated = set()
class Define:
    def __init__(self, e):
        self.name = e.findtext('name')
        initializer = e.find('initializer')
        self.__initializer = initializer.itertext() if initializer is not None else None

    def blocked_reason(self):
        if self.__initializer is None:
            return 'NODEF'

        if self.name in generated:
            return '  DUP'
        
        if (self.name in blocklist or
            self.name in typedefs):
            return ' SKIP'
        
        return None

    def __str__(self):
        name = self.name
        v = ''.join(self.__initializer)
        v = ''.join(map(lambda s: s.lstrip(), v.split('\\\n')))
        (t, v) = translate_initializer(name, v)
        name = RE_IDENT.sub(r'\1', name)
        return 'pub const %s: %s = %s;' % (name, t, v)

def translate_initializer(name, v):
    t = 'c_int'
    has_long_suffix = RE_LONG_SUFFIX.search(v)
    has_uint_suffix = RE_UINT_SUFFIX.search(v)
    has_float_suffix = RE_FLOAT_SUFFIX.search(v)
    if name in long_types or has_long_suffix:
        t = 'c_long'
    elif has_uint_suffix:
        t = 'c_uint'
    elif v == 'true' or v == 'false':
        t = 'bool'
    elif has_float_suffix:
        t = 'f32'
    elif '"' in v:
        t = '&str'
    elif "'" in v:
        (t, v) = bytes_literal(t, v)
    v = RE_LONG_SUFFIX.sub(r'\1', v)
    v = RE_UINT_SUFFIX.sub(r'\1', v)
    v = RE_FLOAT_SUFFIX.sub(r'\1', v)
    # TODO: string types
    v = re.sub(r'wxString\((".+")\)', r'\1', v)
    v = re.sub(r'wxS\((".+")\)', r'\1', v)
    v = re.sub(r'wxT\((".+")\)', r'\1', v)
    # Don't strip `wx` prefix of string literal (c.f. IMAGE_OPTION_BMP_FORMAT)
    if '"' not in v:
        v = RE_IDENT.sub(r'\1', v)
    return (t, v)

def generate_define(e):
    d = Define(e)
    name = d.name
    blocked = d.blocked_reason()
    if blocked is not None:
        yield '// %s: %s' % (blocked, name)
        return
    generated.add(name)
    yield d

long_types = [
]

class Enum:
    def __init__(self, e):
        self.name = e.findtext('name')
        self.__current_initializer = '= 0'
        self.__count = 0
        self.__values = []
        for v in e.findall('enumvalue'):
            v = EnumValue(self, v)
            self._add_value(v)
    
    def _add_value(self, v):
        if v.initializer is None:
            if self.__count:
                v.initializer = '%s + %s' % (
                    self.__current_initializer,
                    self.__count,
                )
            else:
                v.initializer = self.__current_initializer
            self.__count += 1
        else:
            self.__current_initializer = v.initializer
            self.__count = 1
        self.__values.append(v)
    
    def generate(self):
        yield '//  ENUM: %s' % (self.name,)
        for v in self.__values:
            blocked = v.blocked_reason()
            if blocked is not None:
                yield '// %s: %s' % (
                    blocked,
                    v.name,
                )
                continue
            generated.add(v.name)
            yield v

class EnumValue:
    def __init__(self, enum, e):
        self.__enum = enum
        self.name = e.findtext('name')
        self.initializer = e.findtext('initializer')
    
    def blocked_reason(self):
        if self.name in generated:
            return '  DUP'
        
        if (self.name in blocklist or
            self.name in typedefs):
            return ' SKIP'
        
        return None
    
    def __str__(self):
        v = RE_ENUM_INITALIZER.match(self.initializer).group(1).strip()
        v = v.replace('~', '!') # special replacement for negative unsigned value
        (t, v) = translate_initializer(self.__enum.name, v)
        self.name = RE_IDENT.sub(r'\1', self.name)
        return 'pub const %s: %s = %s;' % (
            self.name,
            t,
            v,
        )

def bytes_literal(t, v):
    byte_count = len(v) - 2 # quotes
    if byte_count == 1:
        t = 'char'
    elif byte_count == 2:
        t = 'c_short'
    elif byte_count == 4:
        t = 'c_int'
    v = RE_BYTES_LITERAL.sub(bytes_to_int, v)
    return (t, v)

def bytes_to_int(matched):
    i = 0
    s = matched.group(1)
    for c in s:
        i <<= 8
        i += ord(c)
    return "%s /* '%s' */" % (hex(i), s)

def generate_enum(e):
    enum = Enum(e)
    for line in enum.generate():
        yield line


import re
import xml.etree.ElementTree as ET


# MEMO: don't replace `wx` prefix of `wx_GL_COMPAT_PROFILE`
RE_IDENT = re.compile(r'wx([^_]\w)')

RE_ENUM_INITALIZER = re.compile(r'=\s+(.*)')
RE_BYTES_LITERAL = re.compile(r"'([^']+)'")
RE_INT_CAST = re.compile(r'\(int\)(.*)')
RE_UINT_CAST = re.compile(r'\(\(uint32\)(.*)\)')

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
    
    # i64
    'B_ALIGN_NO_VERTICAL',

    # non-trivial-object
    'B_CATALOG', # BLocaleRoster::Default()->GetCatalog()

    # String concat
    'B_PRId32',
    'B_PRId64',
    'B_PRIi32',
    'B_PRIi64',
    'B_PRIo32',
    'B_PRIo64',
    'B_PRIu32',
    'B_PRIu64',
    'B_PRIx32',
    'B_PRIX32',
    'B_PRIx64',
    'B_PRIX64',
    'B_SCNd32',
    'B_SCNd64',
    'B_SCNi32',
    'B_SCNi64',
    'B_SCNo32',
    'B_SCNo64',
    'B_SCNu32',
    'B_SCNu64',
    'B_SCNx32',
    'B_SCNx64',
    'B_PRIdBIGTIME',
    'B_PRIdDEV',
    'B_PRIdINO',
    'B_PRIdOFF',
    'B_PRIdSSIZE',
    'B_PRIdTIME',
    'B_PRIiBIGTIME',
    'B_PRIiDEV',
    'B_PRIiINO',
    'B_PRIiOFF',
    'B_PRIiSSIZE',
    'B_PRIiTIME',
    'B_PRIoADDR',
    'B_PRIoGENADDR',
    'B_PRIoPHYSADDR',
    'B_PRIoSIZE',
    'B_PRIuADDR',
    'B_PRIuGENADDR',
    'B_PRIuPHYSADDR',
    'B_PRIuSIZE',
    'B_PRIxADDR',
    'B_PRIXADDR',
    'B_PRIxGENADDR',
    'B_PRIXGENADDR',
    'B_PRIxOFF',
    'B_PRIxPHYSADDR',
    'B_PRIXPHYSADDR',
    'B_PRIxSIZE',
    'B_PRIXSIZE',
    'B_SCNdOFF',
    'B_SCNdSSIZE',
    'B_SCNiOFF',
    'B_SCNiSSIZE',
    'B_SCNoADDR',
    'B_SCNoGENADDR',
    'B_SCNoPHYSADDR',
    'B_SCNoSIZE',
    'B_SCNuADDR',
    'B_SCNuGENADDR',
    'B_SCNuPHYSADDR',
    'B_SCNuSIZE',
    'B_SCNxADDR',
    'B_SCNxGENADDR',
    'B_SCNxOFF',
    'B_SCNxPHYSADDR',
    'B_SCNxSIZE',
    
    # C++ namespace alias
    'U_ICU_NAMESPACE',
]
generated = set()
class Define:
    def __init__(self, e):
        self.name = e.findtext('name')
        initializer = e.find('initializer')
        self.__is_func = e.find('param') is not None
        self.__initializer = initializer.itertext() if initializer is not None else None

    def blocked_reason(self):
        if self.__initializer is None:
            return 'NODEF'
        
        if self.__is_func:
            return ' FUNC'

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
    # Expand some function style macros
    # - These macros do nothing under normal circumstance.
    v = re.sub(r'B_TO_POSIX_ERROR\((.+)\)', r'(\1)', v)
    v = re.sub(r'B_FROM_POSIX_ERROR\((.+)\)', r'\1', v)
    # - Simple computation
    v = re.sub(r'B_MOUSE_BUTTON\((.+)\)', r'(1 << ((\1) - 1))', v)
    # - Not a macro, but inline function that simple calculation
    v = re.sub(r'_rule_\((.+),(.+),(.+),(.+)\)', r'(((\1) << 12) | ((\2) << 8) | ((\3) << 4) | (\4))', v)

    t = 'c_int'
    has_long_suffix = RE_LONG_SUFFIX.search(v)
    has_uint_suffix = RE_UINT_SUFFIX.search(v)
    has_uint_cast = RE_UINT_CAST.search(v)
    has_float_suffix = RE_FLOAT_SUFFIX.search(v)
    if name in long_types or has_long_suffix:
        t = 'c_long'
        v = RE_LONG_SUFFIX.sub(r'\1', v)
    elif has_uint_suffix or has_uint_cast:
        t = 'c_uint'
        v = RE_UINT_SUFFIX.sub(r'\1', v)
        v = RE_UINT_CAST.sub(r'\1', v)
        if '-' in v:
            v = hex(2**64 + int(v))
    elif v == 'true' or v == 'false':
        t = 'bool'
    elif has_float_suffix:
        t = 'f32'
        v = RE_FLOAT_SUFFIX.sub(r'\1', v)
    elif '"' in v:
        t = '&str'
    elif "'" in v:
        (t, v) = bytes_literal(t, v)
    v = RE_INT_CAST.sub(r'\1', v) # remove int cast
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
        return (t, v)
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


#!/usr/bin/env python3
import subprocess
import os
from typing import List, Tuple, Optional

import regex

INCLUDES = [
    "extern/CrabNet/include/raknet",
    "extern/tes3mp/apps/openmw-mp",
    "extern/tes3mp"
]


def normalize_var(var: str):
    normalized = ''.join([char if char.islower() else ('_' + char.lower()) for char in var]).lstrip('_') \
        .replace('a_i', 'ai') \
        .replace('s_h_a_2_5_6', 'sha256') \
        .replace('i_p', 'ip')

    if normalized == 'type':
        return '_type'

    return normalized


class CPPFunction:
    def __init__(self, name: str, args: List[Tuple[str, str]], return_type: str, comment: Optional[str]):
        self.name = name.strip()
        self.args = args
        self.return_type = return_type
        self.comment = comment

    def __str__(self):
        args = ', '.join(map(str, self.args))
        return f'{self.comment}\n<CPPFunction "{self.name}" ({args}) -> {self.return_type}>'


class CPPClass:
    def __init__(self, name: str):
        self.name = name
        self.functions = dict()

    def __str__(self):
        funcs = '\n\t'.join(map(str, self.functions)).lstrip()
        return f'<CPPClass "{self.name}" functions=[\n\t{funcs}\n]>'


class PrimitiveHeaderParser:
    STATE_NONE = 0
    STATE_FUNC_LIST = 1

    RE_CLASS = regex.compile(r'^\s*class\s+(\w+)')
    RE_START_COMMENT = regex.compile(r'\s*\/\*\*')
    RE_END_COMMENT = regex.compile(r'.+\*\/')
    RE_FUNCTION = regex.compile(r'^\s*static\s+((?:[a-zA-Z_\:\<\>\*\&0-9]+\s+[\*\&]?)+)([a-zA-Z0-9_]+)\(([^\)]+)?\)')
    RE_ARG = regex.compile(r'^\s*([a-zA-Z_\:\<\>\s\*\&0-9\.]+?)(?:(\s[\.\*\&]*)([a-zA-Z0-9_]+))?$')
    RE_FUNC_PAIR = regex.compile(r'{\s*"([A-Za-z0-9_\s]+)"\s*,\s*([A-Za-z0-9]+)\:\:([A-Za-z0-9]+)\s*}')

    def __init__(self):
        self.classes = dict()
        self.current_class = None
        self.current_comment = None
        self.last_comment = None
        self.line = -1
        self.state = self.STATE_NONE
        self.functions = dict()
        self.current_line = ""

    def parse_line(self, line: str):
        self.line += 1
        if line.rstrip().endswith(",") and self.current_comment is None:
            self.current_line += " " + line.rstrip()
            return

        line = self.current_line + line
        self.current_line = ""

        last_comment = self.last_comment
        self.last_comment = None

        if self.state == self.STATE_FUNC_LIST:
            self.parse_function_list(line)
            return

        new_class = self.RE_CLASS.match(line)
        if new_class is not None:
            self.current_class = CPPClass(new_class[1])
            self.classes[self.current_class.name] = self.current_class

        if self.current_comment is None:
            new_comment = self.RE_START_COMMENT.match(line)
            if new_comment is not None:
                self.current_comment = new_comment[0]
        else:
            self.current_comment += "\n" + line
            if self.RE_END_COMMENT.match(line) is not None:
                self.last_comment = self.current_comment
                self.current_comment = None

            return

        new_function = self.RE_FUNCTION.match(line)
        if new_function is not None:
            failed_func = False
            args = []
            if new_function[3] is not None:
                for arg in new_function[3].split(','):
                    arg = arg.split('=')[0].strip()

                    if arg in ['void', '...']:
                        args.append(('', arg))
                        continue

                    argr = self.RE_ARG.match(arg)
                    if argr is None:
                        print(f"Can't parse: {arg}")
                        failed_func = True
                        break
                    else:
                        name = "" if argr[3] is None else argr[3]
                        type = "" if argr[1] is None else argr[1]
                        pointer = "" if argr[2] is None else argr[2]

                        args.append((name.strip(), type.strip() + pointer.strip()))

            if not failed_func:
                self.current_class.functions[new_function[2].strip()] = CPPFunction(new_function[2].strip(), args,
                                                                                    new_function[1].strip(),
                                                                                    last_comment)

        if line.lstrip().startswith('static constexpr ScriptFunctionData functions'):
            self.state = self.STATE_FUNC_LIST

    def parse_function_list(self, line):
        if line.strip().startswith('};'):
            self.state = self.STATE_NONE
            return

        for item in regex.findall(self.RE_FUNC_PAIR, line):
            self.functions[item[0]] = (item[1], item[2])


TYPE_TRANSLATION = {
    'const char*': '*const c_char',
    'const char *': '*const c_char',
    'bool': 'bool',
    'unsigned short': 'c_ushort',
    'ScriptFunc': 'fn()',
    'int': 'c_int',
    'unsigned int': 'c_uint',
    'unsigned char': 'c_uchar',
    'char': 'c_char',
    'float': 'c_float',
    'double': 'c_double',
    # No good way with these 2 types yet
    'va_list': '<REMOVE>',
    'boost::any': '<REMOVE>'
}

PREFIX = 'rust'

RE_COMMENT_PREFIX = regex.compile(r'^([\\/\*]*)(.*)')
RE_PARAM_PREFIX = regex.compile(r'[\\@]param ([a-zA-Z_-]+)')
RE_BRIEF = regex.compile(r'\\brief\s+')
RE_RETURN = regex.compile(r'\s+[\\@]returns? ([a-zA-Z])')


def main():
    os.chdir(os.path.dirname(__file__))
    gcc_command = list(["gcc", "-C", "-E"])

    for include in INCLUDES:
        gcc_command.append('-I')
        gcc_command.append(include)

    gcc_command.append("extern/tes3mp/apps/openmw-mp/Script/ScriptFunctions.hpp")

    parser = PrimitiveHeaderParser()

    with subprocess.Popen(gcc_command, stdout=subprocess.PIPE) as p:
        out, err = p.communicate()
        for line in out.splitlines():
            parser.parse_line(line.decode())

    raw = f"#[no_mangle]\npub static mut prefix: [u8; {len(PREFIX)}] = *b\"{PREFIX}\";\n"

    fancy = ""

    for func_name in parser.functions:
        ref = parser.functions[func_name]
        func = parser.classes[ref[0]].functions[ref[1]]

        args = ', '.join([TYPE_TRANSLATION[arg[1]] for arg in func.args])
        ret = ""

        if func.return_type in ['const char*', 'const char *']:
            ret = " -> *const c_char"
        elif func.return_type != "void":
            ret = f" -> {TYPE_TRANSLATION[func.return_type]}"

        place_holder = "|" + ', '.join(['_'] * len(func.args)) + f'| {{ unreachable!("{func_name} was called before ' \
                                                                 f'set by TES3MP"); }};'

        func_def = f"#[no_mangle]\npub static mut {PREFIX}{func_name}: fn({args}){ret} = {place_holder}"

        if '<REMOVE>' in func_def:
            continue

        raw += func_def + "\n"

        fancy_name = normalize_var(func_name)
        fancy_args = []

        for arg in func.args:
            fancy_arg = normalize_var(arg[0])

            fancy_arg = fancy_arg.replace('__', '_')

            if arg[1] in ['const char*', 'const char *']:
                fancy_args.append((fancy_arg, '&str', f'CString::new({fancy_arg}).unwrap_or_default().as_ptr()'))
            else:
                fancy_args.append((fancy_arg, TYPE_TRANSLATION[arg[1]], fancy_arg))

        ret = ""
        if func.return_type in ['const char*', 'const char *']:
            ret = " -> String"
        elif func.return_type != "void":
            ret = f" -> {TYPE_TRANSLATION[func.return_type]}"

        func_args = ', '.join([f"{x[0]}: {x[1]}" for x in fancy_args])
        call_args = ', '.join([x[2] for x in fancy_args])

        if func.comment is not None:
            comment = ""
            for line in func.comment.strip().splitlines():
                line = line.strip()
                m = regex.match(RE_COMMENT_PREFIX, line)
                comment += ("///" + (' ' * len(m[1])) + m[2]).rstrip() + "\n"

            comment = regex.sub(RE_BRIEF, '', comment)

            def replace_param(m):
                return f"`{normalize_var(m[1])}`"

            def replace_return(m):
                return f"\n///  Returns {m[1].lower()}"

            comment = regex.sub(RE_PARAM_PREFIX, replace_param, comment)
            comment = regex.sub(RE_RETURN, replace_return, comment)
            comment = regex.subf(r'([^\/])\n\/\/\/([^\n])', '{1}  \n///{2}', comment)
            comment = comment.replace('"[Script]:"', '`[Script]:`')

            fancy += comment

        fancy += f"pub fn {fancy_name}({func_args}){ret} {{\n"
        fancy += "    unsafe {\n"

        if func.return_type in ['const char*', 'const char *']:
            fancy += f"        CStr::from_ptr(raw::{PREFIX}{func_name}({call_args}))\n"
            fancy += f"            .to_str()\n"
            fancy += f"            .unwrap_or_default()\n"
            fancy += f"            .to_string()\n"
        else:
            fancy += f"        raw::{PREFIX}{func_name}({call_args})\n"

        fancy += "    }\n"
        fancy += "}\n\n"

    whole = "use std::ffi::{CStr, CString};\nuse std::os::raw::*;\n\n"
    whole += "#[allow(non_upper_case_globals)]\npub mod raw {\n    use std::os::raw::*;\n"
    for line in raw.splitlines():
        whole += "    " + line + "\n"
    whole += "}\n\n"
    whole += fancy

    with open("tes3mp-plugin/src/plugin/generated.rs", "w") as f:
        f.write(whole)


if __name__ == '__main__':
    main()

from ctypes import Structure, CDLL, POINTER, byref, string_at, c_uint8, c_char_p, c_uint
from os import path
from typing import Tuple, Optional


class BfExecResult(Structure):
    _fields_ = [
        ("success", c_uint8),
        ("output", POINTER(c_uint8)),
        ("output_length", c_uint),
    ]


class BfWrapper(object):
    ARGTYPES = [c_char_p, POINTER(c_uint8), c_uint]

    def __init__(self):
        this_file_directory = path.join(path.sep, *path.abspath(__file__).split(path.sep)[:-1])
        so_file = path.join(this_file_directory, "..", "..", "target", "release", "libbfi.so")
        if not path.exists(so_file):
            raise FileNotFoundError("missing library, have you run `cargo build --release`?")
        self.so = CDLL(so_file)
        self.so.bf_exec.argtypes = self.ARGTYPES
        self.so.bf_exec.restype = BfExecResult

    def bf_exec(self, program: bytes, program_input: Optional[bytes] = None) -> Tuple[bool, bytes]:
        inp = program_input or b""
        input_type = c_uint8 * len(inp)
        result = self.so.bf_exec(program, input_type.from_buffer(bytearray(inp)), len(inp))
        success = result.success == 1
        output = string_at(result.output, size=result.output_length)
        return success, output

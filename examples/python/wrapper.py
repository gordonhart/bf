from ctypes import Structure, CDLL, c_uint8, c_char_p
from os import path


class BfExecResult(Structure):
    _fields_ = [
        ("success", c_uint8),
        ("output", c_char_p),
    ]


class BfWrapper(object):
    def __init__(self):
        this_file_directory = path.join(path.sep, *path.abspath(__file__).split(path.sep)[:-1])
        so_file = path.join(this_file_directory, "..", "..", "target", "release", "libbfi.so")
        if not path.exists(so_file):
            raise FileNotFoundError("missing library, have you run `cargo build --release`?")
        so = CDLL(so_file)
        self.bf_exec = so.bf_exec
        self.bf_exec.argtypes = [c_char_p, c_char_p]
        self.bf_exec.restype = BfExecResult

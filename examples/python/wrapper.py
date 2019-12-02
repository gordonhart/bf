from ctypes import Structure, CDLL, POINTER, byref, string_at, c_uint8, c_char_p, c_size_t
from os import path
from typing import Tuple, Optional


class BfExecResult(Structure):
    _fields_ = [
        ("success", c_uint8),
        ("output", POINTER(c_uint8)),
        ("output_length", c_size_t),
    ]


class BfWrapper(object):
    LIBNAME = "libbfi.so"
    FUNTYPES = {
        "bf_exec": ([c_char_p, POINTER(c_uint8), c_size_t], BfExecResult),
        "bf_free": ([POINTER(c_uint8), c_size_t], None),
    }

    def __init__(self):
        this_file_directory = path.join(path.sep, *path.abspath(__file__).split(path.sep)[:-1])
        so_file = path.join(this_file_directory, "..", "..", "target", "release", self.LIBNAME)
        if not path.exists(so_file):
            raise FileNotFoundError(
                "missing %s, have you run `cargo build --release`?" % self.LIBNAMEi
            )
        self.so = CDLL(so_file)
        self._declare_funtypes()

    def _declare_funtypes(self) -> None:
        for fname, (argtypes, restype) in self.FUNTYPES.items():
            fun = getattr(self.so, fname)
            fun.argtypes = argtypes
            fun.restype = restype

    def bf_exec(self, program: bytes, program_input: Optional[bytes] = None) -> Tuple[bool, bytes]:
        """Call to execute a program with an optional input byte buffer. Kind of funny that manually
        freeing the result is necessary for leak-free interop between Python and Rust, neither of
        which must be managed this way when used alone."""
        input_bytes = program_input or b""
        input_type = c_uint8 * len(input_bytes)
        inp = input_type.from_buffer(bytearray(input_bytes))
        result = self.so.bf_exec(program, inp, len(input_bytes))
        success = result.success == 1
        output = string_at(result.output, size=result.output_length) if success else b""
        # while the program won't crash if asked to free 0 bytes from an invalid location (which is
        # what is returned when success == 0), it's best to not call this free unless we actually
        # want to free the result -- bf certainly will crash if the length requested to be freed
        # is >0 and the pointer is invalid
        if success:
            # return the underlying pointer to Rust so the memory does not leak
            self.so.bf_free(result.output, result.output_length)
        return success, output

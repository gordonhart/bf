from concurrent.futures import ThreadPoolExecutor
from ctypes import Structure, CDLL, POINTER, byref, string_at, c_uint8, c_char_p, c_size_t
from functools import partial
from os import path
from typing import Tuple, Optional


class _BfExecResult(Structure):
    _fields_ = [
        ("success", c_uint8),
        ("output", POINTER(c_uint8)),
        ("output_length", c_size_t),
    ]


class BfBindings(object):
    LIBNAME = "libbfi"
    FUNTYPES = {
        "bf_exec": ([c_char_p, POINTER(c_uint8), c_size_t], _BfExecResult),
        "bf_free": ([POINTER(c_uint8), c_size_t], None),
    }

    # extension could also be e.g. `dylib`, `dll`
    def __init__(self, extension="so"):
        this_file_directory = path.join(path.sep, *path.abspath(__file__).split(path.sep)[:-1])
        lib_stub = "%s.%s" % (self.LIBNAME, extension)
        lib_file = path.join(this_file_directory, "..", "..", "target", "release", lib_stub)
        if not path.exists(lib_file):
            raise FileNotFoundError("missing %s, have you run `cargo build --release`?" % lib_stub)
        self.lib = CDLL(lib_file)
        self._declare_funtypes()
        self._pool = ThreadPoolExecutor(max_workers=1, thread_name_prefix=self.LIBNAME)

    def _declare_funtypes(self) -> None:
        for fname, (argtypes, restype) in self.FUNTYPES.items():
            fun = getattr(self.lib, fname)
            fun.argtypes = argtypes
            fun.restype = restype

    def execute(self, program: bytes, program_input: Optional[bytes] = None) -> Tuple[bool, bytes]:
        """Call to execute a program with an optional input byte buffer. Kind of funny that manually
        freeing the result is necessary for leak-free interop between Python and Rust, neither of
        which must be managed this way when used alone."""
        input_bytes = program_input or b""
        input_type = c_uint8 * len(input_bytes)
        inp = input_type.from_buffer(bytearray(input_bytes))
        # by default foreign calls release the GIL, meaning that the Ctrl-C is not processed if
        # `lib.bf_exec` is runniing in the foreground
        future = self._pool.submit(partial(self.lib.bf_exec, program, inp, len(input_bytes)))
        try:
            result = future.result()
        except KeyboardInterrupt:
            # TODO: this doesn't actually stop the running job in the thread...
            future.cancel() # ensure that the pool is freed of this process
            raise
        success = result.success == 1
        output = string_at(result.output, size=result.output_length) if success else b""
        # while the program won't crash if asked to free 0 bytes from an invalid location (which is
        # what is returned when success == 0), it's best to not call this free unless we actually
        # want to free the result -- bf certainly will crash if the length requested to be freed
        # is >0 and the pointer is invalid
        if success:
            # return the underlying pointer to Rust so the memory does not leak
            self.lib.bf_free(result.output, result.output_length)
        return success, output

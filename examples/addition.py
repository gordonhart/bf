from ctypes import Structure, CDLL, c_uint8, c_char_p
from os import path
import random


class BfExecResult(Structure):
    _fields_ = [
        ("success", c_uint8),
        ("output", c_char_p),
    ]


class BfU8Adder(object):
    PROGRAM = b",>,<[->+<]>."

    def __init__(self):
        this_file_directory = path.join(path.sep, *path.abspath(__file__).split(path.sep)[:-1])
        so_file = path.join(this_file_directory, "..", "target", "release", "libbfi.so")
        if not path.exists(so_file):
            raise FileNotFoundError("missing library, have you run `cargo build --release`?")
        so = CDLL(so_file)
        self.bf_exec = so.bf_exec
        self.bf_exec.argtypes = [c_char_p, c_char_p]
        self.bf_exec.restype = BfExecResult

    def __call__(self, a: int, b: int) -> int:
        if a > 255 or b > 255:
            raise OverflowError("received argument that does not fit in `u8`")
        args = b"".join([i.to_bytes(1, byteorder="big", signed=False) for i in [a, b]])
        response = self.bf_exec(self.PROGRAM, args)
        if response.success == 0:
            raise RuntimeError("internal error: unable to add '%d' and '%d'" (a, b))
        return response.output[0]


if __name__ == "__main__":
    adder = BfU8Adder()
    i = lambda: random.randint(1, 127)  # TODO: get bf implementation to properly read 0 bytes
    for numbers in [(i(), i()) for _ in range(100)]:
        sum_of_numbers = adder(*numbers)
        assert sum_of_numbers == sum(numbers)
        print("%3d + %3d = %3d" % (*numbers, sum_of_numbers))

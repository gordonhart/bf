from pathlib import Path
from typing import Optional
import zlib

from wrapper import BfWrapper


class DeceptionEnabler(object):
    """Make sure to put `*.bf` in your .gitignore!"""
    def __init__(self, binary_extension: str = "gif", bf_extension: str = "bf"):
        self.binary_ext = ".%s" % binary_extension
        self.bf_ext = ".%s" % bf_extension

    def compress_program(self, program: bytes) -> bytes:
        return zlib.compress(program, level=9)

    def decompress_program(self, program: bytes) -> bytes:
        return zlib.decompress(program)

    def load_from_file(self, fname: str) -> bytes:
        with open(fname, "rb") as f:
            compressed_data = f.read()
        uncompressed_data = self.decompress_program(compressed_data)
        return uncompressed_data

    def save_program_to_file(self, program: bytes, out_fname: bool, overwrite: bool = True) -> None:
        compressed_data = self.compress_program(program)
        open_flags = "wb" if overwrite_existing is True else "xb"
        with open(out_fname, open_flags) as f:
            f.write(compressed_data)

    def decompress_from_file(self, binary_fname: str, bf_fname: Optional[str] = None) -> None:
        out_fname = decompressed_fname or str(Path(compressed_fname).with_suffix(self.bf_ext))
        uncompressed_data = self.load_program(compressed_fname)
        with open(out_fname, "wb") as f:
            f.write(uncompressed_data)

    def compress_from_file(self, bf_fname: str, binary_fname: Optional[str] = None) -> None:
        with open(decompressed_fname, "rb") as f:
            uncompressed_data = f.read()
        out_fname = compressed_fname or str(Path(decompressed_fname).with_suffix(self.binary_ext))
        self.save_program_to_file(uncompressed_data, out_fname)

import time
from typing import List

from bindings import BfBindings


class BfSierpinski(BfBindings):
    PROGRAM = b"""
        ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
            -<<<[
                ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
            ]>.>+[>>]>+
        ]
    """

    def __str__(self) -> str:
        success, output = self.execute(self.PROGRAM)
        if not success:
            raise RuntimeError("unable to compute")
        return output.decode("utf-8")


def sierpinski_native(n: int) -> str:
    """This implementation is directly translated from the Rust implementation in
    `tests/executable.rs`."""
    def sierpinski_inner(_n: int) -> List[str]:
        if _n == 0:
            return ["*"]
        else:
            prev = sierpinski_inner(_n - 1)
            prev_width = len(prev[-1])
            next_width = prev_width * 2 + 1
            next_iter = []
            for i, cur in enumerate(prev):
                top = "%s%s" % (" " * ((next_width - prev_width) // 2), cur)
                bottom = "%s%s%s" % (cur, " " * ((next_width - prev_width) - len(cur)), cur)
                next_iter.insert(i, top)
                next_iter.append(bottom)
            return next_iter
    return "%s\n" % "\n".join(sierpinski_inner(n))


if __name__ == "__main__":
    t_start  = time.time()
    print(sierpinski_native(5))
    print("native:  %0.5fs" % (time.time() - t_start))

    t_start  = time.time()
    print(BfSierpinski())
    print("foreign: %0.5fs" % (time.time() - t_start))

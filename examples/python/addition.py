from random import randint

from wrapper import BfWrapper


class BfU8Adder(BfWrapper):
    PROGRAM = b",>,<[->+<]>."

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
    i = lambda: randint(1, 127)  # TODO: get bf implementation to properly read 0 bytes
    for numbers in [(i(), i()) for _ in range(100)]:
        sum_of_numbers = adder(*numbers)
        assert sum_of_numbers == sum(numbers)
        print("%3d + %3d = %3d" % (*numbers, sum_of_numbers))

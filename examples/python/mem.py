from ctypes import c_uint8
import resource

from bindings import BfBindings

def get_max_resident_memory_usage_kb() -> int:
    """On Linux, memory usage returned from `resource.getrusage` is in KB, which is probably not
    the case on all other OSes."""
    return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss

if __name__ == "__main__":
    # double for-loop each from 255 down to zero, outputting a byte each iteration
    program = b"-[->-[-.]<]"
    # this calculation is only going to be more-or-less correct on systems where resource usage is
    # reported in KB
    result_size = 255 * 255 / 1024
    n = 2048
    bf = BfBindings()

    print("demonstration of efficacy of `bf_free`")
    print("=" * 60)

    memusage_before = get_max_resident_memory_usage_kb()
    for i in range(n):
        bf.execute(program)
    memusage_after = get_max_resident_memory_usage_kb()
    print("\nmemory usage summary with proper freeing of results")
    print("=" * 60)
    print("before:   %d" % memusage_before)
    print("after:    %d" % memusage_after)
    print("diff:     %d" % (memusage_after - memusage_before))
    print("expected: %d" % (2 * result_size))

    program_input = (c_uint8 * 0).from_buffer(bytearray(b""))
    memusage_before = get_max_resident_memory_usage_kb()
    for i in range(n):
        bf.lib.bf_exec(program, program_input, 0)
    memusage_after = get_max_resident_memory_usage_kb()
    print("\nmemory usage summary without proper freeing of results")
    print("=" * 60)
    print("before:   %d" % memusage_before)
    print("after:    %d" % memusage_after)
    print("diff:     %d" % (memusage_after - memusage_before))
    print("expected: %d" % (n * result_size))

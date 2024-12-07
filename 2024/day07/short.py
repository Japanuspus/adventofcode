import re
import itertools
import operator
import math


def check_rec(res, val, rem, ops) -> bool:
    if not rem:
        return val==res
    if val>res:
        # all operations increase value - cut if we are past result
        return False
    return any(check_rec(res, op(val, rem[0]), rem[1:], ops) for op in ops)

with open("input.txt") as f:
    eqs = [(a, b) for a, *b in ([int(v) for v in re.split(":? ", ln)] 
                                for ln in f.read().strip().split("\n"))]

print([
    sum(res for res, ns in eqs if check_rec(res, val=ns[0], rem=ns[1:], ops=ops)) for ops in [
        (operator.add, operator.mul),
        (operator.add, operator.mul, lambda a, b: a*10**len(str(b)) + b),
    ]])

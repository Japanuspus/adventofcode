import re
import itertools
import operator
import math


def check_eq(eq, ops):
    res, ns = eq
    for os in itertools.product(ops, repeat=len(ns)-1):
        v = ns[0]
        for b, op in zip(ns[1:], os):
            v = op(v, b)
        if v == res:
            return True
    return False


with open("input.txt") as f:
    eqs = [(a, b) for a, *b in ([int(v) for v in re.split(":? ", ln)] 
                                for ln in f.read().strip().split("\n"))]

print([
    sum(res for res, ns in eqs if check_eq((res, ns), ops=ops)) for ops in [
        (operator.add, operator.mul),
        (operator.add, operator.mul, lambda a, b: a*10**math.ceil(math.log10(b)) + b),
    ]])

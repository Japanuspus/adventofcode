# ---
# jupyter:
#   jupytext:
#     formats: ipynb,py:percent
#     text_representation:
#       extension: .py
#       format_name: percent
#       format_version: '1.3'
#       jupytext_version: 1.16.1
#   kernelspec:
#     display_name: Python 3 (ipykernel)
#     language: python
#     name: python3
# ---

# %%
import re
import itertools
import functools
import collections
import dataclasses


# %%
@dataclasses.dataclass
class State:
    a: int
    b: int
    c: int
    pc: int = 0
    a_shift = 0     # accumulated shift on a
    look_shift = 0  # largest shift on any register. look_shift < a_shift+2**3


with open(["input.txt", "test00.txt", "test01.txt"][0]) as f:
    ns = [int(n) for n in re.findall(r"\d+", f.read().strip())]
    state0 = State(*ns[:3])
    mem0 = ns[3:]

# %%
# there is no branching: exactly one shift of a (by 3 bits) per output
# lookahead can include no more than 2**3 == 8 bits
print(f"Program (opcode, opvals): {list(itertools.batched(mem0, 2))}")


# %%
def step(state):
    def combo(v):
        match v:
            case 4:
                rv = state.a
            case 5:
                rv = state.b
            case 6:
                rv = state.c
            case 7:
                raise ValueError("Comboval 7")
            case _:
                rv = v
        return rv%8

    opcode = mem0[state.pc]
    opval = mem0[state.pc+1]
    state.pc+=2
    match opcode:
        case 0: #adv
            c = combo(opval)
            state.a_shift+=c
            state.look_shift = max(state.look_shift, state.a_shift)
            state.a = state.a//2**c
        case 1: #bxl
            state.b ^= opval
        case 2: #bst
            state.b = combo(opval)
        case 3: #jnz
            if state.a != 0:
                state.pc = opval
        case 4: #bxc
            state.b ^= state.c
        case 5: #out
            c = combo(opval)
            return c
        case 6: #bdv
            c = combo(opval)
            state.look_shift = max(state.look_shift, state.a_shift+c)
            state.b = state.a//2**c
        case 7: #cdv
            c = combo(opval)
            state.look_shift = max(state.look_shift, state.a_shift+c)
            state.c = state.a//2**c


# %%
def outputs(a=None):
    state = State(**dataclasses.asdict(state0))
    if a is not None:
        state.a = a
    while state.pc < len(mem0):
        out=step(state)
        if out is not None:
            yield (state, out)


# %%
print(','.join(str(o) for s,o in outputs()))

# %%
Root = collections.namedtuple('Root', 'n value')
roots = {Root(0,0)}
max_count_exp=10
a_candidates = []
for k in range(len(mem0)):
    next_roots = set()
    for root in roots:
        n_match=k+1
        a_vals = (root.value|(a2<<root.n) for a2 in range(2**max_count_exp))
        for a in a_vals:
            outs = [state for ref, (state, out) in zip(mem0[:n_match],outputs(a)) if ref==out]
            if len(outs)<n_match:
                continue
            if n_match == len(mem0):
                a_candidates.append(a)
                break
            final_state = outs[-1]
            if final_state.look_shift+3>root.n+max_count_exp:
                raise ValueError(f"Program considered values outside max_count")
            next_roots.add(Root(final_state.a_shift, a %(2**final_state.a_shift)))
    roots = next_roots
print(min(a_candidates))

# %%

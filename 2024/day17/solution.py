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
import numpy as np
import itertools
import math
import functools
import operator
import collections
from dataclasses import dataclass
import dataclasses


# %%
@dataclass
class State:
    a: int
    b: int
    c: int
    pc: int = 0

with open(["input.txt", "test00.txt", "test01.txt"][0]) as f:
    ns = [int(n) for n in re.findall(r"\d+", f.read().strip())]
    state0 = State(*ns[:3])
    mem0 = ns[3:]

mem0


# %% [markdown]
# ## Part 1 as solved

# %%
def step(state, mem, out):
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

    opcode = mem[state.pc]
    opval = mem[state.pc+1]
    #print(f"{state} > {out=}")
    match opcode:
        case 0: #adv
            state.a = state.a//2**combo(opval)
        case 1: #bxl
            state.b ^= opval
        case 2: #bst
            state.b = combo(opval)
        case 3: #jnz
            if state.a != 0:
                state.pc = opval-2
        case 4: #bxc
            state.b ^= state.c
        case 5: #out
            c = combo(opval)
            #print(f"Combo: {opval=} @ pc={state.pc}-> {c} ")
            out.append(combo(opval))
        case 6: #bdv
            state.b = state.a//2**combo(opval)
        case 7: #cdv
            state.c = state.a//2**combo(opval)
    state.pc+=2

out = []
mem = mem0.copy()
state = State(**dataclasses.asdict(state0))
#state.a = 117440
while state.pc < len(mem):
    step(state, mem, out)

','.join(str(v) for v in mem), ','.join(str(v) for v in out)

# %% [markdown]
# ## Part 2 - take 1
#
# This works for the example

# %%
infshift = 2**256

@dataclass
class StateShift:
    a: int
    b: int
    c: int
    pc: int = 0
    a_s: int = 0
    b_s: int = infshift
    c_s: int = infshift


def step(state: StateShift, mem):
    def combo(v):
        match v:
            case 4:
                rv = state.a
                s = state.a_s
            case 5:
                rv = state.b
                s = state.b_s
            case 6:
                rv = state.c
                s = state.c_s
            case 7:
                raise ValueError("Comboval 7")
            case _:
                rv = v
                s = infshift
        return (rv%8, s)

    opcode = mem[state.pc]
    opval = mem[state.pc+1]
    state.pc+=2
    #print(f"{state} > {out=}")
    match opcode:
        case 0: #adv
            (c, cs) = combo(opval)
            state.a = state.a//2**c
            state.a_s += c
        case 1: #bxl
            state.b ^= opval
            state.b_s = infshift
        case 2: #bst
            (c, cs) = combo(opval)
            state.b = c
            state.b_s = cs
        case 3: #jnz
            if state.a != 0:
                state.pc = opval
        case 4: #bxc
            state.b ^= state.c
            state.b_s = min(state.b_s, state.c_s)
        case 5: #out
            (c, cs) = combo(opval)
            #print(f"Combo: {opval=} @ pc={state.pc-2}-> {c} ")
            return c
        case 6: #bdv
            (c, cs) = combo(opval)
            state.b = state.a//2**c
            state.b_s = state.a_s + c
        case 7: #cdv
            (c, cs) = combo(opval)
            state.c = state.a//2**c
            state.c_s = state.a_s + c
    return None


def outputs(a):
    state = StateShift(**dataclasses.asdict(state0))
    state.a = a
    mem = mem0
    visited = set()
    step_count = 0
    while state.pc < len(mem):
        v_val = (state.pc, state.a, state.b, state.c)
        step_count +=1
        if v_val in visited:
            print(f" Loop detected at {a=}")
            return
        else:
            visited.add(v_val)

        if step_count % 1_000 == 0:
            print(f" {step_count=} with {state=}")
        if (out:=step(state, mem)) is not None:
            #visited = set()
            yield (state, out)



# %%
known_n = 0
known_val = 0
final_a = None
while final_a is None:
    print(f"Starting from {known_n} bits of a: {known_val:0b}")
    for a_next in itertools.count():
        a=known_val | a_next<<known_n
        if a_next%1_000_000 == 0:
            print(f"  >{a_next=} -> {a}")
        min_shift = 0
        for i,(ref, (state, out)) in enumerate(zip(mem0, outputs(a))):
            #print(out)
            if ref!=out:
                break
            if i+1==len(mem0):
                final_a = a
            min_shift = min([state.a_s, state.b_s, state.c_s])
        if final_a is not None:
            break
        if min_shift>known_n:
            known_n = min_shift
            known_val = a % 2**known_n
            break

print(final_a)
        

# %% [markdown]
# * * *
#
# # Second take on part 2

# %%
@dataclass
class StateRestricted:
    a: int
    b: int
    c: int
    pc: int = 0
    a_shift = 0     # accumulated shift on a
    look_shift = 0  # largest shift on any register. look_shift < a_shift+2**3

def step_restricted(state, mem):
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

    opcode = mem[state.pc]
    opval = mem[state.pc+1]
    #print(f"{state} > {out=}")
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
            #print(f"Combo: {opval=} @ pc={state.pc}-> {c} ")
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
# there is no branching: exactly one shift of a (by 3 bits) per output
# lookahead can include no more than 2**3 == 8 bits
list(itertools.batched(mem0, 2))


# %%

def outputs(a=None):
    """
    Yields (state, out) until maxshift, indicated by (state, None)
    """
    state = StateRestricted(**dataclasses.asdict(state0))
    if a is not None:
        state.a = a
    mem = mem0
    while state.pc < len(mem):
        out=step_restricted(state, mem)
        if out is not None:
            yield (state, out)


# %%
print(mem0)
[o for (state, o) in outputs( a=117440)]


# %%
[(f"{a:08b}",sv[1], sv[0].a_shift) for a,sv in ((a, next(outputs(a))) for a in range(2**8)) if sv is not None and sv[1]==mem0[0]]


# %%
Root = collections.namedtuple('Root', 'n value')
def next_roots(root: Root, n_match, max_count_exp=8):
    a_vals = (root.value|(a2<<root.n) for a2 in range(2**max_count_exp))
    roots = set()
    for a in a_vals:
        outs = [state for ref, (state, out) in zip(mem0[:n_match],outputs(a)) if ref==out]
        if len(outs)<n_match:
            continue
        final_state = outs[-1]
        if final_state.look_shift+3>root.n+max_count_exp:
            raise ValueError(f"Program considered values outside max_count")
        roots.add(Root(final_state.a_shift, a %(2**final_state.a_shift)))
    return roots

def first_a(root: Root, max_count_exp=8):
    a_vals = (root.value|(a2<<root.n) for a2 in range(2**max_count_exp))
    for a in a_vals:
        outs = [state for ref, (state, out) in zip(mem0,outputs(a)) if ref==out]
        if len(outs)<len(mem0):
            continue
        return a



# %%

roots = {Root(0,0)}
for k in range(len(mem0)):
    roots = set.union(*list(next_roots(root, k+1, max_count_exp=10) for root in roots))
    
    

# %%
min(first_a(r) for r in roots)

# %%

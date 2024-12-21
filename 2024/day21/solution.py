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
import itertools
import functools
import dataclasses


# %%
@dataclasses.dataclass
class KeyPad:
    pos_char: dict

    @functools.cached_property
    def char_pos(self):
        return {c:p for p,c in self.pos_char.items()}
        
    def __str__(self):
        return '\n'.join(
            ''.join(
                self.pos_char.get((x,y), ' ')
                for x in range(1+max(x for x,y in self.pos_char.keys()))
            ) for y in range(1+max(y for (x,y) in self.pos_char.keys())))
        

def read_kp(fn):
    with open(fn) as f:
        lines = f.read().strip().split("\n")
        pc = (((x, y),c) for y,ln in enumerate(lines) for x, c in enumerate(ln))
        return  {((x-2)//4, (y-1)//2): c for (x,y),c in pc if c not in {'-','|',' ', '+'}}

read_kp("test00.txt")
knum, kmove = [KeyPad(read_kp(fn)) for fn in ["test00.txt","test01.txt"]]
print(knum)

# %%
with open(["input.txt", "test03.txt"][0]) as f:
    codes = f.read().strip().split("\n")
codes

# %%
dir_chars = {(1,0): '>', (0,-1): '^', (-1,0): '<', (0,1): 'v'}

def routes(kp: KeyPad, ca, cb) -> str:
    (ax,ay), (bx, by) = [kp.char_pos[c] for c in (ca, cb)]
    dx, dy = bx-ax, by-ay
    hv = ((dx,0,abs(dx)), (0,dy,abs(dy)))
    if all(s>0 for _,_,s in hv):
        vvs = [vv for vv in (hv, (hv[1], hv[0])) if (ax+vv[0][0], ay+vv[0][1]) in kp.pos_char]
    else:
        vvs = [hv]
    return [''.join(dir_chars[(sx//s, sy//s)]*s for sx,sy,s in vv if s>0) for vv in vvs]


# %%
def count(keys: str, keypads: list[KeyPad]):
    if not keypads:
        return len(keys)
    kp, *kps = keypads
    cn = [('A', 0)]+[(c,len(list(g))) for c,g in itertools.groupby(keys)]
    return sum(
        min(count(r+'A'*n,kps) for r in routes(kp,a,b))
        for (a,_),(b,n) in zip(cn, cn[1:])
    )

counts = [count(code, [knum, kmove, kmove]) for code in codes]
vals = [int(code[:-1]) for code in codes]
print(sum(c*v for c,v in zip(counts, vals)))


# %%
cc = dict()
def cached_count(keys, keypads):
    if not keypads:
        return len(keys)
    ckey = (keys, len(keypads))
    if (res:=cc.get(ckey)) is None:
        kp, *kps = keypads
        cn = [('A', 0)]+[(c,len(list(g))) for c,g in itertools.groupby(keys)]
        res = sum(
            min(cached_count(r+'A'*n,kps) for r in routes(kp,a,b))
            for (a,_),(b,n) in zip(cn, cn[1:])
        )
        cc[ckey] = res
    return res

counts = [cached_count(code, [knum]+[kmove]*25) for code in codes]
vals = [int(code[:-1]) for code in codes]
print(sum(c*v for c,v in zip(counts, vals)))

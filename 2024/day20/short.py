import collections
import heapq

with open(["input.txt","test00.txt"][0]) as f:
    lines = f.read().strip().split("\n")
    # real, y, down -- imag, x, right
    pc = ((y+1j*x,c) for y,ln in enumerate(lines) for x, c in enumerate(ln))
    walls = set()
    for p,c in pc:
        match c:
            case '#':
                walls.add(p)
            case 'S':
                start = p
            case 'E':
                end = p


# Need to wrap p to store in heap since complex values cannot be compared
class P(collections.namedtuple('PBase', 's p')):
    def __lt__(self, other):
        return self.s<other.s


smap = collections.defaultdict(lambda: 0)
work = [P(0, start)]
heapq.heapify(work)
dirs = [1, 1j, -1, -1j]
while work:
    s, p = heapq.heappop(work)
    if p in smap:
        continue
    smap[p] = s
    for pd in (p+d for d in dirs if p+d not in walls):
        heapq.heappush(work, P(s+1, pd))


cheats = {a+b for a in dirs for b in dirs if a+b != 0}
cheatvals = (smap[p+c]-sp-2 for p,sp in smap.items() for c in cheats if p+c in smap)
print(len([saved for saved in cheatvals if saved>=100]))


cheatlist = [{0}, {d for d in dirs}]
for _ in range(19):
    c1 = cheatlist[-2]
    c2 = cheatlist[-1]
    new_cheats = {c for c in ((a+b) for a in c2 for b in dirs) if c not in c1 and c not in c2}
    cheatlist.append(new_cheats)
cheats = {c: s for s, cs in enumerate(cheatlist) for c in cs if s>1}
cheatvals = (smap[p+c]-sp-sc for p,sp in smap.items() for c, sc in cheats.items() if p+c in smap)
print(len([saved for saved in cheatvals if saved>=100]))
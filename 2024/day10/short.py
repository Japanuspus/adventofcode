import numpy as np
import itertools

with open(["input.txt", "test04.txt"][0]) as f:
    topo = np.array([[int(c) for c in ln] for ln in f.read().strip().split("\n")])
dtopo = {y+1j*x: int(v) for ((y,x),v) in np.ndenumerate(topo)}
theads = [p for p, v in dtopo.items() if v==0]
dirs = [1,1j,-1,-1j]


def reachable(p,pv):
    if pv==9:
        return {p}
    return {v for nb in (p+d for d in dirs if dtopo.get(p+d,0)==pv+1) 
            for v in reachable(nb, pv+1)}


def count_the_ways(p, pv) -> int:
    if pv==9:
        return 1
    return sum(count_the_ways(nb, pv+1) 
               for nb in (p+d for d in dirs if dtopo.get(p+d,0)==pv+1)) 


print(sum(len(reachable(th, 0)) for th in theads))
print(sum(count_the_ways(th,0) for th in theads))
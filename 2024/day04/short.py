with open("input.txt") as f:
    cmap = {(x,y): c for (y, ln) in enumerate(f.read().strip().split("\n")) for (x, c) in enumerate(ln)}
print(
    sum(1 for (px,py) in cmap.keys() for (dx,dy) in [(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1)] 
        if all(cmap.get((px+s*dx, py+s*dy), None)==c for s,c in enumerate("XMAS"))),
    sum(1 for (px,py),c0 in cmap.items() if c0=='A' 
        if all({cmap.get((px+s*dx, py+s*dy), None) for s in [1, -1]}=={'M','S'} for (dx,dy) in [(1,1), (-1,1)])))
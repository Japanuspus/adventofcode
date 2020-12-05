import requests
import json
import sys
from pathlib import Path


day = int(sys.argv[1])
# aoc.json : {"session": "..."}
cfg = json.loads(Path('aoc.json').read_text())
url = f'https://adventofcode.com/2019/day/{day}/input'
outfile = Path(f'day{day:02d}') / 'input.txt'
outfile.parent.mkdir(exist_ok=True)
resp = requests.get(url, cookies={'session': cfg['session']})
outfile.write_text(resp.text)


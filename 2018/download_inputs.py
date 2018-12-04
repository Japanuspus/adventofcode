import requests
import yaml
import sys
from pathlib import Path

day = int(sys.argv[1])
cfg = yaml.load(Path('aoc.yaml').read_text())
url = f'https://adventofcode.com/2018/day/{day}/input'
outfile = Path('inputs') / f'day{day:02d}.txt'
resp = requests.get(url, cookies={'session': cfg['session']})
outfile.write_text(resp.text)


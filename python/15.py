import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict, OrderedDict


def part1(data):
    """ 2023 Day 15 Part 1

    >>> part1(['rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7'])
    1320
    """

    return sum(hashAlg(s) for s in data[0].split(','))


def part2(data):
    """ 2023 Day 15 Part 2

    >>> part2(['rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7'])
    145
    """

    boxes = defaultdict(lambda: OrderedDict())

    for line in data[0].split(','):
        label = re.findall('\w+', line)[0]
        box = hashAlg(label)
        
        op = re.findall('[-=]', line)[0]

        if op == '=':
            lens = int(re.findall('\d+', line)[0])
            boxes[box][label] = lens
        elif label in boxes[box]:
            boxes[box].pop(label)

    s = 0
    for boxNum, contents in boxes.items():
        boxNum += 1

        for slot, (label, fLen) in enumerate(contents.items()):
            slot += 1
            s += boxNum * slot * fLen

    return s


def hashAlg(s):
    val = 0
    for c in s:
        val += ord(c)
        val *= 17
        val %= 256

    return val


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nSum of hashes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFocusing power: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)
import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict
from functools import cache
from itertools import product


def part1(data):
    """ 2023 Day 19 Part 1

    >>> part1(['px{a<2006:qkq,m>2090:A,rfg}', 'pv{a>1716:R,A}', 'lnx{m>1548:A,A}', 'rfg{s<537:gd,x>2440:R,A}', 'qs{s>3448:A,lnx}', 'qkq{x<1416:A,crn}', 'crn{x>2662:A,R}', 'in{s<1351:px,qqz}', 'qqz{s>2770:qs,m<1801:hdj,R}', 'gd{a>3333:R,R}', 'hdj{m>838:A,pv}', '', '{x=787,m=2655,a=1222,s=2876}', '{x=1679,m=44,a=2067,s=496}', '{x=2036,m=264,a=79,s=2244}', '{x=2461,m=1339,a=466,s=291}', '{x=2127,m=1623,a=2188,s=1013}'])
    19114
    """

    ix = 0
    while len(data[ix]) != 0:
        name, ruleInfo = data[ix][:-1].split('{')
        ruleInfo = ruleInfo.split(',')

        for r in ruleInfo:
            try:
                cond, next = r.split(':')
            except ValueError:
                cond, next = 'True', r

            rules[name].append([cond, next])

        ix += 1

    count = 0
    for line in data[ix + 1:]:
        x, m, a, s = [int(n) for n in re.findall('\d+', line)]

        if evalWorkflow('in', x, m, a, s):
            count += x+m+a+s

    return count


def part2(data):
    """ 2023 Day 19 Part 2

    >>> part2(['px{a<2006:qkq,m>2090:A,rfg}', 'pv{a>1716:R,A}', 'lnx{m>1548:A,A}', 'rfg{s<537:gd,x>2440:R,A}', 'qs{s>3448:A,lnx}', 'qkq{x<1416:A,crn}', 'crn{x>2662:A,R}', 'in{s<1351:px,qqz}', 'qqz{s>2770:qs,m<1801:hdj,R}', 'gd{a>3333:R,R}', 'hdj{m>838:A,pv}', '', '{x=787,m=2655,a=1222,s=2876}', '{x=1679,m=44,a=2067,s=496}', '{x=2036,m=264,a=79,s=2244}', '{x=2461,m=1339,a=466,s=291}', '{x=2127,m=1623,a=2188,s=1013}'])
    167409079868000
    """

    if len(rules) == 0:
        part1(data)

    s = 0
    accepted = ruleCombs('in')

    for i in range(len(accepted['x'])):
        rs = [accepted[k][i] for k in 'xmas']
        product = 1
        for small, big in rs:
            product *= (big - small + 1)

        s += product

    return s


rules = defaultdict(lambda: [])


def andConds(mainCond, otherCond):
    newConds = {k: [] for k in 'xmas'}
    for spec, conds in mainCond.items():
        if spec not in otherCond or len(otherCond[spec]) == 0:
            newConds[spec] = conds
            continue

        for (mainMin, mainMax), (otherMin, otherMax) in product(conds, otherCond[spec]):
            newConds[spec].append([max(mainMin, otherMin), min(mainMax, otherMax)])

    return newConds


def orConds(mainCond, otherCond):
    newConds = {k: [] for k in 'xmas'}
    for spec, conds in mainCond.items():
        newConds[spec] += conds

    for spec, conds in otherCond.items():
        newConds[spec] += conds

    return newConds


@cache
def ruleCombs(rule):
    if rule == 'A':
        return {k: [[1, 4000]] for k in 'xmas'}
    
    if rule == 'R':
        return {k: [] for k in 'xmas'}
    
    ruleConds = {k: [] for k in 'xmas'}
    for cond, dest in rules[rule][::-1]:
        try:
            op = re.findall('[<>]+', cond)[0]
            spec, val = cond.split(op)
            val = int(val)

            if op == '<':
                cond = {spec: [[1, val - 1]]}
                notCond = {spec: [[val, 4000]]}
            else:
                cond = {spec: [[val + 1, 4000]]}
                notCond = {spec: [[1, val]]}

            ruleConds = andConds(ruleConds, notCond)

            destCond = ruleCombs(dest)
            destCond = andConds(destCond, cond)
        except IndexError:
            destCond = ruleCombs(dest)

        ruleConds = orConds(ruleConds, destCond)

    return ruleConds


def evalWorkflow(workflow, x, m, a, s):
    while workflow not in 'AR':
        for cond, dest in rules[workflow]:
            if eval(cond):
                workflow = dest
                break

    return workflow == 'A'


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
        print(f"\nPart 1:\nSum of Accepted Ratings: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal combinations accepted: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)
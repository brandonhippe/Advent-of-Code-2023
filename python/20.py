import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict
from sympy import isprime


def part1(data):
    """ 2023 Day 20 Part 1

    >>> part1(['broadcaster -> a, b, c', '%a -> b', '%b -> c', '%c -> inv', '&inv -> a'])
    32000000
    >>> part1(['broadcaster -> a', '%a -> inv, con', '&inv -> b', '%b -> con', '&con -> output'])
    11687500
    """

    outputs = {}
    inputs = {}

    for line in data:
        name, connections = line.split(' -> ')
        connections = connections.split(', ')

        if name[0] in '%&':
            nodeType = name[0]
            name = name[1:]
            if nodeType == '&':
                inputs[name] = set()

        outputs[name] = connections

    state = []
    for name, connections in outputs.items():
        for connect in connections:
            if connect in inputs:
                inputs[connect].add(name)

        state.append((name, name in inputs))

    state = tuple(state)
    lowTot, highTot = 0, 0

    for _ in range(1000):
        state, pulseCounts = pressButton(state, inputs, outputs)

        for cycle in pulseCounts:
            for _, pulse in cycle:
                if pulse:
                    highTot += 1
                else:
                    lowTot += 1

    return lowTot * highTot


def part2(data):
    """ 2023 Day 20 Part 2
    """

    outputs = {}
    inputs = {}

    for line in data:
        name, connections = line.split(' -> ')
        connections = connections.split(', ')

        if name[0] in '%&':
            nodeType = name[0]
            name = name[1:]
            if nodeType == '&':
                inputs[name] = set()

        outputs[name] = connections

    state = []
    for name, connections in outputs.items():
        for connect in connections:
            if connect in inputs:
                inputs[connect].add(name)

        state.append((name, name in inputs))

    state = tuple(state)

    ### For part 2. Find the distict groupings of modules
    portions = []
    for n in outputs['broadcaster']:
        subState = set()
        openList = [n]

        while len(openList) != 0:
            pos = openList.pop(0)
            subState.add(pos)

            for n1 in outputs[pos]:
                if n1 in subState or n1 in openList or n1 == 'rx':
                    continue

                openList.append(n1)

        portions.append(tuple(subState))

    ### Find the common module to all portions. This is a conjunction module that outputs to rx
    common = set(list(portions[0]))
    for p in portions:
        common.intersection_update(p)

    ### Find the modules that input to the common conjunction module
    trackOutputs = {k: [] for k in inputs[list(common)[0]]}

    i = 0

    portionStates = defaultdict(lambda: defaultdict(lambda: 0))
    cyclesFound = {p: None for p in portions}
    while any(c is None for c in cyclesFound.values()):
        state, pulseCounts = pressButton(state, inputs, outputs)

        for portion in portions:
            if cyclesFound[portion] is not None:
                continue
            
            portionPulses = []
            for cycle in pulseCounts:
                portionPulses.append(tuple((n, p) for n, p in cycle if n in portion))

            portionPulses.append(tuple(s for s in state if s[0] in portion))
            
            portionPulses = tuple(portionPulses)

            if portionPulses in portionStates[portion]:
                cyclesFound[portion] = (portionStates[portion][portionPulses], i)
            else:
                portionStates[portion][portionPulses] = i

        i += 1

    ### Figure out when the tracked modules output high
    for t in trackOutputs.keys():
        portion = list(p for p in portions if t in p)[0]
        cyclesFound[t] = cyclesFound[portion]
        del(cyclesFound[portion])

        for state, press in portionStates[portion].items():
            for pulses in state:
                if (t, True) in pulses:
                    trackOutputs[t].append(press)

    for t, rems in trackOutputs.items():
        for r in rems:
            if len([c for c in rems if c == r]) != 1 or r - 1 in rems or (r == cyclesFound[t][0] and cyclesFound[t][1] in rems):
                continue

            cyclesFound[t] = (cyclesFound[t][1] - cyclesFound[t][0], r)
            break

    cycleLens = [c[0] for c in cyclesFound.values()]
    rems = [c[1] for c in cyclesFound.values()]

    return chineseRemainderTheorem(rems, cycleLens)


def chineseRemainderTheorem(rems, cycleLens):
    n = 1
    for cLen in cycleLens:
        n *= cLen

    ys = []
    zs = []
    for cLen in cycleLens:
        ys.append(n // cLen)
        if isprime(cLen):
            zs.append(pow(ys[-1], cLen - 2, cLen))

    result = sum(r*y*z for r, y, z in zip(rems, ys, zs))
    modn = result % n
    
    return modn if modn else n


def pressButton(state, inputs, outputs):
    pulses = []
    newState = {s[0]: s[1] for s in state}

    stack = []
    stack.append(['broadcaster', False])

    while len(stack) != 0:
        newStack = []
        pulses.append([])

        for module, inPulse in stack:
            pulses[-1].append((module, inPulse))

            if module not in outputs or module == 'broadcaster':
                newState[module] = inPulse

                if module not in outputs:
                    continue
            elif module not in inputs:
                ## Module is FF Module
                if inPulse:
                    continue

                newState[module] = not newState[module]
            else:
                ## Module is Conjunction Module
                newPulse = False
                for n in inputs[module]:
                    if not newState[n]:
                        newPulse = True
                        break

                newState[module] = newPulse

            for n in outputs[module]:
                newStack.append((n, newState[module]))

        pulses[-1] = tuple(pulses[-1])
        stack = newStack

    return tuple((k, v) for k, v in newState.items()), tuple(pulses)


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
        print(f"\nPart 1:\nTotal Low Pulses * Total High Pulses: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nButton Presses: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)
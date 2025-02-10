from itertools import combinations
from math import gcd

def parse(input):
    data = input.strip().splitlines()
    nx, ny = len(data), len(data[0])
    antennas = {}
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell != '.':
                antennas.setdefault(cell, []).append(x+1j*y)
    return antennas, nx, ny

def simplify_frac(a, b):
    g = gcd(int(a), int(b))
    return (a/g, b/g)

class Day8:
    def part1(input):
        antennas, nx, ny = parse(input)
        antinodes = {
            antinode
            for freq in antennas.values()
            for a, b in combinations(freq, 2)
            for antinode in [2 * b - a, 2 * a - b]
            if 0 <= antinode.real < nx and 0 <= antinode.imag < ny
        }
        return len(antinodes)

    def part2(input):
        antennas, nx, ny = parse(input)
        antinodes = set()
        for freq in antennas.values():
            for a, b in combinations(freq, 2):
                (dx, dy) = simplify_frac(b.real-a.real, b.imag-a.imag)
                for step in [dx+dy*1j, -dx-dy*1j]:
                    antinode = a
                    while 0 <= antinode.real < nx and 0 <= antinode.imag < ny:
                        antinodes.add(antinode)
                        antinode += step
        return len(antinodes)

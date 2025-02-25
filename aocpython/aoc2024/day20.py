from collections import deque


DIRS = [-1, -1j, 1, 1j]


def parse(input):
    data = input.strip().splitlines()
    maze = set()
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell == 'E':
                end = x+y*1j
            if cell != '#':
                maze.add(x+y*1j)

    return maze, end


def floodfill(x0, maze):
    distances = []
    q = [(x0, 0)]
    while q:
        x, pico = q.pop()
        if x not in maze:
            continue
        maze.remove(x)
        distances.append(x)
        for dx in DIRS:
            new_x = x+dx
            if new_x in maze and new_x:
                q.append((new_x, pico+1))
    
    return distances


def calculate_cheat(input, d):
    maze, end = parse(input)
    cells = floodfill(end, maze)
    ans = 0
    for d1, cell in enumerate(cells[:-102]):
        d2 = d1+102
        while d2 < len(cells):
            cell2 = cells[d2]
            man_d = int(abs(cell.real-cell2.real) + abs(cell.imag-cell2.imag))
            if man_d > d:
                d2 += man_d - d-1
            elif d2-d1-man_d >= 100:
                ans += 1
            d2 += 1

    return ans


class Day20:
    def part1(input):
        return calculate_cheat(input, 2)

    def part2(input):
        return calculate_cheat(input, 20)

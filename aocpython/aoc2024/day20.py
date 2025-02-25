from collections import deque


DIRS = [-1, -1j, 1, 1j]


def parse(input):
    data = input.strip().splitlines()
    maze = set()
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell == '#':
                maze.add(x+y*1j)
            elif cell == 'E':
                end = x+y*1j

    return maze, end


def floodfill(x0, maze):
    distances = {}
    q = [(x0, 0)]
    while q:
        x, pico = q.pop()
        if x in distances:
            continue
        distances[x] = pico
        for dx in DIRS:
            new_x = x+dx
            if new_x not in maze and new_x not in distances:
                q.append((new_x, pico+1))
    return distances


def calculate_cheat(input, d):
    maze, end = parse(input)
    cells = floodfill(end, maze)
    ans = 0
    for cell in cells.keys():
        for dx in range(-d-2, d+2):
            for dy in range(-d+abs(dx), d+1-abs(dx)):
                if (abs(dx) > 1 or abs(dy) > 1) and cell+dx+dy*1j in cells and cells[cell]-cells[cell+dx+dy*1j]-abs(dx)-abs(dy) >= 100:
                    ans += 1

    return ans


class Day20:
    def part1(input):
        return calculate_cheat(input, 2)

    def part2(input):
        return calculate_cheat(input, 20)

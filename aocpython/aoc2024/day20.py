from collections import deque


DIRS = [-1, -1j, 1, 1j]


def parse(input):
    data = input.strip().splitlines()
    nx, ny = len(data[0]), len(data)
    maze = set()
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell == '#':
                maze.add(x+y*1j)
            if cell == 'S':
                start = x+y*1j
            elif cell == 'E':
                end = x+y*1j

    return maze, start, end, nx, ny


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

class Day20:
    def part1(input):
        maze, start, end, nx, ny = parse(input)
        e = floodfill(end, maze)
        s = floodfill(start, maze)
        baseline = s[end]
        ans = 0
        for wall in maze:
            for d in range(4):
                borders = [s.get(wall + DIRS[d],1e9),
                           e.get(wall + DIRS[(d+2) % 4],1e9)]

                new_distance = borders[0]+borders[1]+2
                if baseline-new_distance>=100:
                        ans+=1

        return ans

    def part2(input):
        return "Not implemented"

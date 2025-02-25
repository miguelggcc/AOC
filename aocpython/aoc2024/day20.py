

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


def generate_path(maze, x0):
    path = []
    q = [(x0, 0)]
    while q:
        x, pico = q.pop()
        maze.remove(x)
        path.append(x)
        for dx in DIRS:
            new_x = x+dx
            if new_x in maze:
                q.append((new_x, pico+1))

    return path


def calculate_cheat(maze,end, d):
    path = generate_path(maze,end)
    ans = 0
    for d1, cell1 in enumerate(path[:-102]):
        d2 = d1+102
        while d2 < len(path):
            cell2 = path[d2]
            man_d = int(abs(cell1.real-cell2.real) + abs(cell1.imag-cell2.imag))
            if man_d > d:
                d2 += man_d - d-1
            elif d2-d1-man_d >= 100:
                ans += 1
            d2 += 1

    return ans


class Day20:
    def part1(input):
        return calculate_cheat(*parse(input), 2)

    def part2(input):
        return calculate_cheat(*parse(input), 20)

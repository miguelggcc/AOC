def parse(input):
    data = input.splitlines()
    nx, ny = len(data), len(data[0])
    return {(x, y): data[y][x] for y in range(nx) for x in range(ny)}


class Day4:
    def part1(input):
        grid = parse(input)
        count = 0
        for (x, y), c in grid.items():
            if c == 'X':
                for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, -1), (-1, 1)]:
                    count += all(grid.get((x+dx*i, y+dy*i), "") ==
                                 "MAS"[i-1] for i in range(1, 4))
        return count

    def part2(input):
        grid = parse(input)
        count = 0
        for (x, y), c in grid.items():
            if c == 'A':
                corners = [grid.get((x+dx, y+dy), " ")
                           for (dx, dy) in [(1, 1), (1, -1), (-1, -1), (-1, 1)]]
                count += all(corner in "SM" for corner in corners) and all(
                    corners[i] != corners[i+2] for i in range(2))
        return count

def parse(input):
    grid = {}
    data = input.strip().splitlines()
    nx, ny = len(data), len(data[0])
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell == '@':
                x0 = x+y*1j
                continue
            if cell is not '.':
                grid[x+y*1j] = cell
    return grid, nx, ny, x0


dirs = {'^': -1j, '>': 1, '<': -1, 'v': 1j}


class Day15:
    def part1(input):
        g,m = input.split('\n\n')
        grid, nx, ny, x = parse(g)
        for d in m.replace('\n', ''):
            dir = dirs[d]
            p = x+dir
            while grid.get(p, None) == 'O':
                p += dir
            if p in grid:  # is a '#' wall
                continue
            x += dir
            if x != p:
                grid[p] = 'O'
                del grid[x]

        return sum(int(x.real+100*x.imag) for x, cell in grid.items() if cell == 'O')

    def part2(input):
        return "Not implemented"

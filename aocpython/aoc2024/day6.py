def parse(input):
    grid = set()
    data = input.strip().splitlines()
    nx, ny = len(data), len(data[0])
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell == '#':
                grid.add((x,y))
            if cell == '^':
                x0,y0 = x,y
    return grid,nx,ny,(x0,y0-1)


class Day6:
    def part1(input):
        grid,nx,ny, (x,y) = parse(input)
        visited = {(x,y+1)}
        dir = -1j
        while x>=0 and y>=0 and x<nx and y<ny:
            visited.add((x,y))
            if (x+dir.real,y+dir.imag) in grid:
                dir*=1j
            x,y=x+dir.real,y+dir.imag
        return len(visited)
    
    def part2(input):
        grid = parse(input)
        count = 0
        return count
